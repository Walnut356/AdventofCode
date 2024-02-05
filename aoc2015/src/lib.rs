

use std::{fs::File, io::Read};

pub mod day25;
pub mod day24;
pub mod day23;
pub mod day22;
pub mod day21;
pub mod day20;
pub mod day19;
pub mod day18;
pub mod day17;
pub mod day16;
pub mod day15;
pub mod day14;
pub mod day13;
pub mod day12;
pub mod day11;
pub mod day10;
pub mod dayj;
pub mod day8;
pub mod day7;
pub mod day6;
pub mod day5;
pub mod day4;
pub mod day3;
pub mod day2;
pub mod day1;


#[macro_export]
macro_rules! timeit {
    ($i: ident) => {
        let now = Instant::now();
        let res = $i(data.clone());
        let dur = now.elapsed();
        dur
    }
}

/// dumb workaround because rustanalyzer uses different CWDs for debug/tests vs run/`cargo run --release`
///
/// github says they fixed it (like 3 times). Still broken.
pub fn get_file(path: &str) -> File {
    match File::open(path) {
        Ok(f) => f,
        Err(_) => match File::open(format!("../{path}")) {
            Ok(f) => f,
            Err(_) => File::open(format!("aoc2015/{path}")).unwrap(),
        },
    }
}

pub fn get_data(day: u8) -> String {
    let mut file = get_file(&format!("test_data/day{day}.txt"));
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data
}