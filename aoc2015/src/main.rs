use std::{fs::File, slice};
use std::io::prelude::*;
use std::time::Instant;

use aoc2015::*;



fn main() {
    let data = get_data(6);

    let data2 = data.clone();

    timeit!(
        "Part One"
        let res_1 = day6::p1(data)
    );

    dbg!(res_1);
    timeit!(
        "Part Two"
        let res_2 = day6::p2(data2)
    );

    dbg!(res_2);
}