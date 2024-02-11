#![allow(unused_imports)]

use std::iter::zip;

use crate::get_data;
use fxhash::FxHashMap as Map;

pub fn p1(data: String) -> isize {
    let mut result = 0;

    for line in data.lines() {
        let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();

        let spd = tokens[3].parse::<isize>().unwrap();
        let dur = tokens[6].parse::<isize>().unwrap();
        let rest = tokens[tokens.len() - 2].parse::<isize>().unwrap();

        let time = 2503;

        // the commented out code is the same speed with numbers this low, but the compiler isn't
        // quite smart enough to turn it into the constant time formula i have below.
        // the assembly differences are pretty neat to look at though

        // while time > dur + rest {
        //     time -= dur + rest;
        //     dist += spd * dur;
        // }

        // if time % (dur + rest) > dur {
        //     dist += spd * dur;
        // }

        let over = time % (dur + rest);
        let adj_time = time - over;
        let temp = adj_time / (dur + rest);
        let extra = if over > dur {
            dur * spd
        } else {
            over * spd
        };

        let dist = extra + (temp * spd * dur);

        result = result.max(dist);
    }


    result
}

pub fn p2(data: String) -> usize {
    let mut stats = Vec::new();
    for line in data.lines() {
        let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();

        let spd = tokens[3].parse::<isize>().unwrap();
        let dur = tokens[6].parse::<isize>().unwrap();
        let rest = tokens[tokens.len() - 2].parse::<isize>().unwrap();

        stats.push((spd, dur, rest));
    }

    let mut points = vec![0; stats.len()];
    let mut dist = vec![0;stats.len()];


    // i could track each one's progress individually and just increment it when necessary, but the
    // state tracking is more annoying than just plugging it into the formula at each step.
    for time in 1..=2503 {
        for (i, (spd, dur, rest)) in stats.iter_mut().enumerate() {
            let over = time % (*dur + *rest);
            let adj_time = time - over;
            let temp = adj_time / (*dur + *rest);
            let extra = if over > *dur {
                *dur * *spd
            } else {
                over * *spd
            };

            dist[i] = extra + (temp * *spd * *dur);
        }

        let max = *dist.iter().max().unwrap();
        for (d, p) in zip(dist.iter_mut(), points.iter_mut()) {
            if *d == max {
                *p += 1;
            }
        }
    }

    points.into_iter().max().unwrap()
}

#[test]
fn test_d14() {
    let data = get_data(14);
    assert_eq!(p1(data.clone()), 2640);
    assert_eq!(p2(data), 1102);
}