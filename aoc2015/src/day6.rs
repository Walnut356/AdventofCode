#![allow(unused_imports)]

use crate::get_data;
use std::arch::asm; // awwwww yeeeeeaaaahhhhh

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    pub fn new(val: &str) -> Self {
        let mut tokens = val.split(',');
        Self {
            x: tokens.next().unwrap().parse().unwrap(),
            y: tokens.next().unwrap().parse().unwrap(),
        }
    }
}


pub fn p1(data: String) -> usize {
    let bit_array = Box::new([0u8; 125 * 1000]); // 1000 x 1000 bit grid

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let t1 = tokens.next().unwrap();
        let t2 = tokens.next().unwrap();
        match (t1, t2) {
            ("turn", "on") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        let bit_idx = y * 1000 + x;
                        let ptr = bit_array.as_ptr();
                        // not exactly fast, but it is cool. I don't think there's a true equivalent
                        // to this instruction in rust, even in std::intrinsics
                        unsafe {
                            asm!(
                                "bts [{ptr}], {bit_idx}",
                                bit_idx = in(reg) bit_idx,
                                ptr = in(reg) ptr,
                            )
                        }
                    }
                }
            },
            ("turn", "off") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        let bit_idx = y * 1000 + x;
                        let ptr = bit_array.as_ptr();
                        // not exactly fast, but it is cool
                        unsafe {
                            asm!(
                                "btr [{ptr}], {bit_idx}",
                                bit_idx = in(reg) bit_idx,
                                ptr = in(reg) ptr,
                            )
                        }
                    }
                }
            },
            ("toggle", x) => {
                let start = Coord::new(x);
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        let bit_idx = y * 1000 + x;
                        let ptr = bit_array.as_ptr();
                        unsafe {
                            asm!(
                                "btc [{ptr}], {bit_idx}",
                                bit_idx = in(reg) bit_idx,
                                ptr = in(reg) ptr,
                            )
                        }
                    }
                }
            },
            _ => panic!("malformed input"),
        }
    }

    bit_array.iter().fold(0, |acc, x| acc + x.count_ones() as usize)
}

pub fn p2(data: String) -> usize {
    let mut lights: Vec<usize> = vec![0; 1000 * 1000];

        for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let t1 = tokens.next().unwrap();
        let t2 = tokens.next().unwrap();
        match (t1, t2) {
            ("turn", "on") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        let idx = y * 1000 + x;
                        lights[idx] += 1;
                    }
                }
            },
            ("turn", "off") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        let idx = y * 1000 + x;
                        lights[idx] = lights[idx].saturating_sub(1);
                    }
                }
            },
            ("toggle", x) => {
                let start = Coord::new(x);
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        let idx = y * 1000 + x;
                        lights[idx] += 2;
                    }
                }
            },
            _ => panic!("malformed input"),
        }
    }

    lights.iter().sum()
}

#[test]
fn test_d6() {
    let data = get_data(6);
    assert_eq!(p1(data.clone()), 377891);
    assert_eq!(p2(data), 14110788);
}