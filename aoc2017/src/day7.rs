#![allow(unused_imports)]

use itertools::Itertools;

use crate::get_data;
use crate::utils::{Map, Set};

pub fn p1(data: String) -> String {
    let mut leaves = Set::default();
    let mut branches = Set::default();

    data.lines().for_each(|line| {
        let mut tokens = line.split_ascii_whitespace();
        let name = tokens.next().unwrap();
        let _weight = tokens.next().unwrap();

        if tokens.next().is_some() {
            branches.insert(name);
            for leaf in tokens {
                leaves.insert(leaf.strip_suffix(',').unwrap_or(leaf));
            }
        }
    });

    // dbg!(&leaves);
    // dbg!(&branches);
    // dbg!(branches.difference(&leaves).collect::<Vec<_>>());

    return branches.difference(&leaves).next().unwrap().to_string();
}

pub fn sum_branch(
    node: &str,
    weights: &Map<&str, usize>,
    branches: &Map<&str, Vec<&str>>,
    bad_node: &mut usize,
) -> usize {
    let leaves = &branches.get(node);
    if leaves.is_none() || leaves.is_some_and(|x| x.is_empty()) {
        return weights[node];
    }

    let mut result = weights[node];
    let mut ws = Vec::new();

    let leaves = leaves.unwrap();

    for leaf in leaves {
        let w = sum_branch(leaf, weights, branches, bad_node);
        ws.push(w);
        result += w;
    }

    if *bad_node == 0 && !ws.iter().all_equal() {
        let vals = ws.iter().counts();
        let minmax = vals.iter().minmax_by(|x, y| x.1.cmp(y.1));
        let (bad_weight, common_weight) = match minmax {
            itertools::MinMaxResult::MinMax(x, y) => (**x.0, **y.0),
            _ => panic!("unreachable"),
        };

        let diff = common_weight as i32 - bad_weight as i32;
        *bad_node = (weights[leaves[ws.iter().position(|x| *x == bad_weight).unwrap()]] as i32
            + diff) as usize;
    }

    result
}

pub fn p2(data: String) -> usize {
    // let mut leaves = Set::default();
    let mut branches = Map::default();
    let mut weights = Map::default();

    data.lines().for_each(|line| {
        let mut tokens = line.split_ascii_whitespace();
        let name = tokens.next().unwrap();
        let weight = tokens
            .next()
            .unwrap()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap();
        weights.insert(name, weight.parse::<usize>().unwrap());

        if tokens.next().is_some() {
            branches.insert(
                name,
                tokens
                    .map(|leaf| leaf.strip_suffix(',').unwrap_or(leaf))
                    .collect::<Vec<_>>(),
            );
        }
    });

    let leaves = branches.iter().flat_map(|x| x.1).collect::<Set<_>>();

    let root_name = **branches
        .keys()
        .collect::<Set<_>>()
        .difference(&leaves)
        .next()
        .unwrap();

    let mut bad_node = 0;

    sum_branch(root_name, &weights, &branches, &mut bad_node);

    bad_node
}

#[test]
fn test_d7() {
    let data = get_data(7);
    assert_eq!(p1(data.clone()), "vmpywg");
    assert_eq!(p2(data), 1674);
}
