#![allow(unused_imports)]

use itertools::Itertools;

use crate::get_data;

use std::collections::VecDeque;

pub fn p1(data: String) -> usize {
    let step: usize = data.parse().unwrap();
    let mut buf = VecDeque::with_capacity(2018);
    buf.push_back(0u16);

    for i in 1u16..2018u16 {
        buf.rotate_left((step + 1) % buf.len());
        buf.push_front(i);
    }

    buf[1] as usize
}

pub fn p2(data: String) -> usize {
    let step: usize = data.parse().unwrap();
    let mut buf = VecDeque::with_capacity(2018);
    buf.push_back(0u32);

    // brute fooooooorce!
    for i in 1u32..50_000_000 {
        buf.rotate_left((step + 1) % buf.len());
        buf.push_front(i);
    }

    buf[buf.iter().find_position(|x| **x == 0).unwrap().0 + 1] as usize
}

#[test]
fn test_d17() {
    let data = get_data(17);
    assert_eq!(p1(data.clone()), 419);
    assert_eq!(p2(data), 46038988);
}
