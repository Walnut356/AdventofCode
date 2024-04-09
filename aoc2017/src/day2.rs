#![allow(unused_imports)]

use itertools::Itertools;

use crate::get_data;

pub fn p1(data: String) -> usize {
    data.lines().fold(0, |result, line| {
        let mut min = usize::MAX;
        let mut max = 0;
        line.split_ascii_whitespace().for_each(|x| {
            let _ = x.parse::<usize>().inspect(|x| {
                min = min.min(*x);
                max = max.max(*x);
            });
        });
        result + (max - min)
    })
}

pub fn p2(data: String) -> usize {
    data.lines().fold(0, |mut result, line| {
        let vals = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();

        'outter: {
            for v1 in 0..vals.len() - 1 {
                for v2 in v1 + 1..vals.len() {
                    let big = vals[v1].max(vals[v2]);
                    let small = vals[v1].min(vals[v2]);
                    // x86_64 cpus do integer divison and modulo at the same time (and put the div
                    // and rem in different registers), so this should be a single operation
                    if big % small == 0 {
                        result += big / small;
                        break 'outter;
                    }
                }
            }
        }

        result
    })
}

#[test]
fn test_d2() {
    let data = get_data(2);
    assert_eq!(p1(data.clone()), 0);
    assert_eq!(p2(data), 0);
}
