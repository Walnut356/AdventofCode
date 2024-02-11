#![allow(unused_imports)]

use std::io::Write;

use crate::get_data;

type Lights = [[u8; 100]; 100];

fn print_thing(l: &Lights) {
    let mut lock = std::io::stdout().lock();

    let _ = lock.write_all(b"Lights:\n");
    for inner in l {
        let _ = lock.write_all(format!("{:?}\n", inner).as_bytes());
    }
}

fn tick(lights: &mut Lights) {
    // print_thing(lights);
    let mut result = *lights;
    for row in 0..lights.len() {
        for col in 0..lights.len() {
            let mut neighbors = 0;
            // it wants to infer them as usizes because i'm using them to index =T
            for i in std::ops::RangeInclusive::<i32>::new(-1, 1) {
                for j in std::ops::RangeInclusive::<i32>::new(-1, 1) {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if lights
                        .get((row as i32 + i) as usize)
                        .is_some_and(|l| l.get((col as i32 + j) as usize).is_some_and(|m| *m != 0))
                    {
                        neighbors += 1;
                    }
                }
            }

            match (
                result.get_mut(row).unwrap().get_mut(col).unwrap(),
                neighbors,
            ) {
                (x, 3) if *x == 0 => *x = 1,
                (x, y) if *x == 1 && !(2..=3).contains(&y) => *x = 0,
                _ => (),
            };
        }
    }

    *lights = result;
}

pub fn p1(data: String) -> usize {
    let mut lights: Lights = [[0; 100]; 100];

    for (line_idx, line) in data.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => lights[line_idx][i] = 0,
                '#' => lights[line_idx][i] = 1,
                _ => panic!("Malformed input"),
            }
        }
    }

    for i in 0..100 {
        tick(&mut lights)
    }

    lights.into_iter().flatten().fold(0, |a, b| a + b as usize)
}

fn tick_p2(lights: &mut Lights) {
    // print_thing(lights);
    let mut result = *lights;
    for row in 0..lights.len() {
        for col in 0..lights.len() {
            if matches!((row, col), (0, 0) | (0, 99) | (99, 0) | (99, 99)) {
                continue;
            }
            let mut neighbors = 0;
            // it wants to infer them as usizes because i'm using them to index =T
            for i in std::ops::RangeInclusive::<i32>::new(-1, 1) {
                for j in std::ops::RangeInclusive::<i32>::new(-1, 1) {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if lights
                        .get((row as i32 + i) as usize)
                        .is_some_and(|l| l.get((col as i32 + j) as usize).is_some_and(|m| *m != 0))
                    {
                        neighbors += 1;
                    }
                }
            }

            match (
                result.get_mut(row).unwrap().get_mut(col).unwrap(),
                neighbors,
            ) {
                (x, 3) if *x == 0 => *x = 1,
                (x, y) if *x == 1 && !(2..=3).contains(&y) => *x = 0,
                _ => (),
            };
        }
    }

    *lights = result;
}

pub fn p2(data: String) -> usize {
    let mut lights: Lights = [[0; 100]; 100];

    for (line_idx, line) in data.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '.' => lights[line_idx][i] = 0,
                '#' => lights[line_idx][i] = 1,
                _ => panic!("Malformed input"),
            }
        }
    }

    lights[0][0] = 1;
    lights[0][99] = 1;
    lights[99][0] = 1;
    lights[99][99] = 1;
    for i in 0..100 {
        tick_p2(&mut lights)
    }

    lights.into_iter().flatten().fold(0, |a, b| a + b as usize)
}

#[test]
fn test_d18() {
    let data = get_data(18);
    assert_eq!(p1(data.clone()), 814);
    assert_eq!(p2(data), 0);
}
