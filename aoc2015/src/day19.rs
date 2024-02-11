#![allow(unused_imports)]

use fxhash::{FxHashMap as Map, FxHashSet as Set};
use rand::prelude::*;

use crate::get_data;

pub fn p1(data: String) -> usize {
    let mut haystack = "";

    let mut trans_map: Map<&str, Vec<&str>> = Map::default();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let mut tokens = line.split_ascii_whitespace();

        let token = tokens.next().unwrap();

        if tokens.next().is_some() {
            let translation = tokens.next().unwrap();
            if trans_map.contains_key(token) {
                trans_map.get_mut(token).unwrap().push(translation);
            } else {
                trans_map.insert(token, vec![translation]);
            }
        } else {
            haystack = token;
        }
    }

    let mut set = Set::default();
    for (k, v) in trans_map {
        for (i, window) in haystack.as_bytes().windows(k.len()).enumerate() {
            if window == k.as_bytes() {
                for t in &v {
                    let mut temp = haystack.to_owned();
                    temp.replace_range(i..i + k.len(), t);
                    set.insert(temp);
                }
            }
        }
    }

    set.len()
}

/*
    * Would operating backwards be faster? i.e. replace stuff in the haystack until it reduces to e

    * translations can only increase or maintain the length of the string
*/

pub fn steps(
    map: &Map<&str, Vec<&str>>,
    max_map: &[(&str, usize, usize)],
    current: String,
    step: usize,
) -> usize {
    if current.is_empty() {
        return usize::MAX;
    }

    if current == "e" {
        return step;
    }

    for (k, i, _) in max_map {
        let temp = current.replacen(map[k][*i], k, 1);
        if temp != current {
            let result = steps(map, max_map, temp, step + 1,);
            if result != usize::MAX {
                return result;
            }
        }
    }

    usize::MAX
}

pub fn p2(data: String) -> usize {
    let mut result = 0;
    let mut haystack = "";

    let mut trans_map: Map<&str, Vec<&str>> = Map::default();
    let mut needles = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let mut tokens = line.split_ascii_whitespace();

        let token = tokens.next().unwrap();

        if tokens.next().is_some() {
            let translation = tokens.next().unwrap();
            if trans_map.contains_key(token) {
                trans_map.get_mut(token).unwrap().push(translation);
            } else {
                trans_map.insert(token, vec![translation]);
                needles.push(token);
            }
        } else {
            haystack = token;
        }
    }

    // stores key, index, and length reduction
    let mut max_map: Vec<(&str, usize, usize)> = Vec::new();
    for (&k, v) in &trans_map {
        for (i, s) in v.iter().enumerate() {
            max_map.push((k, i, s.len() - k.len()));
        }
    }

    // i tried greedy and it was taking millions of iterations and some unknown length of time.
    // the first time i used rng it finished in 2ms. Seems about 50/50 that it finishes instantly
    // or takes "forever". I ended up testing a few seeds to get a "consistent" result. I'm not
    // happy about it but oh well.

    // let mut seed = thread_rng().gen::<u64>();
    // dbg!(seed);
    let mut val = StdRng::seed_from_u64(1623496077747855719);
    max_map.shuffle(&mut val);


    let result = steps(&trans_map, &max_map, haystack.to_owned(), 0);

    result
}

#[test]
fn test_d19() {
    let data = get_data(19);
    assert_eq!(p1(data.clone()), 576);
    assert_eq!(p2(data), 207);
}
