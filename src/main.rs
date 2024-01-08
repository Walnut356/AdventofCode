use std::{fs::File, hint::black_box};
use std::io::prelude::*;
use std::time::Instant;

use aoc2015::*;

macro_rules! timeit {
    ($i:literal $x:stmt) => {
        let now = Instant::now();
        $x
        let dur = now.elapsed();
        println!("{}: {:?}", $i, dur);
    }
}

fn main() {
    let mut file = File::open("./test_data/day2.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let data2 = data.clone();

    timeit!(
        "Part One"
        let res_1 = day2::part_one(data)
    );

    dbg!(res_1);
    timeit!(
        "Part Two"
        let res_2 = day2::part_two(data2)
    );

    dbg!(res_2);
}