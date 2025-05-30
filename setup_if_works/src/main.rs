extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

struct Pixel {
    x: i32,
    y: i32,
    color: (u8, u8, u8),
}

fn window_init(
    win_name: &str,
    win_height: u32,
    win_width: u32,
) -> (sdl2::Sdl, sdl2::VideoSubsystem, Window, u32, u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(win_name, win_width, win_height)
        .position_centered()
        .build()
        .unwrap();
    (sdl_context, video_subsystem, window, win_height, win_width)
}

fn draw_pixel(canvas: &mut Canvas<Window>, pixel: &Pixel) {
    canvas.set_draw_color(Color::RGB(pixel.color.0, pixel.color.1, pixel.color.2));
    canvas.draw_point(Point::new(pixel.x, pixel.y)).unwrap();
}

fn draw_window(canvas: &mut Canvas<Window>, win_width: u32, win_height: u32) {
    // Clear screen with white color
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    // Create pixels to draw
    let mut pixels_ltor = vec![];
    for (x, y) in (0..win_height).zip(0..win_width) {
        pixels_ltor.push(Pixel {
            x: x as i32,
            y: y as i32,
            color: (255, 0, 0), // red pixel
        });
    }
    let mut pixels_rtol = vec![];
    for (x, y) in (0..win_height).zip((0..win_width).rev()) {
        pixels_rtol.push(Pixel {
            x: x as i32,
            y: y as i32,
            color: (255, 0, 0),
        });
    }

    // Draw all pixels
    for pixel in &pixels_ltor {
        draw_pixel(canvas, pixel);
    }
    for pixel in &pixels_rtol {
        draw_pixel(canvas, pixel);
    }

    canvas.present();
    std::thread::sleep(Duration::from_millis(16));
}

fn main_loop(
    mut canvas: Canvas<Window>,
    mut event_pump: sdl2::EventPump,
    win_height: u32,
    win_width: u32,
) {
    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'running;
            }
        }
        draw_window(&mut canvas, win_width, win_height);
    }
}

fn main() {
    let (sdl_context, _video_subsystem, window, win_height, win_width) =
        window_init("SDL2 SETUP IN RUST", 720, 720);
    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    main_loop(canvas, event_pump, win_height, win_width);
}
