#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std::str::FromStr;

use crate::get_data;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, EnumString, Display, PartialEq, Eq)]
enum Opcode {
    /// (r) / 2
    hlf,
    /// (r) * 3
    tpl,
    /// increment (r)
    inc,
    /// jump (offset)
    jmp,
    /// jump even (offset)
    jie,
    /// jump odd (offset)
    jio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
enum R {
    a,
    b,
}

pub fn p1(data: String) -> usize {
    use Opcode::*;

    let instrs = data
        .lines()
        .map(|line| {
            let mut tokens = line.split_ascii_whitespace();

            let op = Opcode::from_str(tokens.next().unwrap()).unwrap();

            let (r, offset) = match op {
                hlf | tpl | inc => (R::from_str(&tokens.next().unwrap()[0..1]).unwrap(), 0),
                jie | jio => (
                    R::from_str(&tokens.next().unwrap()[0..1]).unwrap(),
                    tokens.next().unwrap().parse::<i32>().unwrap(),
                ),
                jmp => (R::a, tokens.next().unwrap().parse::<i32>().unwrap()),
            };

            (op, r, offset)
        })
        .collect::<Vec<_>>();

    let mut rip = 0;
    let mut a: usize = 0;
    let mut b: usize = 0;

    while let Some((op, r, offset)) = instrs.get(rip) {
        let reg: &mut usize = match r {
            R::a => &mut a,
            R::b => &mut b,
        };
        match op {
            hlf => *reg /= 2,
            tpl => *reg *= 3,
            inc => *reg += 1,
            jmp => {
                rip = (rip as i32 + offset) as usize;
                continue;
            }
            jie => {
                if *reg & 1 == 0 {
                    rip = (rip as i32 + offset) as usize;
                    continue;
                }
            }
            jio => {
                if *reg == 1 {
                    rip = (rip as i32 + offset) as usize;
                    continue;
                }
            }
        }
        rip += 1;
    }

    b
}

pub fn p2(data: String) -> usize {
        use Opcode::*;

    let instrs = data
        .lines()
        .map(|line| {
            let mut tokens = line.split_ascii_whitespace();

            let op = Opcode::from_str(tokens.next().unwrap()).unwrap();

            let (r, offset) = match op {
                hlf | tpl | inc => (R::from_str(&tokens.next().unwrap()[0..1]).unwrap(), 0),
                jie | jio => (
                    R::from_str(&tokens.next().unwrap()[0..1]).unwrap(),
                    tokens.next().unwrap().parse::<i32>().unwrap(),
                ),
                jmp => (R::a, tokens.next().unwrap().parse::<i32>().unwrap()),
            };

            (op, r, offset)
        })
        .collect::<Vec<_>>();

    let mut rip = 0;
    let mut a: usize = 1;
    let mut b: usize = 0;

    while let Some((op, r, offset)) = instrs.get(rip) {
        let reg: &mut usize = match r {
            R::a => &mut a,
            R::b => &mut b,
        };
        match op {
            hlf => *reg /= 2,
            tpl => *reg *= 3,
            inc => *reg += 1,
            jmp => {
                rip = (rip as i32 + offset) as usize;
                continue;
            }
            jie => {
                if *reg & 1 == 0 {
                    rip = (rip as i32 + offset) as usize;
                    continue;
                }
            }
            jio => {
                if *reg == 1 {
                    rip = (rip as i32 + offset) as usize;
                    continue;
                }
            }
        }
        rip += 1;
    }

    b
}

#[test]
fn test_d23() {
    let data = get_data(23);
    assert_eq!(p1(data.clone()), 0);
    assert_eq!(p2(data), 0);
}
