use bitflags::bitflags;

use graphics::{math::*, Graphics};
use graphics::{DrawState, Image};
use opengl_graphics::{Filter, ImageSize, Texture, TextureSettings};

use crate::constants::{Direction, TILE_SIZE};
use std::f64::consts::PI;

pub enum PlayerAction {
    Move(Direction),
    Interact,
}

bitflags! {
    #[derive(Default)]
    pub struct PlayerState: u32 {
        const FACE_UP    = 0b1000_0000;
        const FACE_DOWN  = 0b0100_0000;
        const FACE_LEFT  = 0b0010_0000;
        const FACE_RIGHT = 0b0001_0000;
        const INTERACT   = 0b0000_1000;
    }
}

pub trait Entity {
    fn get_position(self) -> [u32; 2];
}

pub struct Player {
    pub pos: [u32; 2],
    pub state: PlayerState,
    sprite: Texture,
    img: Image,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: [0, 0],
            state: PlayerState::FACE_DOWN,
            sprite: Texture::from_path(
                "assets/pika.png",
                &TextureSettings::new().mag(Filter::Nearest),
            )
            .unwrap(),
            img: Image::new(),
        }
    }

    pub fn update(&mut self, pos: [u32; 2], state: PlayerState) {
        self.pos = pos;
        self.state = state;
    }

    fn facing(&self) -> Matrix2d {
        let mut angle = 0.0;
        match self.state.difference(PlayerState::INTERACT) {
            PlayerState::FACE_UP => angle = PI,
            PlayerState::FACE_DOWN => {}
            PlayerState::FACE_LEFT => angle = PI * 0.5,
            PlayerState::FACE_RIGHT => angle = PI * 1.5,
            _ => {}
        };
        let sx = self.sprite.get_size().0 as f64;
        let sy = self.sprite.get_size().0 as f64;
        multiply(
            translate([sx / 2.0, sy / 2.0]),
            multiply(rotate_radians(angle), translate([sx / -2.0, sy / -2.0])),
        )
    }

    pub fn draw<G: Graphics<Texture = Texture>>(&self, t: Matrix2d, g: &mut G) {
        let sz = self.sprite.get_size();
        self.img.draw(
            &self.sprite,
            &DrawState::default(),
            multiply(
                t,
                multiply(
                    translate([
                        TILE_SIZE.0 * self.pos[0] as f64,
                        TILE_SIZE.1 * self.pos[1] as f64,
                    ]),
                    multiply(
                        scale(TILE_SIZE.0 / sz.0 as f64, TILE_SIZE.1 / sz.1 as f64),
                        self.facing(),
                    ),
                ),
            ),
            g,
        );
    }
}

impl Entity for Player {
    fn get_position(self) -> [u32; 2] {
        self.pos
    }
}
