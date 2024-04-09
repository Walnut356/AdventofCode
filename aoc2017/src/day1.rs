#![allow(unused_imports)]

use crate::get_data;


pub fn p1(data: String) -> usize {
    let start  = if data.as_bytes()[0] == data.as_bytes()[data.len() - 1] {
        (data.as_bytes()[0] - b'0') as usize
    } else {
        0
    };
    data.as_bytes().windows(2).fold(start, |acc, x| {
        if x[0] == x[1] {
            acc + (x[0] - b'0') as usize
        } else {
        acc
        }
    })
}

pub fn p2(data: String) -> usize {
    let half = data.len() / 2;
    // input is always an even length, therefore the one halfway around the buffer should always be
    // the same pair on both go-rounds
    data.as_bytes().iter().take(half).zip(data.as_bytes().iter().skip(half)).fold(0, |acc, (&x, &y)| {
        if x == y {
            acc + (((x - b'0') as usize) * 2)
        } else {
            acc
        }
    })
}

#[test]
fn test_d1() {
    let data = get_data(1);
    assert_eq!(p1(data.clone()), 0);
    assert_eq!(p2(data), 0);
}