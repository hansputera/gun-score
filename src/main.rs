extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::input::{RenderEvent, UpdateEvent, ReleaseEvent, PressEvent};

mod util;
mod geom;
mod schemas;
mod textures;
mod colors;
mod app;

use app::{GunScoreApp};
use piston::event_loop::{EventSettings, Events};

fn main() {
    let mut app = GunScoreApp::new();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window) {
        if let Some(i) = e.press_args() {
            app.input(&i, true);
        }

        if let Some(i) = e.release_args() {
            app.input(&i, false);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

