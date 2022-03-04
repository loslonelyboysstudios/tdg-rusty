use graphics::clear;
use opengl_graphics::GlGraphics;
use piston::input::{Button, ButtonArgs, ButtonState, RenderArgs, UpdateArgs};

use crate::{constants, level, player};

fn _add(u: u32, i: i32, b: u32) -> u32 {
    if i.is_negative() {
        let i = i.wrapping_abs() as u32;
        if i > u {
            return u;
        }
        u - i
    } else {
        let i = i as u32;
        if u + i > b {
            return u;
        }
        u + i
    }
}

fn add_pos(a1: [u32; 2], a2: [i32; 2], b: [usize; 2]) -> [u32; 2] {
    [
        _add(a1[0], a2[0], b[0] as u32),
        _add(a1[1], a2[1], b[1] as u32),
    ]
}

pub struct Tdg {
    gl: GlGraphics, // OpenGL drawing backend.
    p: player::Player,
    lvl: level::Level,
    next_pos: [u32; 2],
    next_state: player::PlayerState,
    controls: constants::Controls,
}
impl Tdg {
    pub fn new(gl: GlGraphics) -> Tdg {
        Tdg {
            gl,
            p: player::Player::new(),
            lvl: level::Level::new("assets/level.lvl"),
            next_pos: [0, 0],
            next_state: player::PlayerState::FACE_DOWN,
            controls: constants::controls(),
        }
    }

    pub fn update_inputs(&mut self, args: &ButtonArgs) {
        self.next_pos = self.p.pos;
        if let (Button::Keyboard(k), ButtonState::Press) = (args.button, args.state) {
            if self.controls.contains_key(&k) {
                match self.controls[&k] {
                    player::PlayerAction::MOVE_UP => {
                        println!("MOVE_UP");
                        self.next_pos = add_pos(self.p.pos, constants::Direction::UP, self.lvl.size)
                    }
                    player::PlayerAction::MOVE_DOWN => {
                        println!("MOVE_DOWN");
                        self.next_pos =
                            add_pos(self.p.pos, constants::Direction::DOWN, self.lvl.size)
                    }
                    player::PlayerAction::MOVE_LEFT => {
                        println!("MOVE_LEFT");
                        self.next_pos =
                            add_pos(self.p.pos, constants::Direction::LEFT, self.lvl.size)
                    }
                    player::PlayerAction::MOVE_RIGHT => {
                        println!("MOVE_RIGHT");
                        self.next_pos =
                            add_pos(self.p.pos, constants::Direction::RIGHT, self.lvl.size)
                    }
                    player::PlayerAction::INTERACT => {}
                    _ => {}
                }
            }
            println!("{:?}", self.next_pos);
        }
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        // args.dt
        self.p.update(self.next_pos, self.next_state);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            self.lvl.draw(c.transform, gl);
            self.p.draw(c.transform, gl);
        });
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
