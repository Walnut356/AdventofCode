use std::{ops::Deref, time::Instant};

use aoc2017::*;

day! {16}

fn main() {
    let data = get_data(DAY);

    let data2 = data.clone();

    let now = Instant::now();
    let res_1 = p1(data);
    let dur = now.elapsed();

    println!("| {DAY}-1 | {res_1} | {dur:?} |");

    let now = Instant::now();
    let res_2 = p2(data2);
    let dur = now.elapsed();

    println!("| {DAY}-2 | {res_2} | {dur:?} |");
}
