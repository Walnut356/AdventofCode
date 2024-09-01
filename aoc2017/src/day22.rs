#![allow(unused_imports)]

use crate::get_data;
use strum::FromRepr;
use crate::utils::Map;

#[derive(Debug, Clone, Copy, Default, FromRepr)]
#[repr(u8)]
enum Dir {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, Default)]
struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Default)]
struct Bot {
    pub pos: Pos,
    pub direction: Dir,
}

impl Bot {
    pub fn advance(&mut self) {
        match self.direction {
            Dir::Up => self.pos.y -= 1,
            Dir::Right => self.pos.x += 1,
            Dir::Down => self.pos.y += 1,
            Dir::Left => self.pos.x -= 1,
        }
    }

    /// Returns true if infecting a tile, false if cleaning it
    pub fn act(&mut self, tile: &mut u8) -> bool {
        if *tile == b'#' {
            // this is why i always define directions in clockwise order
            self.direction = Dir::from_repr((self.direction as u8 + 1) % 4).unwrap();
            *tile = b'.';
            self.advance();
            false
        } else {
            self.direction = Dir::from_repr(((self.direction as u8).wrapping_sub(1)) % 4).unwrap();
            *tile = b'#';
            self.advance();
            true
        }
    }
}

pub fn p1(data: String) -> usize {
    let mut map = Vec::new();

    // jank to account for the "infinite field"
    // the extraneous pushes just padd by half the total steps we run
    for _ in 0..5000 {
        map.push(vec![b'.'; 5000 * 2 + 25]);
    }

    for line in data.lines() {
        map.push(vec![b'.'; 5000]);
        for b in line.as_bytes().iter() {
            map.last_mut().unwrap().push(*b);
        }
        for _ in 0..5000 {
            map.last_mut().unwrap().push(b'.');
        }
    }

    for _ in 0..5000 {
        map.push(vec![b'.'; 5000 * 2 + 25]);
    }

    let mut bot = Bot::default();
    bot.pos.y = map.len() / 2;
    bot.pos.x = map[bot.pos.y].len() / 2;
    let mut result = 0;
    // dbg!(map[bot.pos.y][bot.pos.x]);
    for _ in 0..10000 {
        result += bot.act(&mut map[bot.pos.y][bot.pos.x]) as usize;
    }

    result
}

impl Bot {
    /// Returns true if infecting a tile, false if cleaning it
    pub fn act_2(&mut self, map: &mut Map<(usize, usize), u8>) -> bool {
        let tile = map.entry((self.pos.x, self.pos.y)).or_insert(b'.');
        match *tile {
            b'#' => {
                // this is why i always define directions in clockwise order
                self.direction = Dir::from_repr((self.direction as u8 + 1) % 4).unwrap();
                *tile = b'F';
                self.advance();
                false
            }
            b'.' => {
                self.direction =
                    Dir::from_repr(((self.direction as u8).wrapping_sub(1)) % 4).unwrap();
                *tile = b'W';
                self.advance();
                false
            }
            b'W' => {
                *tile = b'#';
                self.advance();
                true
            }
            b'F' => {
                self.direction = Dir::from_repr((self.direction as u8 + 2) % 4).unwrap();
                *tile = b'.';
                self.advance();
                false
            }
            _ => panic!("wrong tile type")
        }
    }
}

pub fn p2(data: String) -> usize {
    // can't just casually allocate like 100gb of data. I'd love to, but only 64 gb of ram =(
    // i GUESS i'll work smarter instead of harder
    let mut map = Map::default();

    for (i, line) in data.lines().enumerate() {
        for (j, b) in line.as_bytes().iter().enumerate() {
            // aligning it to be centered on (usize::MAX / 2, usize::MAX / 2)
            map.insert((j + (usize::MAX / 2) - line.len() / 2, i + (usize::MAX / 2) - line.len() / 2), *b);
        }
    }

    let mut bot = Bot::default();
    bot.pos.y = usize::MAX / 2;
    bot.pos.x = usize::MAX / 2;

    let mut result = 0;
    // dbg!(map[&(bot.pos.y, bot.pos.x)]);
    for _ in 0..10000000 {
        result += bot.act_2(&mut map) as usize;
    }

    result

}

#[test]
fn test_d22() {
    let data = get_data(22);
    assert_eq!(p1(data.clone()), 5450);
    assert_eq!(p2(data), 2511957);
}
