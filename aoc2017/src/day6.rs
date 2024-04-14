#![allow(unused_imports)]

use itertools::Itertools;

use crate::get_data;
use crate::utils::{Map, Set};

pub fn p1(data: String) -> usize {
    let mut mem = data
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut states: Set<Vec<u32>> = Set::default();
    states.insert(mem.clone());

    let mut steps = 1;
    loop {
        let max_val = *mem.iter().max().unwrap();
        let dist = mem.iter().position(|&x| x == max_val).unwrap();
        mem[dist] = 0;
        let mut i = dist;
        for _ in 0..max_val {
            i += 1;
            if i >= mem.len() {
                i = 0;
            }
            mem[i] += 1;
        }

        // dbg!(&mem);
        if !states.insert(mem.clone()) {
            return steps;
        }
        steps += 1;
    }
}

pub fn p2(data: String) -> usize {
    let mut mem: [u32; 16] = data
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut states: Map<[u32; 16], usize> = Map::default();
    states.insert(mem, 0);

    let mut steps = 1;
    loop {
        let max_val = *mem.iter().max().unwrap();
        let dist = mem.iter().position(|&x| x == max_val).unwrap();
        mem[dist] = 0;
        let mut i = dist;
        for _ in 0..max_val {
            i += 1;
            if i >= mem.len() {
                i = 0;
            }
            mem[i] += 1;
        }

        // dbg!(&mem);
        if let Some(v) = states.insert(mem, steps) {
            return steps - v;
        }
        steps += 1;
    }
}

#[test]
fn test_d6() {
    let data = get_data(6);
    assert_eq!(p1(data.clone()), 0);
    assert_eq!(p2(data), 0);
}
