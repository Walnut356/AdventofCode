#![allow(unused_imports)]

use std::collections::HashSet;

use crate::get_data;

pub fn p1(data: String) -> usize {
    let mut result = 0;

    for line in data.lines() {
        let mut vowels = 0;
        let mut rep = false;
        let mut bad_pair = false;
        for window in line.as_bytes().windows(2) {
            vowels += match window[0] as char {
                'a' | 'e' | 'i' | 'o' | 'u' => 1,
                _ => 0,
            };

            rep = rep || window[0] == window[1];

            if [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']]
                .contains(&[window[0] as char, window[1] as char])
            {
                bad_pair = true;
                break;
            }
        }
        vowels += match line.as_bytes()[line.len() - 1] as char {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        };

        if vowels >= 3 && rep && !bad_pair {
            result += 1;
        }
    }

    result
}

pub fn p2(data: String) -> usize {
    let mut result = 0;

    for line in data.lines() {
        let mut pair = false;
        // let pairs = HashSet::new();
        // brute force
        for i in 0..line.len() - 1 {
            let needle = &line[i..=i + 1];
            if line.get(i+2..).is_some_and(|l| l.contains(needle)) {
                pair = true;
                break;
            }
        }

        let mut gap = false;
        for window in line.as_bytes().windows(3) {
            if window[0] == window[2] && window[0] != window[1] {
                gap = true;
                break;
            }
        }
        if gap && pair {
            result += 1;
        }
    }

    result
}

#[test]
fn test_d5() {
    let data = get_data(5);
    assert_eq!(p1(data.clone()), 258);
    assert_eq!(p2(data), 53);
}
