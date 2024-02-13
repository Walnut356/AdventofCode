#![allow(unused_imports)]

use crate::get_data;
use fxhash::FxHashSet as Set;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

fn dfs(
    idxs: Vec<usize>,
    containers: &[u8],
    variants: &mut Set<Vec<usize>>,
    cache: &mut Set<Vec<usize>>,
) {
    if variants.contains(&idxs)
        || cache.contains(&idxs)
        || idxs.len() >= containers.len()
    {
        return;
    }

    containers.iter().enumerate().for_each(|(i, c)| {
        if !idxs.contains(&i) {
            let new = *c + idxs.iter().map(|x| containers[*x]).sum::<u8>();
            match new.cmp(&150) {
                Ordering::Greater => (),
                Ordering::Equal => {
                    let mut new = idxs.clone();
                    new.push(i);
                    new.sort();
                    variants.insert(new);
                }
                Ordering::Less => {
                    let mut new = idxs.clone();
                    new.push(i);
                    new.sort();
                    dfs(new, containers, variants, cache);
                }
            }
        }
    });

    cache.insert(idxs);
}

pub fn p1(data: String) -> usize {
    let mut containers = Vec::new();

    for line in data.lines() {
        containers.push(line.parse::<u8>().unwrap());
    }

    let mut variants: Set<Vec<usize>> = Set::default();
    let mut cache: Set<Vec<usize>> = Set::default();
    dfs(Vec::new(), &containers, &mut variants, &mut cache);

    variants.len()
}

pub fn p2(data: String) -> usize {
    let mut containers = Vec::new();

    for line in data.lines() {
        containers.push(line.parse::<u8>().unwrap());
    }

    let mut variants: Set<Vec<usize>> = Set::default();
    let mut cache: Set<Vec<usize>> = Set::default();
    dfs(Vec::new(), &containers, &mut variants, &mut cache);

    let mut min_len = usize::MAX;
    for v in &variants {
        min_len = min_len.min(v.len());
    }

    let mut count = 0;
    for v in variants {
        if v.len() == min_len {
            count += 1;
        }
    }

    count
}

#[test]
fn test_d17() {
    let data = get_data(17);
    assert_eq!(p1(data.clone()), 4372);
    assert_eq!(p2(data), 4);
}
