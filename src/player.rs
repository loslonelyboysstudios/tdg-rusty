use bitflags::bitflags;
use graphics::math::{Vec2d, Matrix2d};
use graphics::Graphics;
use piston::input::{Button, ButtonArgs, ButtonState};
// pub mod player;

bitflags! {
    #[derive(Default)]
    pub struct PlayerState: u32 {
        const MOVE_UP    = 0b1000_0000;
        const MOVE_DOWN  = 0b0100_0000;
        const MOVE_LEFT  = 0b0010_0000;
        const MOVE_RIGHT = 0b0001_0000;
        const INTERACT   = 0b0000_1000;
    }
}

pub struct Player {
    pos: Vec2d,
    state: PlayerState,
}

impl Player {
    pub fn new() -> Player{
        Player {
            pos: [0.0, 0.0],
            state: PlayerState::default()
        }
    }

    pub fn update(&mut self, dt: f64) {
    }

    pub fn update_inputs(&mut self, b: &ButtonArgs) {
    }

    fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
    }
}