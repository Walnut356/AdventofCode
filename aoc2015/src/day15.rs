#![allow(unused_imports)]

use crate::get_data;
use fxhash::FxHashMap as Map;

#[derive(Debug, Clone, Default, Copy)]
struct Stats {
    pub cap: isize,
    pub dur: isize,
    pub flv: isize,
    pub txt: isize,
    pub cal: isize,
}

impl Stats {
    fn product(&self) -> isize {
        self.cap.max(0) * self.dur.max(0) * self.flv.max(0) * self.txt.max(0)
    }
}

impl std::ops::Mul<usize> for Stats {
    type Output = Stats;

    fn mul(self, rhs: usize) -> Self::Output {
        Stats {
            cap: self.cap * rhs as isize,
            dur: self.dur * rhs as isize,
            flv: self.flv * rhs as isize,
            txt: self.txt * rhs as isize,
            cal: self.cal * rhs as isize,
        }
    }
}

impl std::ops::Add<&Stats> for &Stats {
    type Output = Stats;

    fn add(self, rhs: &Stats) -> Self::Output {
        Stats {
            cap: self.cap + rhs.cap,
            dur: self.dur + rhs.dur,
            flv: self.flv + rhs.flv,
            txt: self.txt + rhs.txt,
            cal: self.cal + rhs.cal,
        }
    }
}

/// RECURSIVE
fn grade(running: Stats, vals: &[Stats], idx: usize, remaining: usize) -> isize {
    // base case
    if idx == vals.len() || remaining == 0 {
        return running.product();
    }
    let mut result = 0;

    for i in 0..=remaining {
        let temp = grade(&running + &(vals[idx] * i), vals, idx + 1, remaining - i);
        result = result.max(temp);
    }

    result
}

pub fn p1(data: String) -> isize {
    let mut vals = Vec::new();

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();

        tokens.next().unwrap();
        tokens.next();
        let cap = tokens
            .next()
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<isize>()
            .unwrap();
        tokens.next();
        let dur = tokens
            .next()
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<isize>()
            .unwrap();
        tokens.next();
        let flv = tokens
            .next()
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<isize>()
            .unwrap();
        tokens.next();
        let txt = tokens
            .next()
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<isize>()
            .unwrap();
        tokens.next();
        let cal = tokens.next().unwrap().parse::<isize>().unwrap();

        vals.push(Stats {
            cap,
            dur,
            flv,
            txt,
            cal,
        });
    }

    grade(Stats::default(), &vals, 0, 100)
}

fn grade_2(running: Stats, vals: &[Stats], idx: usize, remaining: usize) -> isize {
    // base case
    if idx == vals.len() || remaining == 0 || running.cal > 500 {
        return if running.cal == 500 {
            running.product()
        } else {
            0
        };
    }
    let mut result = 0;

    for i in 0..=remaining {
        let temp = grade_2(&running + &(vals[idx] * i), vals, idx + 1, remaining - i);
        result = result.max(temp);
    }

    result
}

pub fn p2(data: String) -> isize {
    let mut vals = Vec::new();

    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();

        tokens.next().unwrap();
        tokens.next();
        let cap = tokens
            .next()
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<isize>()
            .unwrap();
        tokens.next();
        let dur = tokens
            .next()
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<isize>()
            .unwrap();
        tokens.next();
        let flv = tokens
            .next()
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<isize>()
            .unwrap();
        tokens.next();
        let txt = tokens
            .next()
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<isize>()
            .unwrap();
        tokens.next();
        let cal = tokens.next().unwrap().parse::<isize>().unwrap();

        vals.push(Stats {
            cap,
            dur,
            flv,
            txt,
            cal,
        });
    }

    grade_2(Stats::default(), &vals, 0, 100)
}

#[test]
fn test_d15() {
    let data = get_data(15);
    assert_eq!(p1(data.clone()), 222870);
    assert_eq!(p2(data), 117936);
}
