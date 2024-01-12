use std::{fs::File, slice};
use std::io::prelude::*;
use std::time::Instant;

use aoc2015::*;



fn main() {
    dbg!(std::env::current_dir().unwrap());
    let data = get_data(2);

    let data2 = data.clone();

    timeit!(
        "Part One"
        let res_1 = day2::p1(data)
    );

    dbg!(res_1);
    timeit!(
        "Part Two"
        let res_2 = day2::p2(data2)
    );

    dbg!(res_2);
}