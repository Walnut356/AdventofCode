#![allow(unused_imports)]
#![allow(clippy::all)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::get_data;

/*
    the input on this one is a lot of fluff for 2 numbers, so i'm "cheating" a little and just
    putting the row and column numbers directly into the test data document separated by a space
*/
pub fn p1(data: String) -> usize {
    let mut result = 0;

    let (row, col) = {
        let mut tokens = data.split_ascii_whitespace();
        (
            tokens.next().unwrap().parse::<usize>().unwrap(),
            tokens.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    // translate a 2d diagonal 1-indexed coordinates to a 1d 0-index
    // column 1 is always the sum of n-1 (i.e. if (1, 1) = 0 and each box increases by 1, coordinate (5, 1) will be
    // 10). The higher the column number, the higher the row starting point has to be to "meet" it.
    // It's a simple linear relationship, so we simply add them together and subtract 1. Once we have
    // an index in column 0, we can just "step forward" col steps (and subtract 1 since we're translating
    // from 1-indexed to 0-indexed)

    // by doing this, we don't have to make an array to traverse or fake it, we can just handle it
    // the same as a a fibonacci sequence since we know how many steps to take
    let stop = ((row + col - 2) * (row + col - 1) / 2) + (col - 1);

    let mut prev = 20151125;
    for _ in 0..stop {
        prev = (prev * 252533) % 33554393;
    }

    prev
}

pub fn p2(data: String) -> usize {
    let mut result = 0;

    result
}

#[test]
fn test_d25() {
    let data = get_data(25);
    assert_eq!(p1(data.clone()), 0);
    assert_eq!(p2(data), 0);
}
