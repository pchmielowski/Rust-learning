extern crate sdl2;
extern crate rand;

use rand::prelude::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::{Rect, Point};

fn random(range: u32) -> u32 {
    return (rand::random::<f32>() * range as f32 / 2.0 + range as f32 / 2.0) as u32;
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let width = 800;
    let height = 600;

    let window = video_subsystem.window("Fire", width, height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(random(255) as u8, 0, 0));
        for x in 0..width {
            for y in 0..random(height / 2) {
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }
        canvas.present();
    }

    Ok(())
}
