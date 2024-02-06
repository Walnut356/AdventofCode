#![allow(unused_imports)]

use crate::get_data;

pub fn p1(data: String) -> usize {
    let mut total = 0;
    let mut adjusted = 0;

    for line in data.lines() {
        let bytes = line.as_bytes();
        total += bytes.len();
        adjusted += bytes.len() - 2;

        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'\\' {
                match bytes[i + 1] {
                    b'x' => {
                        adjusted -= 3;
                        i += 3;
                    }
                    b'\\' | b'\"' => {
                        adjusted -= 1;
                        i += 1;
                    }
                    _ => (),
                }
            }

            i += 1;
        }
    }

    total - adjusted
}

pub fn p2(data: String) -> usize {
    let mut total = 0;
    let mut encoded = 0;

    for line in data.lines() {
        total += line.len();
        encoded += line.len() + 2;

        for c in line.chars() {
            match c {
                '\\' | '\"' => encoded += 1,
                _ => (),
            }
        }

    }

    encoded - total
}

#[test]
fn test_d8() {
    let data = get_data(8);
    assert_eq!(p1(data.clone()), 1333);
    assert_eq!(p2(data), 2046);
}
