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
pub mod day9;
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

#[macro_export]
macro_rules! day {
    (1) => {

        use day1::*;

        const DAY: u8 = 1;
    };

    (2) => {

        use day2::*;

        const DAY: u8 = 2;
    };

    (3) => {

        use day3::*;

        const DAY: u8 = 3;
    };

    (4) => {

        use day4::*;

        const DAY: u8 = 4;
    };

    (5) => {

        use day5::*;

        const DAY: u8 = 5;
    };

    (6) => {

        use day6::*;

        const DAY: u8 = 6;
    };

    (7) => {

        use day7::*;

        const DAY: u8 = 7;
    };

    (8) => {

        use day8::*;

        const DAY: u8 = 8;
    };

    (9) => {

        use day9::*;

        const DAY: u8 = 9;
    };

    (10) => {

        use day10::*;

        const DAY: u8 = 10;
    };

    (11) => {

        use day11::*;

        const DAY: u8 = 11;
    };

    (12) => {

        use day12::*;

        const DAY: u8 = 12;
    };

    (13) => {

        use day13::*;

        const DAY: u8 = 13;
    };

    (14) => {

        use day14::*;

        const DAY: u8 = 14;
    };

    (15) => {

        use day15::*;

        const DAY: u8 = 15;
    };

    (16) => {

        use day16::*;

        const DAY: u8 = 16;
    };

    (17) => {

        use day17::*;

        const DAY: u8 = 17;
    };

    (18) => {

        use day18::*;

        const DAY: u8 = 18;
    };

    (19) => {

        use day19::*;

        const DAY: u8 = 19;
    };

    (20) => {

        use day20::*;

        const DAY: u8 = 20;
    };

    (21) => {

        use day21::*;

        const DAY: u8 = 21;
    };

    (22) => {

        use day22::*;

        const DAY: u8 = 22;
    };

    (23) => {

        use day23::*;

        const DAY: u8 = 23;
    };

    (24) => {

        use day24::*;

        const DAY: u8 = 24;
    };

    (25) => {

        use day25::*;

        const DAY: u8 = 25;
    };
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