#![allow(unused_imports)]

use crate::get_data;
use std::{
    arch::asm,
    fs::File,
    io::prelude::{Read, Write},
}; // awwwww yeeeeeaaaahhhhh

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
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

#[inline(never)]
pub fn asm_on(ptr: *mut u8, x: usize, y: usize) {
    let bit_idx = y * 1000 + x;

    unsafe {
        asm!(
            "bts [{ptr}], {bit_idx}",
            bit_idx = in(reg) bit_idx,
            ptr = in(reg) ptr,
        )
    }
}

pub fn p1(data: String) -> usize {
    let mut bit_array = [0u8; 125 * 1000]; // 1000 x 1000 bit grid

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let t1 = tokens.next().unwrap();
        let t2 = tokens.next().unwrap();
        match (t1, t2) {
            ("turn", "on") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());
                let ptr = bit_array.as_mut_ptr();

                for x in start.x ..=end.x {
                    for y in start.y..=end.y {
                        // not exactly fast, but it is cool. I don't think there's a true equivalent
                        // to this instruction in rust, even in std::intrinsics
                        unsafe {
                            asm!(
                                "bts [{ptr}], {bit_idx}",
                                bit_idx = in(reg) y * 1000 + x,
                                ptr = in(reg) ptr,
                            )
                        }
                    }
                }
            }
            ("turn", "off") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                let ptr = bit_array.as_mut_ptr();
                for x in start.x ..=end.x {
                    for y in start.y..=end.y {
                        unsafe {
                            asm!(
                                "btr [{ptr}], {bit_idx}",
                                bit_idx = in(reg) y * 1000 + x,
                                ptr = in(reg) ptr,
                            )
                        }
                    }
                }
            }
            ("toggle", x) => {
                let start = Coord::new(x);
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                let ptr = bit_array.as_mut_ptr();
                for x in start.x ..=end.x {
                    for y in start.y..=end.y {
                        unsafe {
                            asm!(
                                "btc [{ptr}], {bit_idx}",
                                bit_idx = in(reg) y * 1000 + x,
                                ptr = in(reg) ptr,
                            )
                        }
                    }
                }
            }
            _ => panic!("malformed input"),
        }
    }

    bit_array
        .iter()
        .fold(0, |acc, x| acc + x.count_ones() as usize)
}
#[inline(never)]
pub fn bit_on(bit_array: &mut [u8; 125 * 1000], x: usize, y: usize) {
    let idx = (y * 125) + (x / 8);
    let offset = x % 8;
    bit_array[idx] |= 1 << offset;
}
#[inline(never)]
pub fn bit_off(bit_array: &mut [u8; 125 * 1000], x: usize, y: usize) {
    let idx = (y * 125) + (x / 8);
    let offset = x % 8;
    bit_array[idx] &= 0xFF ^ (1 << offset);
}
#[inline(never)]
pub fn bit_toggle(bit_array: &mut [u8; 125 * 1000], x: usize, y: usize) {
    let idx = (y * 125) + (x / 8);
    let offset = x % 8;
    bit_array[idx] ^= 1 << offset;
}

pub fn p1_bit(data: String) -> usize {
    let mut bit_array = [0u8; 125 * 1000]; // 1000 x 1000 bit grid

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let t1 = tokens.next().unwrap();
        let t2 = tokens.next().unwrap();
        match (t1, t2) {
            ("turn", "on") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for x in start.x..=end.x {
                    for y in start.y..=end.y {
                        let idx = (y * 125) + (x / 8);
                        let offset = x % 8;
                        bit_array[idx] |= 1 << offset;
                    }
                }
            }
            ("turn", "off") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for x in start.x..=end.x {
                    for y in start.y..=end.y {
                        let idx = (y * 125) + (x / 8);
                        let offset = x % 8;
                        bit_array[idx] &= 0xFF ^ (1 << offset);
                    }
                }
            }
            ("toggle", x) => {
                let start = Coord::new(x);
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for x in start.x..=end.x {
                    for y in start.y..=end.y {
                        let idx = (y * 125) + (x / 8);
                        let offset = x % 8;
                        bit_array[idx] ^= 1 << offset;
                    }
                }
            }
            _ => panic!("malformed input"),
        }
    }

    bit_array
        .iter()
        .fold(0, |acc, x| acc + x.count_ones() as usize)
}

pub fn p1_bool(data: String) -> usize {
    let mut array = [false; 1000 * 1000]; // 1000 x 1000 bit grid

    for (i, line) in data.lines().enumerate() {
        let mut tokens = line.split_ascii_whitespace();
        let t1 = tokens.next().unwrap();
        let t2 = tokens.next().unwrap();
        match (t1, t2) {
            ("turn", "on") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                let mut count = 0;
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        array[y * 1000 + x] = true;
                        count += 1;
                    }
                }

                println!("On: {count}");
            }
            ("turn", "off") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                let mut count = 0;
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        array[y * 1000 + x] = false;
                        count += 1;
                    }
                }
                println!("Off: {count}");
            }
            ("toggle", x) => {
                let start = Coord::new(x);
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                let mut count = 0;
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        array[y * 1000 + x] ^= true;
                        count += 1;
                    }
                }
                println!("Toggle: {count}");
            }
            _ => panic!("malformed input"),
        }
        println!("{i}: {}", array.iter().fold(0, |acc, x| acc + *x as usize));
    }

    array.iter().fold(0, |acc, x| acc + *x as usize)
}

pub fn p1_airquotes_simd(data: String) -> usize {
    let mut bit_array = [0u64; 15625]; // 1000 x 1000 bit grid

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
                    let start_idx = (y * 1000 + start.x) / 64;
                    let start_offset = (y * 1000 + start.x) % 64;

                    let end_idx = (y * 1000 + end.x) / 64;
                    let end_offset = (y * 1000 + end.x) % 64;

                    let mut start = u64::MAX;

                    // clear bits that occur before our starting point
                    start <<= start_offset;
                    start >>= start_offset;

                    // if we're setting 64 bits or less, we modify only the start value
                    if start_idx == end_idx {
                        // reverse of the above operation
                        start >>= 63 - end_offset;
                        start <<= 63 - end_offset;

                        bit_array[start_idx] |= start;
                        continue;
                    }

                    bit_array[start_idx] |= start;

                    // loop over x values
                    for i in start_idx + 1..end_idx {
                        bit_array[i] = u64::MAX;
                    }

                    let mut end = u64::MAX;

                    // clear off bits past the end of our range
                    end >>= 63 - end_offset;
                    end <<= 63 - end_offset;
                    bit_array[end_idx] |= end;
                }
            }
            ("turn", "off") => {
                let start = Coord::new(tokens.next().unwrap());
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for y in start.y..=end.y {
                    let start_idx = (y * 1000 + start.x) / 64;
                    let start_offset = (y * 1000 + start.x) % 64;

                    let end_idx = (y * 1000 + end.x) / 64;
                    let end_offset = (y * 1000 + end.x) % 64;

                    // println!("start: {}.b{}, end: {}.b{}\n", start_idx, start_offset, end_idx, end_offset);

                    let mut start = u64::MAX;

                    start <<= start_offset;
                    start >>= start_offset;

                    // if we're setting 64 bits or less, we modify only the start value
                    if start_idx == end_idx {
                        // reverse of the above operation
                        start >>= 63 - end_offset;
                        start <<= 63 - end_offset;

                        bit_array[start_idx] &= !start;
                        continue;
                    }

                    bit_array[start_idx] &= !start;

                    // loop over x values
                    for i in start_idx + 1..end_idx {
                        bit_array[i] = 0;
                    }

                    let mut end = u64::MAX;

                    // clear off only the bits within our range
                    end >>= 63 - end_offset;
                    end <<= 63 - end_offset;
                    bit_array[end_idx] &= !end;
                }
            }
            ("toggle", x) => {
                let start = Coord::new(x);
                tokens.next();
                let end = Coord::new(tokens.next().unwrap());

                for y in start.y..=end.y {
                    let start_idx = (y * 1000 + start.x) / 64;
                    let start_offset = (y * 1000 + start.x) % 64;

                    let end_idx = (y * 1000 + end.x) / 64;
                    let end_offset = (y * 1000 + end.x) % 64;

                    if start_idx == end_idx {
                        // xor has a nice bit fiddling trick for operating on a range within a single
                        // value
                        bit_array[start_idx] ^=
                            ((1u64.checked_shl(63 - end_offset as u32).unwrap_or(0))
                                .wrapping_sub(1))
                                ^ ((1u64.checked_shl(64 - start_offset as u32).unwrap_or(0))
                                    .wrapping_sub(1));
                        continue;
                    }

                    let mut start = u64::MAX;

                    start <<= start_offset;
                    start >>= start_offset;

                    bit_array[start_idx] ^= start;

                    // loop over x values
                    for i in start_idx + 1..end_idx {
                        bit_array[i] ^= u64::MAX;
                    }

                    let mut end = u64::MAX;

                    // clear off bits past the end of our range
                    end >>= 63 - end_offset;
                    end <<= 63 - end_offset;

                    bit_array[end_idx] ^= end;
                }
            }
            _ => panic!("malformed input"),
        }
    }

    bit_array
        .iter()
        .fold(0, |acc, x| acc + x.count_ones() as usize)
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
            }
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
            }
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
            }
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
