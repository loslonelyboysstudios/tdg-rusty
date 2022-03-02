// mod player;

use phf::{phf_map, phf_set};
use std::collections::HashMap;

use graphics::types::Color;
use piston::input::Key;

use crate::player::PlayerAction;

pub const WINDOW_SIZE: (f64, f64) = (660.0, 540.0);
const _TILE_SIZE: f64 = 60.0;
pub const TILE_SIZE: (f64, f64) = (_TILE_SIZE, _TILE_SIZE);
pub const FRAMES_PER_SECOND: u64 = 1;

pub type Controls = HashMap<Key, PlayerAction>;
pub type TileColor = phf::Map<u8, Color>;
pub type Interactions = phf::Map<u8, &'static str>;

pub fn controls() -> Controls {
    HashMap::from([
        (Key::W, PlayerAction::MOVE_UP),
        (Key::S, PlayerAction::MOVE_DOWN),
        (Key::A, PlayerAction::MOVE_LEFT),
        (Key::D, PlayerAction::MOVE_RIGHT),
        (Key::I, PlayerAction::INTERACT),
    ])
}

#[non_exhaustive]
pub struct DIRECTION;
impl DIRECTION {
    pub const UP: [i32; 2] = [0, -1];
    pub const RIGHT: [i32; 2] = [1, 0];
    pub const DOWN: [i32; 2] = [0, 1];
    pub const LEFT: [i32; 2] = [-1, 0];
    pub const NULL: [i32; 2] = [0, 0];
}

pub static TILE_MAP: TileColor = phf_map! {
    b'g' => [0.6156862, 0.9647058, 0.1686274, 1.0],
    b'_' => [0.9176471, 0.7294117, 0.1607843, 1.0],
    b'T' => [0.0117647, 0.7882352, 0.0117647, 1.0],
    b'r' => [0.7882352, 0.7803921, 0.7529411, 1.0],
};

pub static COLLIDEABLE: phf::Set<char> = phf_set! {'T', 'r'};

pub static INTERACT_MAP: Interactions = phf_map! {
    b'g' => "It\'s grass",
    b'_' => "It\'s a path",
    b'T' => "It\'s a tree",
    b'r' => "It\'s a rock",
};
