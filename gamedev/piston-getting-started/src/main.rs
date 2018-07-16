extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate spinning_square;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

fn main() {
    // Original Defenders had a resolution of 320x256
    let mut app = App::new(GraphicsConfig::new("Defender", 960, 768));

    // Poll for events from the window.
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        // Handle rendering
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        // Handle any updates
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
