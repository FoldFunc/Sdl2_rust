use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::input_absinfo;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
struct Input_bar {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    clicked: bool,
    text: String,
    color: (u8, u8, u8),
}
struct Button {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: (u8, u8, u8),
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2 Button", 720, 720)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let button_register = Button {
        x: 240,
        y: 400,
        w: 200,
        h: 100,
        color: (255, 0, 0),
    };
    let mut input_bar_email = Input_bar {
        x: 240,
        y: 300,
        w: 200,
        h: 50,
        clicked: false,
        text: "".to_string(),
        color: (100, 0, 0),
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == MouseButton::Left {
                        if point_in_button(x, y, &button_register) {
                            println!("Button clicked!");
                            let _ = send_email_to_server(input_bar_email.text);
                            input_bar_email.text = "".to_string();
                            input_bar_email.clicked = false;
                        }
                    }
                    if mouse_btn == MouseButton::Right {
                        if point_in_button(x, y, &button_register) {
                            println!("Right mouse button clicked");
                        }
                    }
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == MouseButton::Left {
                        if point_in_input_bar_email(x, y, &input_bar_email) {
                            input_bar_email.clicked = !input_bar_email.clicked;
                            println!("Change to: {}", input_bar_email.clicked);
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if input_bar_email.clicked == true {
                        input_bar_email
                            .text
                            .push(key.to_string().parse::<char>().unwrap_or_default());
                        println!("Current what is in the buffer: {}", input_bar_email.text);
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();
        let _ = draw_shapes(&mut canvas, &button_register, &input_bar_email);
    }
}
fn draw_shapes(canvas: &mut Canvas<Window>, button_register: &Button, input_field: &Input_bar) {
    canvas.set_draw_color(Color::RGB(
        button_register.color.0,
        button_register.color.1,
        button_register.color.2,
    ));
    let rect = Rect::new(
        button_register.x,
        button_register.y,
        button_register.w,
        button_register.h,
    );
    canvas.fill_rect(rect).unwrap();

    canvas.set_draw_color(Color::RGB(
        input_field.color.0,
        input_field.color.1,
        input_field.color.2,
    ));
    let input_f = Rect::new(input_field.x, input_field.y, input_field.w, input_field.h);
    canvas.fill_rect(input_f).unwrap();

    canvas.present();

    std::thread::sleep(Duration::from_millis(16));
}
fn point_in_button(x: i32, y: i32, button: &Button) -> bool {
    x >= button.x
        && x <= button.x + button.w as i32
        && y >= button.y
        && y <= button.y + button.h as i32
}
fn point_in_input_bar_email(x: i32, y: i32, button: &Input_bar) -> bool {
    x >= button.x
        && x <= button.x + button.w as i32
        && y >= button.y
        && y <= button.y + button.h as i32
}

fn send_email_to_server(val: String) {
    //foo
}
