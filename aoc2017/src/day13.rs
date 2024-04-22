#![allow(unused_imports)]

use crate::get_data;

pub fn p1(data: String) -> usize {
    let mut walls = [0u8; 99];

    data.lines().for_each(|line| {
        let mut tokens = line.split_ascii_whitespace();
        let idx = tokens
            .next()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();
        walls[idx] = tokens.next().unwrap().parse::<u8>().unwrap();
    });

    let mut result = 0;

    // let mut scan_pos = [0u8; 7];
    // println!("{:?}", scan_pos);

    for i in 1..walls.len() {
        if walls[i] > 0 && i % (((walls[i] as usize).saturating_sub(1)) * 2) == 0 {
            result += i * walls[i] as usize;
        }

    }

    result
}

pub fn p2(data: String) -> usize {
    let mut walls = [0u8; 99];

    data.lines().for_each(|line| {
        let mut tokens = line.split_ascii_whitespace();
        let idx = tokens
            .next()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();
        walls[idx] = tokens.next().unwrap().parse::<u8>().unwrap();
    });

    let mut result = 0;

    // let mut scan_pos = [0u8; 7];
    // println!("{:?}", scan_pos);
    let mut wait = 0;

    loop {
        'cont: {
            for i in 0..walls.len() {
                if walls[i] > 0 && (wait + i) % (((walls[i] as usize).saturating_sub(1)) * 2).max(1) == 0 {
                    break 'cont;
                }
            }
            return wait;
        }
        wait += 1;
        // dbg!(wait);
    }
}

#[test]
fn test_d13() {
    let data = get_data(13);
    assert_eq!(p1(data.clone()), 632);
    assert_eq!(p2(data), 3849742);
}
