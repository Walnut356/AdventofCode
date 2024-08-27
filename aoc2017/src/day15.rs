#![allow(unused_imports)]

use crate::get_data;

pub fn p1(data: String) -> usize {
    const A_FACTOR: usize = 16807;
    const B_FACTOR: usize = 48271;

    const DIV: usize = 2147483647;
    const MASK: usize = 0b1111_1111_1111_1111;

    let mut lines = data.lines();

    let mut a: usize = lines.next().unwrap().parse().unwrap();
    let mut b: usize = lines.next().unwrap().parse().unwrap();
    let mut result = 0;

    for _ in 0..40_000_000 {
        a = (a * A_FACTOR) % DIV;
        b = (b * B_FACTOR) % DIV;
        if a & MASK == b & MASK {
            result += 1;
        }
    }

    result
}

pub fn p2(data: String) -> usize {
    const A_FACTOR: usize = 16807;
    const A_MULTIPLE: usize = 4;
    const B_FACTOR: usize = 48271;
    const B_MULTIPLE: usize = 8;

    const DIV: usize = 2147483647;
    const MASK: usize = 0b1111_1111_1111_1111;

    let mut lines = data.lines();

    let mut a: usize = lines.next().unwrap().parse().unwrap();
    let mut b: usize = lines.next().unwrap().parse().unwrap();
    let mut result = 0;

    for i in 0..5_000_000 {
        a = (a * A_FACTOR) % DIV;
        while a % A_MULTIPLE != 0 {
            a = (a * A_FACTOR) % DIV;
        }
        b = (b * B_FACTOR) % DIV;
        while b % B_MULTIPLE != 0 {
            b = (b * B_FACTOR) % DIV;
        }

        if a & MASK == b & MASK {
            result += 1;
        }
    }

    result
}

#[test]
fn test_d15() {
    let data = get_data(15);
    assert_eq!(p1(data.clone()), 600);
    assert_eq!(p2(data), 313);
}
