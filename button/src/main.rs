use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::time::Duration;

// --- Drawable Trait ---
trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

// --- UI Elements ---
struct InputBar {
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

// --- Implement Drawable for Button ---
impl Drawable for Button {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(self.color.0, self.color.1, self.color.2));
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        canvas.fill_rect(rect).unwrap();
    }
}

// --- Extra Drawing for InputBar with Text ---
impl InputBar {
    fn draw_with_text(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        // Draw background rectangle
        canvas.set_draw_color(Color::RGB(self.color.0, self.color.1, self.color.2));
        let rect = Rect::new(self.x, self.y, self.w, self.h);
        canvas.fill_rect(rect).unwrap();

        // Draw text if available
        if !self.text.is_empty() {
            let surface = font
                .render(&self.text)
                .blended(Color::RGB(255, 255, 255))
                .unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let text_rect = Rect::new(self.x + 5, self.y + 10, surface.width(), surface.height());
            canvas.copy(&texture, None, Some(text_rect)).unwrap();
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("SDL2 Input Bar Example", 720, 720)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let font_path = "src/font.ttf"; // adjust path if needed
    let font = ttf_context.load_font(font_path, 24).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let button_register = Button {
        x: 240,
        y: 400,
        w: 200,
        h: 100,
        color: (255, 0, 0),
    };

    let mut input_bar_email = InputBar {
        x: 240,
        y: 300,
        w: 200,
        h: 50,
        clicked: false,
        text: String::new(),
        color: (100, 0, 0),
    };

    let mut input_bar_password = InputBar {
        x: 240,
        y: 200,
        w: 200,
        h: 50,
        clicked: false,
        text: String::new(),
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
                    if mouse_btn == MouseButton::Left && point_in_button(x, y, &button_register) {
                        println!("Button clicked!");
                        send_email_to_server(input_bar_email.text.clone());
                        input_bar_email.text.clear();
                        input_bar_email.clicked = false;

                        send_password_to_server(input_bar_password.text.clone());
                        input_bar_password.text.clear();
                        input_bar_password.clicked = false;
                    }
                }

                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    if mouse_btn == MouseButton::Left {
                        input_bar_email.clicked = point_in_input_bar(x, y, &input_bar_email);
                        input_bar_password.clicked = point_in_input_bar(x, y, &input_bar_password);
                        println!(
                            "Email clicked: {}, Password clicked: {}",
                            input_bar_email.clicked, input_bar_password.clicked
                        );
                    }
                }

                Event::TextInput { text, .. } => {
                    if input_bar_email.clicked {
                        input_bar_email.text.push_str(&text);
                    } else if input_bar_password.clicked {
                        input_bar_password.text.push_str(&text);
                    }
                }

                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        button_register.draw(&mut canvas);
        input_bar_email.draw_with_text(&mut canvas, &font, &texture_creator);
        input_bar_password.draw_with_text(&mut canvas, &font, &texture_creator);

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}

// --- Helpers ---
fn point_in_button(x: i32, y: i32, button: &Button) -> bool {
    x >= button.x
        && x <= button.x + button.w as i32
        && y >= button.y
        && y <= button.y + button.h as i32
}

fn point_in_input_bar(x: i32, y: i32, bar: &InputBar) -> bool {
    x >= bar.x && x <= bar.x + bar.w as i32 && y >= bar.y && y <= bar.y + bar.h as i32
}

fn send_email_to_server(val: String) {
    println!("Sending email: {}", val);
}
fn send_password_to_server(val: String) {
    println!("Sending Password: {}", val);
}
