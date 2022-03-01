// mod player;

use phf::{phf_map, phf_set};
use std::collections::HashMap;

use piston::input::{Key};
use graphics::types::Color;

use crate::player::PlayerState;


pub const WINDOW_SIZE: (f64, f64) = (660.0, 540.0);
pub const TILE_SIZE: (f64, f64) = (60.0, 60.0);
pub const FRAMES_PER_SECOND: u64 = 30;


pub type Controls = HashMap<Key, PlayerState>;
pub type TileColor = phf::Map<u8, Color>;
pub type Interactions = phf::Map<u8, &'static str>;

pub fn controls() -> Controls {
    HashMap::from([
        (Key::W, PlayerState::MOVE_UP),
        (Key::S, PlayerState::MOVE_DOWN),
        (Key::A, PlayerState::MOVE_LEFT),
        (Key::D, PlayerState::MOVE_RIGHT),
        (Key::I, PlayerState::INTERACT),
    ])
}

pub static TILE_MAP: TileColor = phf_map! {
    b'g' => [0.615686275, 0.964705882, 0.168627451, 1.0],
    b'_' => [0.917647059, 0.729411765, 0.160784314, 1.0],
    b'T' => [0.011764706, 0.788235294, 0.011764706, 1.0],
    b'r' => [0.788235294, 0.780392157, 0.752941176, 1.0],
};

pub static COLLIDEABLE: phf::Set<char> = phf_set! {'T', 'r'};

pub static INTERACT_MAP: Interactions = phf_map! {
    b'g' => "It\'s grass",
    b'_' => "It\'s a path",
    b'T' => "It\'s a tree",
    b'r' => "It\'s a rock",
};