#![allow(unused_imports)]

use std::str::FromStr;

use itertools::Itertools;

use crate::get_data;
use crate::utils::Map;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Part(pub u8, pub u8);

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('/').unwrap();
        Ok(Part(a.parse().unwrap(), b.parse().unwrap()))
    }
}

impl Part {
    pub fn strength(&self) -> usize {
        self.0 as usize + self.1 as usize
    }

    pub fn can_fit(&self, val: Part) -> bool {
        self.1 == val.0 || self.1 == val.1
    }
}

fn dfs(curr: &mut Vec<Part>, parts: &[Part], cache: &mut Map<Vec<Part>, usize>) -> usize {
    if cache.contains_key(curr) || parts.is_empty() {
        return curr.iter().map(|x| x.strength()).sum();
    }

    let last = *curr.last().unwrap();
    let next_parts = parts.iter().enumerate().filter(|(_i, x)| last.can_fit(**x));

    let mut result = 0;

    for (i, p) in next_parts {
        let mut p = *p;
        if p.1 == last.1 {
            std::mem::swap(&mut p.0, &mut p.1);
        }
        curr.push(p);
        let mut temp = parts.to_vec();
        temp.remove(i);

        result = result.max(dfs(curr, &temp, cache));
        curr.pop();
    }

    cache.insert(curr.clone(), result);

    result.max(curr.iter().map(|x| x.strength()).sum())
}

pub fn p1(data: String) -> usize {
    let mut parts = data
        .lines()
        .map(|x| x.parse::<Part>().unwrap())
        .collect::<Vec<_>>();

    parts.sort_by_key(|b| std::cmp::Reverse(b.strength()));

    let mut curr = Vec::new();
    let start = parts
        .iter()
        .find_position(|x| x.0 == 0 || x.1 == 0)
        .unwrap();
    curr.push(*start.1);
    parts.remove(start.0);

    let mut map = Map::default();
    dfs(&mut curr, &parts, &mut map)
}

fn dfs_2(
    curr: &mut Vec<Part>,
    parts: &[Part],
    cache: &mut Map<Vec<Part>, (usize, usize)>,
) -> (usize, usize) {
    if parts.is_empty() {
        return (curr.len(), curr.iter().map(|x| x.strength()).sum());
    }

    let last = *curr.last().unwrap();
    let next_parts = parts.iter().enumerate().filter(|(_i, x)| last.can_fit(**x));

    let mut result = (0, 0);

    for (i, p) in next_parts {
        let mut p = *p;
        if p.1 == last.1 {
            std::mem::swap(&mut p.0, &mut p.1);
        }
        curr.push(p);
        let mut temp = parts.to_vec();
        temp.remove(i);
        let val = dfs_2(curr, &temp, cache);
        if val.0 > result.0 || (val.0 == result.0 && val.1 > result.1) {
            result = val;
        }

        curr.pop();
    }

    cache.insert(curr.clone(), result);

    let val = (curr.len(), curr.iter().map(|x| x.strength()).sum());
    if val.0 > result.0 || (val.0 == result.0 && val.1 > result.1) {
        val
    } else {
        result
    }
}

pub fn p2(data: String) -> usize {
    let mut parts = data
        .lines()
        .map(|x| x.parse::<Part>().unwrap())
        .collect::<Vec<_>>();

    parts.sort_by_key(|b| std::cmp::Reverse(b.strength()));

    let mut curr = Vec::new();
    let start = parts
        .iter()
        .find_position(|x| x.0 == 0 || x.1 == 0)
        .unwrap();
    curr.push(*start.1);
    parts.remove(start.0);

    let mut map = Map::default();
    dfs_2(&mut curr, &parts, &mut map).1
}

#[test]
fn test_d24() {
    let data = get_data(24);
    assert_eq!(p1(data.clone()), 1695);
    assert_eq!(p2(data), 1673);
}
