extern crate bitflags;
extern crate phf;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod constants;
mod level;
mod player;
mod world;

fn main() {
    println!("enter main");

    //init
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Hello Piston!", constants::WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut tdg = world::Tdg::new(GlGraphics::new(opengl));
    //game loop
    let mut es = EventSettings::new();
    es.ups = constants::FRAMES_PER_SECOND;
    let mut events = Events::new(es);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            if let Button::Keyboard(k) = args.button {
                tdg.update_inputs(&k);
            }
        }
        if let Some(args) = e.render_args() {
            tdg.render(&args);
        }
        if let Some(args) = e.update_args() {
            tdg.update(&args);
            // tdg.print_player_state();
        }
    }
}
