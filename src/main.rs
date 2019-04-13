extern crate sdl2;
extern crate time;

use std::cmp::min;
use std::f32::consts::PI;
use std::fmt::Debug;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

struct Platform {
    x: Meters,
    y: Meters,
    width: Meters,
}

struct Board {
    platforms: Vec<Platform>,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            platforms: vec![Platform {
                x: 0.0,
                y: 0.0,
                width: 10.0,
            }]
        }
    }
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

type Millis = i32;
type Seconds = f32;

struct State {
    direction: Direction,
    is_moving: bool,
    is_jumping: bool,
    x: Meters,
    y: Meters,
    speed_y: MetersPerSecond,
    board: Board,
}

impl Default for State {
    fn default() -> Self {
        State {
            x: 0.0,
            y: 0.0,
            speed_y: 0.0,
            direction: Direction::default(),
            is_moving: false,
            is_jumping: false,
            board: Board::default(),
        }
    }
}

fn millis_to_seconds(millis: Millis) -> Seconds {
    millis as Seconds / 1000.0
}

impl State {
    fn go_in_direction(self, direction: Direction) -> Self {
        if self.is_jumping {
            self
        } else {
            State {
                direction,
                is_moving: true,
                ..self
            }
        }
    }
    fn go_forward(self) -> Self {
        self.go_in_direction(Direction::Forward)
    }

    fn go_backward(self) -> Self {
        self.go_in_direction(Direction::Backward)
    }

    fn stop(self) -> Self {
        if self.is_jumping {
            self
        } else {
            State {
                is_moving: false,
                ..self
            }
        }
    }

    fn jump(self) -> Self {
        State {
            speed_y: 7.0,
            is_jumping: true,
            ..self
        }
    }

    fn update(self, time_delta: Millis) -> Self {
        let seconds = millis_to_seconds(time_delta);
        let x_delta = if self.is_moving {
            seconds * self.direction.speed()
        } else {
            0.0
        };
        let g = 9.81; // m/s^2
        let y = (self.y + self.speed_y * seconds).max(0.0);
        State {
            x: self.x + x_delta,
            y,
            is_jumping: y != 0.0,
            speed_y: self.speed_y - g * seconds,
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
    let sprite_tile_size = 32;

    let mut character_src = Rect::new(0, 64, sprite_tile_size, sprite_tile_size);
    let mut character_dst = Rect::new(0, 64, sprite_tile_size * 4, sprite_tile_size * 4);

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

        // Draw platforms.
        let platform_height = 0.5;
        let base_y = (height - sprite_tile_size * 4) as i32-platform_height.to_pixels();
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        for platform in state.board.platforms.iter() {
            canvas.fill_rect(Rect::new(platform.x.to_pixels(),
                                       (height as i32) - platform.y.to_pixels() - platform_height.to_pixels(),
                                       platform.width.to_pixels() as u32,
                                       platform_height.to_pixels() as u32))?;
        }

        // Draw character.
        let frame_offset = 32 * ((state.x as i32) % frames_per_anim);
        character_src.set_x(frame_offset);
        character_dst.set_x(state.x.to_pixels());
        character_dst.set_y(base_y - state.y.to_pixels());
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

trait ToPixels {
    fn to_pixels(self) -> i32;
}

impl ToPixels for Meters {
    fn to_pixels(self) -> i32 {
        (self * 60.0) as i32
    }
}
