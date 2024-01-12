#![allow(unused_imports)]

use crate::get_data;

pub fn p1(data: String) -> isize{

    let mut result = 0;

    for char in data.chars() {
        match char {
            '(' => result += 1,
            ')' => result -= 1,
            _ => panic!("malformed input")
        }
    }

    result
}

pub fn p2(data: String) -> usize {
    let mut result = 0;

    for (i, char) in data.chars().enumerate() {
        match char {
            '(' => result += 1,
            ')' => result -= 1,
            _ => panic!("malformed input")
        }
        if result == -1 {
            return i + 1;
        }
    }

    0
}

#[test]
fn test_d1() {
    let data = get_data(1);
    assert_eq!(p1(data.clone()), 280);
    assert_eq!(p2(data), 1797);
}