//#![deny(warnings)]

extern crate sdl2;
extern crate time;

use std::path::Path;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use hello_rust::lib::*;

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
    let sprite_src_tile_size = 32; // Size in the graphic.
    let scale = 4;
    let sprite_dst_tile_size = sprite_src_tile_size * scale;

    let mut character_src = Rect::new(0, 64, sprite_src_tile_size, sprite_src_tile_size);
    let mut character_dst = Rect::new(0, 64, sprite_dst_tile_size, sprite_dst_tile_size);

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


        let apply_scroll_y = |y: Meters| {
            (y - state.y).to_pixels() + height as i32 / 2
        };
        let apply_scroll_x = |x: Meters| {
            (x - state.x).to_pixels() + width as i32 / 2
        };

        // Draw platforms.
        let platform_height = 0.2;
        let base_y = (height - sprite_dst_tile_size) as i32 - platform_height.to_pixels();
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        for platform in state.board.platforms.iter() {
            canvas.fill_rect(Rect::new(
                apply_scroll_x(platform.x_from),
                (height as i32) - apply_scroll_y(platform.y) - platform_height.to_pixels(),
                platform.width().to_pixels() as u32,
                platform_height.to_pixels() as u32,
            ))?;
        }

        // Draw coins.
        for coin in state.board.coins.iter() {
            canvas.filled_circle(
                apply_scroll_x(coin.x) as i16,
                (base_y - apply_scroll_y(coin.y)) as i16,
                10,
                Color::RGB(255, 128, 0),
            )?;
        }

        // Draw character.
        let frame_offset = 32 * ((state.x as i32) % frames_per_anim);
        character_src.set_x(frame_offset);
        character_dst.set_x((width / 2) as i32 - sprite_dst_tile_size as i32 / 2);
        character_dst.set_y(base_y - apply_scroll_y(state.y));
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
