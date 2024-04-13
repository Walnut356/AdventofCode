#![allow(unused_imports)]

use crate::get_data;

pub fn p1(data: String) -> usize {
    let mut jumps = data
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut steps = 0;
    while i < jumps.len() {
        let temp = &mut jumps[i];
        i = (i as i32 + *temp) as usize;
        *temp += 1;

        steps += 1;
    }

    steps
}

pub fn p2(data: String) -> usize {
    let mut jumps = data
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut steps = 0;
    while i < jumps.len() {
        let temp = &mut jumps[i];
        i = (i as i32 + *temp) as usize;
        if *temp >= 3 {
            *temp -= 1;
        } else {
            *temp += 1;
        }

        steps += 1;
    }

    steps
}

#[test]
fn test_d5() {
    let data = get_data(5);
    assert_eq!(p1(data.clone()), 318883);
    assert_eq!(p2(data), 23948711);
}
