extern crate rand;
extern crate sdl2;
extern crate time;


use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use time::Tm;

fn random_color() -> u8 {
    rand::random::<u8>()
}

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

//fn create_colors() -> Colors { [[0; HEIGHT]; WIDTH] }
fn create_colors() -> Colors { [0; WIDTH * HEIGHT] }

//type Colors = [[u8; HEIGHT]; WIDTH];
type Colors = [u8; WIDTH * HEIGHT];

//fn get(colors: &Colors, x: usize, y: usize) -> u8 { colors[x][y] }
fn get(colors: &Colors, x: usize, y: usize) -> u8 { colors[x + WIDTH * y] }


//fn set(colors: &mut Colors, x: usize, y: usize, value: u8) { colors[x][y] = value; }
fn set(colors: &mut Colors, x: usize, y: usize, value: u8) { colors[x + WIDTH * y] = value; }

fn recalculate_image(mut colors: &mut Colors) {
    for x in 0..WIDTH - 1 {
        set(colors, x, HEIGHT - 1, random_color());
    }
    for x in 1..WIDTH - 1 {
        for y in (0..HEIGHT - 1).rev() {
            let sum = get(colors, x, y) as u16
                + get(colors, x, y + 1) as u16
                + get(colors, x - 1, y + 1) as u16
                + get(colors, x + 1, y + 1) as u16;
            set(&mut colors, x, y, (sum / 4) as u8);
        }
    }
}

fn redraw_image(colors: &mut Colors, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    for x in 0..WIDTH - 1 {
        for y in 0..HEIGHT - 1 {
            let color = get(colors, x, y);
            canvas.set_draw_color(Color::RGB(color, color, color));
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        }
    }
    canvas.present();
}

fn update_time(time_info: (Tm, i64, i64)) -> (Tm, i64, i64) {
    let (prev, sum, num_iterations) = time_info;
    let now = time::now();
    let delta_time = now - prev;
    (now, sum + delta_time.num_milliseconds(), num_iterations + 1)
}

fn should_quit(pump: &mut EventPump) -> bool {
    for event in pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return true;
            }
            _ => {}
        }
    }
    false
}

fn setup_sdl() -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Fire", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string()).unwrap();
    (window.into_canvas().build().map_err(|e| e.to_string()).unwrap(),
     sdl_context.event_pump().unwrap())
}

fn main() -> Result<(), String> {
    let (mut canvas, mut event_pump) = setup_sdl();
    let mut colors = create_colors();
    let mut time_info = (time::now(), 0, 0);

    loop {
        if should_quit(&mut event_pump) { break; }

        recalculate_image(&mut colors);
        redraw_image(&mut colors, &mut canvas);

        time_info = update_time(time_info);
    }

    let (_, sum_time, iterations) = time_info;
    println!("Avg time for frame: {}", sum_time / iterations);
    println!("Avg FPS:            {}", 1_000 / (sum_time / iterations));
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
