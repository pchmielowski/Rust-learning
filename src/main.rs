extern crate sdl2;
extern crate time;

use std::f32::consts::PI;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

struct Platform {
    x: Position,
    y: Position,
    width: Position,
    height: Position,
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
    fn get_delta(self) -> Position {
        match self {
            Direction::Forward => 1,
            Direction::Backward => -1,
        }
    }
}

type Position = i32;

#[derive(Clone, Copy, Default)]
struct JumpProgress {
    value: Option<i32>,
}

impl JumpProgress {
    const MAX: i32 = 1000;

    fn y(self) -> Position {
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
            .unwrap_or(0)
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

#[derive(Default)]
struct State {
    direction: Direction,
    is_moving: bool,
    x: Position,
    y: Position,
    jump_progress: JumpProgress,
    board: Board,
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
            jump_progress: self.jump_progress.new_jump(),
            ..self
        }
    }

    fn update(self, time_delta: Millis) -> Self {
        let x_delta = if self.is_moving {
            time_delta * self.direction.get_delta()
        } else {
            0
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

    let window = video_subsystem
        .window("SDL2", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

    let mut event_pump = sdl_context.event_pump()?;

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string())?;

    let frames_per_anim = 4;
    let sprite_tile_size = (32, 32);

    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_2.center_on(Point::new(440, 360));

    let mut time = time::now();

    let mut state :State= Default::default();

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

        source_rect_2.set_x(32 * ((state.x / 100) % frames_per_anim));
        dest_rect_2.set_x(1 * ((state.x / 10) % 768) - 128);
        let floor = 300;
        dest_rect_2.set_y(floor - state.y);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        canvas.fill_rect(Rect::new(0, source_rect_2.y(), 300, 10))?;
        canvas.copy_ex(
            &texture,
            Some(source_rect_2),
            Some(dest_rect_2),
            0.0,
            None,
            state.direction == Direction::Backward,
            false,
        )?;
        canvas.present();
    }

    Ok(())
}
