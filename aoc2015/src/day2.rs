#![allow(unused_imports)]
use rayon::prelude::*;

use crate::get_data;

pub fn p1(data: String) -> usize {
    data.lines().par_bridge().into_par_iter().map( |line| {
        let mut dim: Vec<usize> = line.split('x').map(|x| x.parse().unwrap()).collect();

        dim.sort();

        (2 * dim[0] * dim[1] ) + (2 * dim[1] * dim[2]) + (2 * dim[2] * dim[0]) + dim[0] * dim[1]
    }).sum()
}

pub fn p2(data: String) -> usize {
    data.lines().par_bridge().map( |line| {
        let mut dim: Vec<usize> = line.split('x').map(|x| x.parse().unwrap()).collect();

        dim.sort();

        (2 * dim[0] + 2 * dim[1]) + (dim[0] * dim[1] * dim[2])
    }).sum()
}

#[test]
fn test_d2() {
    let data = get_data(2);
    assert_eq!(p1(data.clone()), 1586300);
    assert_eq!(p2(data), 3737498);
}