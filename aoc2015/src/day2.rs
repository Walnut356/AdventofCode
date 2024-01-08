#![allow(unused_imports)]
use rayon::prelude::*;

pub fn part_one(data: String) -> usize {
    data.lines().par_bridge().into_par_iter().map( |line| {
        let mut dim: Vec<usize> = line.split('x').map(|x| x.parse().unwrap()).collect();

        dim.sort();

        (2 * dim[0] * dim[1] ) + (2 * dim[1] * dim[2]) + (2 * dim[2] * dim[0]) + dim[0] * dim[1]
    }).sum()
}

pub fn part_two(data: String) -> usize {
    data.lines().par_bridge().map( |line| {
        let mut dim: Vec<usize> = line.split('x').map(|x| x.parse().unwrap()).collect();

        dim.sort();

        (2 * dim[0] + 2 * dim[1]) + (dim[0] * dim[1] * dim[2])
    }).sum()
}