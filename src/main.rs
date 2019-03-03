extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;
extern crate rayon;

use std::time::Instant;
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use opengl_graphics::{OpenGL, GlGraphics};

use crate::controller::RainDropController;

mod drop;
mod controller;


fn main() {
    let opengl = OpenGL::V3_2;
    let dims = [475; 2];
    let settings = WindowSettings::new("rain", dims)
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    let mut drop_controller = RainDropController::new(dims);
    let mut prev_loop_time = Instant::now();

    while let Some(e) = events.next(&mut window) {
        let now = Instant::now();
        let elapsed_secs = now.duration_since(prev_loop_time).as_millis() as f64 / 1000.0;
        prev_loop_time = now;
        drop_controller.event(elapsed_secs, &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                clear([1.0; 4], g);
                drop_controller.draw_background(&c, g);
                drop_controller.drops
                    .iter_mut()
                    .for_each(|x| x.draw(&c, g));
            });
        }
    }
}
