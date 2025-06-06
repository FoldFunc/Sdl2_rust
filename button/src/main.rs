extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
fn main() {
    let (sdl_content, _video_subsysstem, window, win_height, win_width) =
        window_init("SDL2 Button".to_string(), 720, 720);
    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_content.event_pump().unwrap();
    main_loop(canvas, event_pump, win_height, win_width)
}

fn window_init(
    window_name: String,
    win_height: u32,
    win_width: u32,
) -> (sdl2::Sdl, sdl2::VideoSubsystem, Window, u32, u32) {
    let sdl_content = sdl2::init().unwrap();
    let video_subsysstem = sdl_content.video().unwrap();
    let window = video_subsysstem
        .window(&window_name, win_width, win_height)
        .position_centered()
        .build()
        .unwrap();
    (sdl_content, video_subsysstem, window, win_height, win_width)
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
        let _ = draw_window(&mut canvas, win_height, win_width);
    }
}
fn draw_window(canvas: &mut Canvas<Window>, win_height: u32, win_width: u32) {
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    canvas.clear();
    canvas.present();
    std::thread::sleep(Duration::from_millis(16));
}
