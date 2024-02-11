#![allow(unused_imports)]

use crate::get_data;
use fxhash::FxHashMap as Map;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn p1(_data: String) -> usize {
    let target = 36000000 / 10;
    let mut result = 1;

    let mut curr = 10;

    // I want to say there's a formula for calculating the sum of factors, but I don't know it offhand
    // time to be slow I guess.

    // ^-- yup it relies on prime factorization which i ALSO don't know offhand.
    while curr < target {
        curr = (result + 1) + 1;
        curr += (1..(result + 1) / 2)
            .into_par_iter()
            .map(|x| {
                if (result + 1) % (x + 1) == 0 {
                    x
                } else {
                    0
                }
            })
            .sum::<usize>();

        result += 1;
    }

    result
}

pub fn p2(_data: String) -> usize {
    let target = 36000000;
    let mut result: usize = 1;

    let mut curr = 11;

    while curr < target {
        curr = ((result + 1) * 11) + 11;
        curr += 11 * (1..(result + 1) / 2)
            .into_par_iter()
            .map(|x| {
                if (result + 1) % (x + 1) == 0 && result / x <= 50{
                    x
                } else {
                    0
                }
            })
            .sum::<usize>();

        result += 1;
    }

    result
}

#[test]
fn test_d20() {
    let data = get_data(20);
    assert_eq!(p1(data.clone()), 831600);
    assert_eq!(p2(data), 0);
}
