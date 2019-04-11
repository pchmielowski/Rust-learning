extern crate sdl2;
extern crate time;

use std::path::Path;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::mouse::SystemCursor::No;

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
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

type Percent = i32;

type Millis = i32;

struct State {
    direction: Option<Direction>,
    x: Position,
    y: Position,
    jump_progress: Option<Percent>,
}

impl State {
    fn go_forward(self) -> Self {
        State { direction: Some(Direction::Forward), ..self }
    }

    fn go_backward(self) -> Self {
        State { direction: Some(Direction::Backward), ..self }
    }

    fn stop(self) -> Self {
        State { direction: None, ..self }
    }

    fn jump(self) -> Self {
        State { jump_progress: Some(0), ..self }
    }

    fn update(self, time_delta: Millis) -> Self {
        let x_delta = time_delta * self.direction.map_or(0, |d| d.get_delta());
        let y = self.jump_progress
            .map(|it| if it > 50 { 100 - it } else { it })
            .unwrap_or(0);
        let jum_height = 4;
        State {
            x: self.x + x_delta,
            y: y * jum_height,
            jump_progress: self.jump_progress
                .and_then(|it| if it >= 100 { None } else {
                    Some(it + time_delta)
                }),
            ..self
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered().build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

    let mut timer = sdl_context.timer()?;

    let mut event_pump = sdl_context.event_pump()?;

// animation sheet and extras are available from
// https://opengameart.org/content/a-platformer-in-the-forest
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp"))?;
    let texture = texture_creator.create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string())?;

    let frames_per_anim = 4;
    let sprite_tile_size = (32, 32);

    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_2.center_on(Point::new(440, 360));

    let mut time = time::now();

    let mut state = State { direction: None, x: 0, y: 0, jump_progress: None };

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                }
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Left) => {
                            state = state.go_backward();
                        }
                        Some(Keycode::Right) => {
                            state = state.go_forward();
                        }
                        Some(Keycode::Space) => {
                            state = state.jump();
                        }
                        _ => {}
                    };
                }
                Event::KeyUp { .. } => {
                    state = state.stop();
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
        dest_rect_2.set_y(300 - state.y);
        canvas.clear();
// copy the frame to the canvas
        canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false)?;
        canvas.present();
    }

    Ok(())
}
