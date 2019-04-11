extern crate sdl2;
extern crate time;

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;

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

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_0.center_on(Point::new(-64, 120));

    // King - walk animation
    let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_1.center_on(Point::new(0, 240));

    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_2.center_on(Point::new(440, 360));

    let mut walking = true;
    let mut time = time::now();
    let mut ticks: i32 = 0;

    let mut speed = 1.0;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    walking = !walking;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    speed += 0.5;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    speed -= 0.5;
                }
                _ => {}
            }
        }

        let now = time::now();
        let delta = now - time;
        time = now;

        if walking {
            ticks += (delta.num_milliseconds() as f32 * speed) as i32;

            // set the current frame for time
            source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
            dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);

            source_rect_1.set_x(32 * ((ticks / 100) % frames_per_anim));
            dest_rect_1.set_x((1 * ((ticks / 12) % 768) - 672) * -1);

            source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
            dest_rect_2.set_x(1 * ((ticks / 10) % 768) - 128);

            canvas.clear();
            // copy the frame to the canvas
            canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false)?;
            canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, true, false)?;
            canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false)?;
            canvas.present();
        }
    }

    Ok(())
}
