#![allow(unused_imports)]

use crate::get_data;

pub fn p1(data: String) -> usize {
    let mut result = 0;

    let mut ignore = false;
    let mut garbage = false;
    let mut nesting = 0;

    for char in data.chars() {
        if char == '!' && !ignore {
            ignore = true;
            continue;
        }
        if ignore {
            ignore = false;
            continue;
        }
        if garbage {
            garbage = char != '>';
            continue;
        }

        match char {
            '<' => garbage = true,
            '{' => {
                nesting += 1;
                result += nesting;
            }
            '}' => nesting -= 1,
            _ => (),
        }
    }

    result
}

pub fn p2(data: String) -> usize {
    let mut result = 0;

    let mut ignore = false;
    let mut garbage = false;

    for char in data.chars() {
        if char == '!' && !ignore {
            ignore = true;
            continue;
        }
        if ignore {
            ignore = false;
            continue;
        }
        if garbage {
            garbage = char != '>';
            result += garbage as usize;
            continue;
        }

        garbage = char == '<';
    }

    result
}

#[test]
fn test_d9() {
    let data = get_data(9);
    assert_eq!(p1(data.clone()), 0);
    assert_eq!(p2(data), 0);
}
