// mod player;

use phf::phf_map;
use phf::phf_set;

use std::collections::HashMap;

use graphics::types::Color;
use piston::input::Key;

use crate::player::PlayerAction;

pub const WINDOW_SIZE: (f64, f64) = (660.0, 540.0);
const _TILE_SIZE: f64 = 60.0;
pub const TILE_SIZE: (f64, f64) = (_TILE_SIZE, _TILE_SIZE);
pub const FRAMES_PER_SECOND: u64 = 30;

pub type Controls = HashMap<Key, PlayerAction>;
pub fn controls() -> Controls {
    HashMap::from([
        (Key::W, PlayerAction::Move(Directions::UP)),
        (Key::S, PlayerAction::Move(Directions::DOWN)),
        (Key::A, PlayerAction::Move(Directions::LEFT)),
        (Key::D, PlayerAction::Move(Directions::RIGHT)),
        (Key::I, PlayerAction::Interact),
    ])
}

pub type Direction = [i32; 2];

#[non_exhaustive]
pub struct Directions;
impl Directions {
    pub const UP: Direction = [0, -1];
    pub const RIGHT: Direction = [1, 0];
    pub const DOWN: Direction = [0, 1];
    pub const LEFT: Direction = [-1, 0];
    // pub const NULL: Direction = [0, 0];
}

pub type TileColor = phf::Map<u8, Color>;
pub static TILE_MAP: TileColor = phf_map! {
    b'g' => [0.6156862, 0.9647058, 0.1686274, 1.0],
    b'_' => [0.9176471, 0.7294117, 0.1607843, 1.0],
    b'T' => [0.0117647, 0.7882352, 0.0117647, 1.0],
    b'r' => [0.7882352, 0.7803921, 0.7529411, 1.0],
};
pub static COLLIDEABLE: phf::Set<char> = phf_set! {'T', 'r'};

// pub type Interactions = phf::Map<u8, &'static str>;
// pub static INTERACT_MAP: Interactions = phf_map! {
//     b'g' => "It\'s grass",
//     b'_' => "It\'s a path",
//     b'T' => "It\'s a tree",
//     b'r' => "It\'s a rock",
// };
