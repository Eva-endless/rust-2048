extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate piston_window;
extern crate opengl_graphics;
extern crate glutin_window;

use piston_window::*;
use glutin_window::GlutinWindow;

mod app;
mod board;
mod number_renderer;
mod settings;
mod tile;
mod ai;
mod achievements;

fn main() {
    use opengl_graphics::GlGraphics;    
    let settings = settings::Settings::load();

    let (width, height) = (settings.window_size[0], 
                           settings.window_size[1]);

    let mut window: PistonWindow<GlutinWindow> =
        WindowSettings::new("Rust-2048", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut app = app::App::new(&settings);

    app.load();

    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let mut mouse_pos = [0.0, 0.0];

    while let Some(e) = window.next() {
        if let Some(ref args) = e.render_args() {
            app.render(args, &mut gl);
        }

        if let Some(ref args) = e.update_args() {
            app.update(args);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            mouse_pos = pos;
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
            app.mouse_press(args, mouse_pos);
        }
    }
}