extern crate sdl2;
extern crate time;

use std::f32::consts::PI;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::fmt::Debug;

struct Platform {
    x: Meters,
    y: Meters,
    width: Meters,
    height: Meters,
}

#[derive(Default)]
struct Board {
    platforms: Vec<Platform>,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Forward,
    Backward,
}

impl Default for Direction {
    fn default() -> Self { Direction::Forward }
}

impl Direction {
    fn speed(self) -> Meters {
        let speed: MetersPerSecond = 6.0;
        match self {
            Direction::Forward => speed,
            Direction::Backward => -speed,
        }
    }
}

type Meters = f32;
type MetersPerSecond = f32;

#[derive(Clone, Copy, Default)]
struct JumpProgress {
    value: Option<i32>,
}

impl JumpProgress {
    const MAX: i32 = 1000;

    fn y(self) -> Meters {
        let height = 200.0;
        self.value
            .map(|it| {
                if it > JumpProgress::MAX / 2 {
                    JumpProgress::MAX - it
                } else {
                    it
                }
            })
            .map(|it| it as f32 * 2.0)
            .map(|it| it / JumpProgress::MAX as f32 * PI / 2.0)
            .map(|it| (it.sin() * height) as i32)
            .unwrap_or(0) as f32
    }

    fn new_jump(self) -> Self {
        Self {
            value: self.value.or(Some(0)),
        }
    }

    fn update(self, time_delta: Millis) -> Self {
        Self {
            value: self.value.and_then(|it| {
                if it >= JumpProgress::MAX {
                    None
                } else {
                    Some(it + time_delta)
                }
            }),
        }
    }
}

type Millis = i32;
type Seconds = f32;

struct State {
    direction: Direction,
    is_moving: bool,
    x: Meters,
    y: Meters,
    dy: MetersPerSecond,
    jump_progress: JumpProgress,
    board: Board,
}

impl Default for State {
    fn default() -> Self {
        State {
            x: 0.0,
            y: 0.0,
            dy: 0.0,
            direction: Direction::default(),
            is_moving: false,
            jump_progress: JumpProgress::default(),
            board: Board::default(),
        }
    }
}

impl State {
    fn go_forward(self) -> Self {
        State {
            direction: Direction::Forward,
            is_moving: true,
            ..self
        }
    }

    fn go_backward(self) -> Self {
        State {
            direction: Direction::Backward,
            is_moving: true,
            ..self
        }
    }

    fn stop(self) -> Self {
        State {
            is_moving: false,
            ..self
        }
    }

    fn jump(self) -> Self {
        State {
            dy: 5.0, // TODO: find a good value.
            jump_progress: self.jump_progress.new_jump(),
            ..self
        }
    }

    fn update(self, time_delta: Millis) -> Self {
        let x_delta = if self.is_moving {
            time_delta as Seconds / 1000.0 * self.direction.speed()
        } else {
            0.0
        };
        State {
            x: self.x + x_delta,
            y: self.jump_progress.y(),
            jump_progress: self.jump_progress.update(time_delta),
            ..self
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let width = 640;
    let height = 480;

    let window = video_subsystem
        .window("SDL2", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut event_pump = sdl_context.event_pump()?;

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string())?;

    let frames_per_anim = 4;
    let sprite_tile_size = (32, 32);

    let mut character_src = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut character_dst = Rect::new(0, 64, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    character_dst.center_on(Point::new(440, 360));

    let mut time = time::now();

    let mut state: State = Default::default();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'main;
                }
                Event::KeyDown { keycode, .. } => {
                    state = match keycode {
                        Some(Keycode::Left) => state.go_backward(),
                        Some(Keycode::Right) => state.go_forward(),
                        Some(Keycode::Space) => state.jump(),
                        _ => state,
                    };
                }
                Event::KeyUp { keycode, .. } => {
                    state = match keycode {
                        Some(Keycode::Left) => {
                            if state.direction == Direction::Backward {
                                state.stop()
                            } else {
                                state
                            }
                        }
                        Some(Keycode::Right) => {
                            if state.direction == Direction::Forward {
                                state.stop()
                            } else {
                                state
                            }
                        }
                        _ => state,
                    };
                }
                _ => {}
            }
        }

        let now = time::now();
        let delta = now - time;
        time = now;

        state = state.update(delta.num_milliseconds() as Millis);

        // Clear surface.
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw platforms. TODO: Read from state.
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        canvas.fill_rect(Rect::new(0, 0, 300, 10))?;

        // Draw character.
        let frame_offset = 32 * ((state.x as i32) % frames_per_anim);
        character_src.set_x(frame_offset);
        character_dst.set_x((state.x * 20.0) as i32);
        character_dst.set_y((state.y * 20.0) as i32);
        canvas.copy_ex(
            &texture,
            Some(character_src),
            Some(character_dst),
            0.0,
            None,
            state.direction == Direction::Backward,
            false,
        )?;

        canvas.present();
    }

    Ok(())
}
