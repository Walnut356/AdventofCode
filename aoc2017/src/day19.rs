#![allow(unused_imports)]

use std::str::FromStr;

use itertools::Itertools;

use crate::get_data;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    Corner,
    Letter,
    Blank,
}

impl Tile {
    fn tile_type(val: u8) -> Tile {
        match val {
            b'-' => Tile::Horizontal,
            b'|' => Tile::Vertical,
            b'+' => Tile::Corner,
            _ => {
                if val.is_ascii_alphabetic() {
                    Tile::Letter
                } else {
                    Tile::Blank
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn find_start(first_line: &[u8]) -> usize {
    first_line.iter().find_position(|x| **x == b'|').unwrap().0
}

pub fn p1(data: String) -> String {
    let map = data.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut result = String::new();

    // (x, y)
    let mut pos = (find_start(map[0]), 0);
    let mut direction = Direction::Down;

    loop {
        pos = match direction {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
        };

        match Tile::tile_type(map[pos.1][pos.0]) {
            Tile::Corner => match direction {
                Direction::Up | Direction::Down => {
                    if map[pos.1]
                        .get(pos.0 + 1)
                        .is_some_and(|x| *x == b'-' || x.is_ascii_alphabetic())
                    {
                        direction = Direction::Right;
                    } else if map[pos.1]
                        .get(pos.0 - 1)
                        .is_some_and(|x| *x == b'-' || x.is_ascii_alphabetic())
                    {
                        direction = Direction::Left;
                    } else {
                        panic!("uh oh")
                    }
                }
                Direction::Right | Direction::Left => {
                    if map
                        .get(pos.1 + 1)
                        .is_some_and(|x| x[pos.0] == b'|' || x[pos.0].is_ascii_alphabetic())
                    {
                        direction = Direction::Down;
                    } else if map
                        .get(pos.1 - 1)
                        .is_some_and(|x| x[pos.0] == b'|' || x[pos.0].is_ascii_alphabetic())
                    {
                        direction = Direction::Up;
                    } else {
                        panic!("uh oh")
                    }
                }
            },
            Tile::Letter => result.push(map[pos.1][pos.0] as char),
            Tile::Blank => break,
            _ => continue,
        }
    }

    result
}

pub fn p2(data: String) -> usize {
    let map = data.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut result = 0;

    // (x, y)
    let mut pos = (find_start(map[0]), 0);
    let mut direction = Direction::Down;

    loop {
        pos = match direction {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
        };

        result += 1;

        match Tile::tile_type(map[pos.1][pos.0]) {
            Tile::Corner => match direction {
                Direction::Up | Direction::Down => {
                    if map[pos.1]
                        .get(pos.0 + 1)
                        .is_some_and(|x| *x == b'-' || x.is_ascii_alphabetic())
                    {
                        direction = Direction::Right;
                    } else if map[pos.1]
                        .get(pos.0 - 1)
                        .is_some_and(|x| *x == b'-' || x.is_ascii_alphabetic())
                    {
                        direction = Direction::Left;
                    } else {
                        panic!("uh oh")
                    }
                }
                Direction::Right | Direction::Left => {
                    if map
                        .get(pos.1 + 1)
                        .is_some_and(|x| x[pos.0] == b'|' || x[pos.0].is_ascii_alphabetic())
                    {
                        direction = Direction::Down;
                    } else if map
                        .get(pos.1 - 1)
                        .is_some_and(|x| x[pos.0] == b'|' || x[pos.0].is_ascii_alphabetic())
                    {
                        direction = Direction::Up;
                    } else {
                        panic!("uh oh")
                    }
                }
            },
            Tile::Blank => break,
            _ => continue,
        }
    }

    result
}

#[test]
fn test_d19() {
    let data = get_data(19);
    assert_eq!(p1(data.clone()), "FEZDNIVJWT");
    assert_eq!(p2(data), 17200);
}
