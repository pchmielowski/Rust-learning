extern crate rand;
extern crate sdl2;


use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

fn random_color() -> u8 {
    rand::random::<u8>()
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;

    let window = video_subsystem.window("Fire", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut colors: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        for x in 0..WIDTH {
            colors[x + WIDTH * (HEIGHT - 1)] = random_color();
        }
        for x in 1..WIDTH {
            for y in (1..HEIGHT).rev() {
                colors[x + WIDTH * y] = ((colors[x - 1 + WIDTH * y] as u16 + colors[x + WIDTH * y] as u16) / 2) as u8;
            }
        }


        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let color = colors[x + WIDTH * y];
                canvas.set_draw_color(Color::RGB(color, color, color));
                canvas.draw_point(Point::new(x as i32, y as i32))?;
            }
        }
        canvas.present();
    }

    Ok(())
}

#[test]
fn test_add() {
    let mut ys: [i32; 10] = [0; 10];
    println!("{:?}", ys);
    ys[0] = 12;
    println!("{:?}", ys);
    assert_eq!(1 + 2, 3);
}
