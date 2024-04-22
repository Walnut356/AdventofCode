#![allow(unused_imports)]

use std::collections::VecDeque;

use crate::get_data;

pub fn p1(data: String) -> usize {
    // quick inspection of the source data shows the max value is 1999
    let mut pipes: [Vec<u16>; 2000] = std::array::from_fn(|_| Vec::new());

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let src = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next(); // skip "->"

        for dest in tokens {
            let val = if let Some(y) = dest.strip_suffix(',') { y } else { dest };
            pipes[src].push(val.parse::<u16>().unwrap());
        }
    }

    let mut group = [0u16; 2000];
    let mut stack = VecDeque::new();
    stack.push_back(0);

    while !stack.is_empty() {
        let i = stack.pop_back().unwrap();
        for &val in &pipes[i as usize] {
            if group[val as usize] == 0 {
                stack.push_front(val);
                group[val as usize] = 1;
            }
        }
    }

    group.into_iter().sum::<u16>() as usize
}

pub fn p2(data: String) -> usize {
        // quick inspection of the source data shows the max value is 1999
    let mut pipes: [Vec<u16>; 2000] = std::array::from_fn(|_| Vec::new());

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let src = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next(); // skip "->"

        for dest in tokens {
            let val = if let Some(y) = dest.strip_suffix(',') { y } else { dest };
            pipes[src].push(val.parse::<u16>().unwrap());
        }
    }

    let mut group = [0u16; 2000];
    let mut stack = VecDeque::new();
    stack.push_back(0);

    let mut result = 0;

    loop {
        let i = stack.pop_back().unwrap();
        for &val in &pipes[i as usize] {
            if group[val as usize] == 0 {
                stack.push_front(val);
                group[val as usize] = 1;
            }
        }
        if stack.is_empty() {
            result += 1;
            match group.iter().position(|x| *x == 0) {
                Some(x) => stack.push_front(x as u16),
                None => break,
            }
        }
    }

    result
}

#[test]
fn test_d12() {
    let data = get_data(12);
    assert_eq!(p1(data.clone()), 130);
    assert_eq!(p2(data), 189);
}
