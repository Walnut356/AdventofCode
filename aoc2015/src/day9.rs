#![allow(unused_imports)]

use std::hint::black_box;

use crate::get_data;
use fxhash::{FxHashMap, FxHashSet};
use itertools::{Itertools, Permutations};

// sanity
type Map<'a> = FxHashMap<&'a str, FxHashMap<&'a str, usize>>;


pub fn p1(data: String) -> usize {
    let mut map: Map = FxHashMap::default();

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();

        let node_1 = tokens.next().unwrap();
        tokens.next();
        let node_2 = tokens.next().unwrap();
        tokens.next();
        let dist = tokens.next().unwrap().parse::<usize>().unwrap();

        if !map.contains_key(node_1) {
            map.insert(node_1, FxHashMap::default());
        }
        if !map.contains_key(node_2) {
            map.insert(node_2, FxHashMap::default());
        }

        map.get_mut(node_1).unwrap().insert(node_2, dist);
        map.get_mut(node_2).unwrap().insert(node_1, dist);
    }

    let mut result = usize::MAX;
    for t in map.keys().permutations(map.keys().len()) {
        let mut dist = 0;
        for i in 0..t.len() - 1 {
            dist += map.get(t[i]).unwrap().get(t[i + 1]).unwrap();
        }
        result = result.min(dist);
    }

    result
}

pub fn p2(data: String) -> usize {
        let mut map: Map = FxHashMap::default();

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();

        let node_1 = tokens.next().unwrap();
        tokens.next();
        let node_2 = tokens.next().unwrap();
        tokens.next();
        let dist = tokens.next().unwrap().parse::<usize>().unwrap();

        if !map.contains_key(node_1) {
            map.insert(node_1, FxHashMap::default());
        }
        if !map.contains_key(node_2) {
            map.insert(node_2, FxHashMap::default());
        }

        map.get_mut(node_1).unwrap().insert(node_2, dist);
        map.get_mut(node_2).unwrap().insert(node_1, dist);
    }

    let mut result = 0;
    for t in map.keys().permutations(map.keys().len()) {
        let mut dist = 0;
        for i in 0..t.len() - 1 {
            dist += map.get(t[i]).unwrap().get(t[i + 1]).unwrap();
        }
        result = result.max(dist);
    }

    result
}

#[test]
fn test_d9() {
    let data = get_data(9);
    assert_eq!(p1(data.clone()), 207);
    assert_eq!(p2(data), 804);
}