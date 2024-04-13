#![allow(unused_imports)]

use itertools::Itertools;

use crate::get_data;
use crate::utils::Set;

pub fn p1(data: String) -> usize {
    let mut result = 0;
    let mut set: Set<&str> = Set::default();

    data.lines().for_each(|line| {
        let mut r = 1;
        for t in line.split_ascii_whitespace() {
            if !set.insert(t) {
                r = 0;
                break;
            };
        }
        set.clear();

        result += r;
    });

    result
}

pub fn p2(data: String) -> usize {
    let mut result = 0;
    let mut set: Set<Vec<u8>> = Set::default();

    data.lines().for_each(|line| {
        let mut r = 1;
        for t in line.split_ascii_whitespace() {
            if !set.insert(t.as_bytes().to_owned()) {
                r = 0;
            }
            for p in t.bytes().permutations(t.len()) {
                set.insert(p);
            }
        }
        set.clear();

        result += r;
    });

    result
}

#[test]
fn test_d4() {
    let data = get_data(4);
    assert_eq!(p1(data.clone()), 325);
    assert_eq!(p2(data), 119);
}
