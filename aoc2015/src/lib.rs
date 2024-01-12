use std::{fs::File, io::Read};

pub mod day6;
pub mod day5;
pub mod day4;
pub mod day3;
pub mod day2;
pub mod day1;

#[macro_export]
macro_rules! timeit {
    ($i:literal $x:stmt) => {
        let now = Instant::now();
        $x
        let dur = now.elapsed();
        println!("{}: {:?}", $i, dur);
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