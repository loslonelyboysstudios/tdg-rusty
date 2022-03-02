use bitflags::bitflags;

use graphics::{math::*, Graphics};
use graphics::{DrawState, Image};
use opengl_graphics::{Filter, ImageSize, Texture, TextureSettings};

use crate::constants::TILE_SIZE;
use piston::input::{Button, ButtonArgs, ButtonState};
use std::f64::consts::PI;

bitflags! {
    #[derive(Default)]
    pub struct PlayerAction: u32 {
        const MOVE_UP    = 0b1000_0000;
        const MOVE_DOWN  = 0b0100_0000;
        const MOVE_LEFT  = 0b0010_0000;
        const MOVE_RIGHT = 0b0001_0000;
        const INTERACT   = 0b0000_1000;
    }

    #[derive(Default)]
    pub struct PlayerState: u32 {
        const FACE_UP    = 0b1000_0000;
        const FACE_DOWN  = 0b0100_0000;
        const FACE_LEFT  = 0b0010_0000;
        const FACE_RIGHT = 0b0001_0000;
        const INTERACT   = 0b0000_1000;
    }
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

    pub fn update(&mut self, pos: [u32; 2], state: PlayerState, dt: f64) {
        self.pos = pos;
        self.state = state;
    }

    fn facing(&self, t: Matrix2d) -> Matrix2d {
        let mut angle = 0.0;
        match self.state.difference(PlayerState::INTERACT) {
            PlayerState::FACE_UP => angle = PI,
            PlayerState::FACE_DOWN => {}
            PlayerState::FACE_LEFT => angle = PI * 0.5,
            PlayerState::FACE_RIGHT => angle = PI * 1.5,
            _ => {}
        };
        multiply(t, rotate_radians(angle))
    }

    pub fn draw<G: Graphics<Texture = Texture>>(&self, t: Matrix2d, g: &mut G) {
        let sz = self.sprite.get_size();
        let sx = TILE_SIZE.0 / sz.0 as f64;
        let sy = TILE_SIZE.1 / sz.1 as f64;
        self.img.draw(
            &self.sprite,
            &DrawState::default(),
            multiply(
                t,
                multiply(
                    self.facing(scale(sx, sy)),
                    translate([
                        0.5 * TILE_SIZE.0 * self.pos[0] as f64,
                        0.5 * TILE_SIZE.1 * self.pos[1] as f64,
                    ]),
                ),
            ),
            g,
        );
    }
}
