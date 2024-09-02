#![allow(unused_imports)]

use std::str::FromStr;

use crate::get_data;
use crate::utils::*;

#[derive(Debug, Clone, Copy)]
#[repr(i8)]
enum Dir {
    Left = -1,
    Right = 1,
}

#[derive(Debug, Clone, Copy)]
struct Op {
    write: bool,
    shift: Dir,
    new_state: u8,
}

struct Machine {
    total_steps: usize,
    state: u8,
    pos: isize,
    ops: Map<(u8, bool), Op>,
    tape: Map<isize, bool>,
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first = lines.next().unwrap().as_bytes();
        let state = first[first.len() - 2];

        let total_steps: usize = lines
            .next()
            .unwrap()
            .strip_prefix("Perform a diagnostic checksum after ")
            .unwrap()
            .strip_suffix(" steps.")
            .unwrap()
            .parse()
            .unwrap();

        let mut machine = Machine {
            total_steps,
            state,
            pos: 0,
            ops: Map::default(),
            tape: Map::default(),
        };

        lines.next(); // skip blank line

        while let Some(def) = lines.next() {
            let state = def.as_bytes()[def.as_bytes().len() - 2];
            lines.next(); // 0-state is always defined first

            let write = lines.next().unwrap().bytes().nth_back(1).unwrap() == b'1';
            // lines end in "right." or "left.", so we can just test the second to last letter to
            // differentiate
            let shift = match lines.next().unwrap().bytes().nth_back(2).unwrap() {
                b'f' => Dir::Left,
                b'h' => Dir::Right,
                _ => panic!("unknown direction"),
            };
            let new_state = lines.next().unwrap().bytes().nth_back(1).unwrap();

            machine.ops.insert(
                (state, false),
                Op {
                    write,
                    shift,
                    new_state,
                },
            );

            lines.next(); // 1-state is always defined second
            let write = lines.next().unwrap().bytes().nth_back(1).unwrap() == b'1';

            // lines end in "right." or "left.", so we can just test the second to last letter to
            // differentiate
            let shift = match lines.next().unwrap().bytes().nth_back(2).unwrap() {
                b'f' => Dir::Left,
                b'h' => Dir::Right,
                _ => panic!("unknown direction"),
            };
            let new_state = lines.next().unwrap().bytes().nth_back(1).unwrap();

            machine.ops.insert(
                (state, true),
                Op {
                    write,
                    shift,
                    new_state,
                },
            );

            lines.next(); // skip blank line
        }

        Ok(machine)
    }
}

impl Machine {
    pub fn run(&mut self) {
        for _i in 0..self.total_steps {
            let bit = self.tape.entry(self.pos).or_insert(false);
            let op = self.ops.get(&(self.state, *bit)).unwrap();
            *bit = op.write;
            self.pos += op.shift as i8 as isize;
            self.state = op.new_state;
        }
    }
}

pub fn p1(data: String) -> usize {
    let mut machine = data.parse::<Machine>().unwrap();
    machine.run();

    let mut result = 0;
    for (_, v) in machine.tape {
        result += v as usize
    }

    result
}

pub fn p2(_data: String) -> usize {
    0
}

#[test]
fn test_d25() {
    let data = get_data(25);
    assert_eq!(p1(data.clone()), 3578);
    assert_eq!(p2(data), 0);
}
