#![allow(unused_imports)]

use crate::get_data;

pub fn p1(mut data: String) -> usize {
    let mut result = String::default();

    for _i in 0..40 {
        result.clear();
        let mut count = 0;
        let mut current = data.as_bytes()[0] as char;
        for c in data.chars() {
            if c != current {
                result.push(count.to_string().chars().next().unwrap());
                result.push(current);
                count = 0;
                current = c;
            }

            count += 1;
        }

        result.push(count.to_string().chars().next().unwrap());
        result.push(current);

        // dbg!(&result);

        data = result.clone();

    }

    result.len()
}

pub fn p2(mut data: String) -> usize {
    let mut result = String::with_capacity(4_000_000);

    for _i in 0..50 {
        result.clear();
        let mut count = 0;
        let mut current = data.as_bytes()[0] as char;
        for c in data.chars() {
            if c != current {
                result.push(count.to_string().chars().next().unwrap());
                result.push(current);
                count = 0;
                current = c;
            }

            count += 1;
        }

        result.push(count.to_string().chars().next().unwrap());
        result.push(current);

        // dbg!(&result);

        data = result.clone();

    }

    result.len()
}

#[test]
fn test_d10() {
    let data = get_data(10);
    assert_eq!(p1(data.clone()), 252594);
    assert_eq!(p2(data), 3579328);
}