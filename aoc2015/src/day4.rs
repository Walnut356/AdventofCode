#![allow(unused_imports)]

use std::iter::zip;

fn to_message(data: &[u8], val: u32) -> u128 {
    let mut result: [u8; 64] = [0; 64];

    for (i, &c) in data.iter().enumerate() {
        result[i] = c;
    }

    for (i, &b) in zip(data.len()..data.len() + 5, val.to_ne_bytes().iter()) {
        result[i] = b;
    }

    *result.last_mut().unwrap() = data.len() as u8 + 4;

    0
}


pub fn part_one(data: String) -> usize {
    let mut result = 0;



    result
}

pub fn part_two(data: String) -> usize {
    let mut result = 0;



    result
}