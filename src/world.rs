use graphics::clear;
use graphics::math::*;
use opengl_graphics::GlGraphics;
use piston::input::{Button, ButtonArgs, ButtonState, RenderArgs, UpdateArgs};

use crate::{
    constants, level,
    player::{Player, PlayerAction, PlayerState},
};

fn add_pos(a1: [u32; 2], a2: [i32; 2], b: [usize; 2]) -> [u32; 2] {
    let _add = |u: u32, i: i32, b: u32| -> u32 {
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
    };

    [
        _add(a1[0], a2[0], (b[0] - 1) as u32),
        _add(a1[1], a2[1], (b[1] - 1) as u32),
    ]
}

fn camera_transform(target: [u32; 2]) -> Matrix2d {
    translate([
        constants::WINDOW_SIZE.0 / 2. - (constants::TILE_SIZE.0 * (target[0] as f64 + 0.5)),
        constants::WINDOW_SIZE.1 / 2. - (constants::TILE_SIZE.1 * (target[1] as f64 + 0.5)),
    ])
}

pub struct Tdg {
    gl: GlGraphics, // OpenGL drawing backend.
    p: Player,
    lvl: level::Level,
    next_pos: [u32; 2],
    next_state: PlayerState,
    controls: constants::Controls,
}
impl Tdg {
    pub fn new(gl: GlGraphics) -> Tdg {
        Tdg {
            gl,
            p: Player::new(),
            lvl: level::Level::new("assets/level2.lvl"),
            next_pos: [0, 0],
            next_state: PlayerState::FACE_DOWN,
            controls: constants::controls(),
        }
    }

    pub fn update_inputs(&mut self, args: &ButtonArgs) {
        if let (Button::Keyboard(k), ButtonState::Press) = (args.button, args.state) {
            if self.controls.contains_key(&k) {
                match self.controls[&k] {
                    PlayerAction::MOVE_UP => {
                        println!("MOVE_UP");
                        if !self.p.state.contains(PlayerState::FACE_UP) {
                            self.next_state = PlayerState::FACE_UP;
                        } else {
                            self.next_pos =
                                add_pos(self.p.pos, constants::Direction::UP, self.lvl.size);
                        }
                    }
                    PlayerAction::MOVE_DOWN => {
                        println!("MOVE_DOWN");
                        if !self.p.state.contains(PlayerState::FACE_DOWN) {
                            self.next_state = PlayerState::FACE_DOWN;
                        } else {
                            self.next_pos =
                                add_pos(self.p.pos, constants::Direction::DOWN, self.lvl.size);
                        }
                    }
                    PlayerAction::MOVE_LEFT => {
                        println!("MOVE_LEFT");
                        if !self.p.state.contains(PlayerState::FACE_LEFT) {
                            self.next_state = PlayerState::FACE_LEFT;
                        } else {
                            self.next_pos =
                                add_pos(self.p.pos, constants::Direction::LEFT, self.lvl.size);
                        }
                    }
                    PlayerAction::MOVE_RIGHT => {
                        println!("MOVE_RIGHT");
                        if !self.p.state.contains(PlayerState::FACE_RIGHT) {
                            self.next_state = PlayerState::FACE_RIGHT;
                        } else {
                            self.next_pos =
                                add_pos(self.p.pos, constants::Direction::RIGHT, self.lvl.size);
                        }
                    }
                    // PlayerAction::INTERACT => {}
                    _ => self.next_pos = self.p.pos,
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
        let cam = camera_transform(self.p.pos);
        self.gl.draw(args.viewport(), |c, gl| {
            let ct = multiply(c.transform, cam);
            clear([0.0, 0.0, 0.0, 1.0], gl);
            self.lvl.draw(ct, gl);
            self.p.draw(ct, gl);
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
