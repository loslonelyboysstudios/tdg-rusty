use crate::constants::{TILE_MAP, TILE_SIZE};
use graphics::{math::*, polygon, Graphics};
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Level {
    pub size: [usize; 2],
    map: Vec<u8>,
}

impl Level {
    pub fn new(lvl: &str) -> Level {
        let lvlf = File::open(lvl).unwrap();
        let mut reader = io::BufReader::new(lvlf);

        // get level size
        let mut lvlsz = String::new();
        let len = reader.read_line(&mut lvlsz).unwrap();

        //check format
        if lvlsz.as_bytes()[0] as char != '#' {
            panic!("Wrong lvl format");
        }

        //read size
        let size: [usize; 2];
        let sz = lvlsz[1..len - 1].split_once('.');
        match sz {
            Some((x, y)) => {
                size = [
                    x.parse::<u32>().unwrap() as usize,
                    y.parse::<u32>().unwrap() as usize,
                ]
            }
            None => panic!("Wrong lvl format"),
        }

        //read map
        let mut map: Vec<u8> = vec![0; size[0] * size[1]];
        let mut skip: [u8; 1] = [0];
        let mut c = 0;
        for r in 1..size[0] {
            reader.read_exact(&mut map[c..c + size[1]]).unwrap();
            reader.read_exact(&mut skip).unwrap();
            c = r * size[1];
        }
        Level { size, map }
    }

    pub fn draw<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        let mut r = 0;
        let mut c = 0;
        for cell in self.map.iter() {
            if *cell == 0 {
                continue;
            }
            let color = TILE_MAP[cell];
            polygon(
                color,
                &[
                    [0.0, 0.0],
                    [TILE_SIZE.0, 0.0],
                    [TILE_SIZE.0, TILE_SIZE.1],
                    [0.0, TILE_SIZE.1],
                ],
                multiply(
                    t,
                    translate([c as f64 * TILE_SIZE.0, r as f64 * TILE_SIZE.1]),
                ),
                g,
            );
            c += 1;
            if c >= self.size[0] {
                r += 1;
            }
            c %= self.size[0];
        }
    }
}
