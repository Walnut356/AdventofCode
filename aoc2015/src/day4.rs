#![allow(unused_imports)]
use std::slice;
use std::iter::zip;
use md5;

use crate::get_data;

pub fn p1(data: String) -> u32 {
    let mut result = 0;
    let d = data.trim();
    loop {
        let mut temp = md5::compute(format!("{}{}", &d, result)).0;
        unsafe {
            *(temp.as_mut_ptr().offset(2)) &= 0b1111_0000;

            if slice::from_raw_parts(temp.as_mut_ptr(), 3) == [0, 0, 0] {
                return result;
            }
        }

        result += 1;
    }
}

pub fn p2(data: String) -> usize {
    let mut result = 0;
    let d = data.trim();
    loop {

            if md5::compute(format!("{}{}", &d, result)).0[0..3] == [0, 0, 0] {
                return result;
            }

        result += 1;
    }
}

#[test]
fn test_d4() {
    let data = get_data(4);
    assert_eq!(p1(data.clone()), 282749);
    assert_eq!(p2(data), 9962624);
}