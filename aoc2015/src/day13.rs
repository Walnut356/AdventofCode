#![allow(unused_imports)]

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

use crate::get_data;

type InnerMap<'a> = FxHashMap<&'a str, isize>;
type Map<'a> = FxHashMap<&'a str, InnerMap<'a>>;
type Set<'a> = FxHashSet<&'a str>;


// there's so few routes to check that i'm not gonna bother caching it atm
fn arrange(person: &str, map: &Map, placed: Set, first: &str) -> isize {
    let mut result = isize::MIN;
    let mut set = placed.clone();
    set.insert(person);

    let binding: Set = map.keys().copied().collect();
    let remaining = binding.difference(&set).collect::<Vec<_>>();
    if remaining.len() == 0 {
        return *map.get(person).unwrap().get(first).unwrap() + *map.get(first).unwrap().get(person).unwrap();
    }

    for &key in remaining {
        let me_to_you = *map.get(person).unwrap().get(key).unwrap();
        let you_to_me = *map.get(key).unwrap().get(person).unwrap();
        let recurse = arrange(key, map, set.clone(), first);
        result = result.max(me_to_you + recurse + you_to_me);
    }


    result
}

pub fn p1(data: String) -> isize {
    let mut result = 0;

    let mut map: Map = FxHashMap::default();
    let mut person = "";

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();

        person = tokens.next().unwrap();
        tokens.next();
        let feeling = if tokens.next().unwrap().starts_with('g') {
            1
        } else {
            -1
        };

        let amount = tokens.next().unwrap().parse::<isize>().unwrap();

        let val = tokens.last().unwrap().trim_end_matches('.');

        if !map.contains_key(person) {
            map.insert(person, FxHashMap::default());
        }

        map.get_mut(person).unwrap().insert(val, amount * feeling);
    }

    let set = Set::default();
    arrange(person, &map, set, person)
}

pub fn p2(data: String) -> isize {
    let mut result = 0;

    let mut map: Map = FxHashMap::default();
    let mut person = "";

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();

        person = tokens.next().unwrap();
        tokens.next();
        let feeling = if tokens.next().unwrap().starts_with('g') {
            1
        } else {
            -1
        };

        let amount = tokens.next().unwrap().parse::<isize>().unwrap();

        let val = tokens.last().unwrap().trim_end_matches('.');

        if !map.contains_key(person) {
            map.insert(person, FxHashMap::default());
        }

        map.get_mut(person).unwrap().insert(val, amount * feeling);
    }

    for (k, v) in map.iter_mut() {
        v.insert("me", 0);
    }

    map.insert("me", InnerMap::default());

    for key in map.keys().copied().collect_vec() {
        map.get_mut("me").unwrap().insert(key, 0);
    }

    let set = Set::default();
    arrange(person, &map, set, person)
}

#[test]
fn test_d13() {
    let data = get_data(13);
    assert_eq!(p1(data.clone()), 733);
    assert_eq!(p2(data), 725);
}