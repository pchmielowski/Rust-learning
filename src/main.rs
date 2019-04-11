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

    let mut colors: [[u8; HEIGHT]; WIDTH] = [[0; HEIGHT]; WIDTH];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        for x in 0..WIDTH - 1 {
            colors[x][HEIGHT - 2 /* I don't know why 2 works and 1 not */] = 255;
        }
//        for x in 1..WIDTH - 1 {
//            for y in (1..HEIGHT - 1).rev() {
//                colors[x][y] = ((colors[x - 1][y] as u16 + colors[x][y] as u16) / 2) as u8;
//            }
//        }


        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for x in 0..WIDTH - 1 {
            for y in 0..HEIGHT - 1 {
                let color = colors[x][y];
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
    let mut ys: [[i32; 4]; 4] = [[0; 4]; 4];
    println!("{:?}", ys);
    ys[2][2] = 12;
    println!("{:?}", ys);
    println!("{:?}", ys[2][2]);
    assert_eq!(1 + 2, 3);
}
