#![allow(unused_imports)]
#![allow(clippy::needless_range_loop)]

use crate::utils::Set;
use itertools::Itertools;

use crate::{get_data, utils::knot_hash_raw};

pub fn p1(data: String) -> usize {
    let mut result = 0;

    let input = data.lines().next().unwrap();

    for i in 0..128 {
        let start_value = format!("{input}-{i}");

        let nums = knot_hash_raw(start_value.as_bytes());

        for chunk in &nums.iter().chunks(16) {
            result += chunk.cloned().reduce(|a, b| a ^ b).unwrap().count_ones() as usize;
        }
    }

    result
}

pub fn p2(data: String) -> usize {
    let mut result = 0;

    let input = data.lines().next().unwrap();
    let mut map = [[false; 128]; 128];

    for i in 0..128 {
        let start_value = format!("{input}-{i}");

        let nums = knot_hash_raw(start_value.as_bytes());

        let mut hex_str = String::with_capacity(32);
        for chunk in &nums.iter().chunks(16) {
            hex_str.push_str(&format!(
                "{:08b}",
                chunk.cloned().reduce(|a, b| a ^ b).unwrap()
            ));
        }

        for (j, char) in hex_str.bytes().enumerate() {
            map[i][j] = char == b'1';
        }
    }

    let mut visited: Set<(usize, usize)> = Set::default();

    // let mut temp = [[0u8; 128]; 128];

    for i in 0..128 {
        for j in 0..128 {
            if !visited.insert((i, j)) || !map[i][j] {
                continue;
            }

            let mut stack = vec![(i, j)];
            result += 1;

            while let Some(pos) = stack.pop() {
                // temp[pos.0][pos.1] = result as u8;

                // check all 4 adjacent values and add them to the stack if
                // 1. we haven't already seen them (prevents infinite loops) and
                // 2. they're a "1" value
                let test = (pos.0, pos.1 + 1);
                if !visited.contains(&test)
                    && map
                        .get(test.0)
                        .is_some_and(|x| x.get(test.1).is_some_and(|y| *y))
                {
                    visited.insert(test);
                    stack.push(test);
                }
                let test = (pos.0, pos.1.wrapping_sub(1));
                if !visited.contains(&test)
                    && map
                        .get(test.0)
                        .is_some_and(|x| x.get(test.1).is_some_and(|y| *y))
                {
                    visited.insert(test);
                    stack.push(test);
                }
                let test = (pos.0 + 1, pos.1);
                if !visited.contains(&test)
                    && map
                        .get(test.0)
                        .is_some_and(|x| x.get(test.1).is_some_and(|y| *y))
                {
                    visited.insert(test);
                    stack.push(test);
                }
                let test = (pos.0.wrapping_sub(1), pos.1);
                if !visited.contains(&test)
                    && map
                        .get(test.0)
                        .is_some_and(|x| x.get(test.1).is_some_and(|y| *y))
                {
                    visited.insert(test);
                    stack.push(test);
                }
            }
        }
    }

    // for line in temp {
    //     println!("{:?}", line);
    // }

    result
}

#[test]
fn test_d14() {
    let data = get_data(14);
    assert_eq!(p1(data.clone()), 8250);
    assert_eq!(p2(data), 1113);
}
