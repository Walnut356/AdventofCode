#![allow(unused_imports)]
use strum::EnumString;
use std::collections::HashSet;

// #[derive(Debug, EnumString)]
// enum Direction {

// }

#[derive(Debug, Hash, Eq, Clone, PartialEq, Copy)]
struct Pos(i32, i32);

pub fn part_one(data: String) -> usize {
    let mut pos = Pos(0, 0);
    let mut map: HashSet<Pos> = HashSet::new();
    map.insert(pos);

    for c in data.chars() {
        match c {
            '^' => pos.1 += 1,
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            'v' => pos.1 -= 1,
            _ => panic!("malformed input"),
        }
        map.insert(pos);
    }

    map.len()
}

pub fn part_two(data: String) -> usize {
    let mut pos = Pos(0, 0);
    let mut map: HashSet<Pos> = HashSet::new();
    map.insert(pos);

    for c in data.chars().step_by(2) {
        match c {
            '^' => pos.1 += 1,
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            'v' => pos.1 -= 1,
            _ => panic!("malformed input"),
        }
        map.insert(pos);
    }

    let mut pos = Pos(0, 0);
    for c in data.chars().skip(1).step_by(2) {
        match c {
            '^' => pos.1 += 1,
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            'v' => pos.1 -= 1,
            _ => panic!("malformed input"),
        }
        map.insert(pos);
    }

    map.len()
}