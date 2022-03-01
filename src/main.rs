extern crate bitflags;
extern crate phf;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::clear;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, ButtonArgs, ButtonEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod player;
mod constants;
mod level;

pub struct TDG {
    gl: GlGraphics, // OpenGL drawing backend.
    p: player::Player,
    lvl: level::Level
}
impl TDG {
    fn new(gl: GlGraphics) -> TDG {
        TDG {
            gl,
            p: player::Player::new(),
            lvl: level::Level::new("src/assets/level.lvl")
        }
    }

    fn render(&mut self, args: &RenderArgs) {

        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            self.lvl.draw(c.transform, gl);
        });
    }
    fn update_inputs(&mut self, args: &ButtonArgs) {
        self.p.update_inputs(args);
    }
    fn update(&mut self, args: &UpdateArgs) {
        self.p.update(args.dt);
    }
    // #[allow(dead_code)]
    // fn print_player_state(&mut self) {
    //     for p in self.players.iter_mut() {
    //         match p {
    //             Some(p) => {
    //                 print!("{:}\t", p.get_debug_state());
    //             },
    //             None => ()
    //         }
    //     }
    //     print!("\r");
    //     stdout().flush().ok();
    // }
}

fn main() {
    println!("enter main");

    //init
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Hello Piston!", constants::WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut tdg = TDG::new(GlGraphics::new(opengl));
    //game loop
    let mut es = EventSettings::new();
    es.ups = constants::FRAMES_PER_SECOND;
    let mut events = Events::new(es);
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.button_args() {
            tdg.update_inputs(&args);
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