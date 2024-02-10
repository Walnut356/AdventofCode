#![allow(unused_imports)]

use crate::get_data;
use std::collections::HashMap;

#[derive(Debug)]
struct Op<'a> {
    lhs: &'a str,
    rhs: &'a str,
    op: char,
    to: &'a str,
}

impl<'a> Op<'a> {
    fn execute(&self, lhs: u16, rhs: u16) -> u16 {
        match self.op{
        'W' => lhs,
        'N' => !lhs,
        'A' => lhs & rhs,
        'O' => lhs | rhs,
        'L' => lhs << rhs,
        'R' => lhs >> rhs,
        _ => panic!("invalid operation {}", self.op),
    }
    }
}


pub fn p1(data: String) -> usize {
    let mut wires: HashMap<&str, u16> = HashMap::new();
    let mut ops = Vec::new();

    // not super fond of this 2 stage approach
    for line in data.lines() {
        let mut tokens = line.split_whitespace();

        let t1 = tokens.next().unwrap();
        let t2 = tokens.next().unwrap();
        let t3 = tokens.next().unwrap();

        if t2 == "->" {
            // load literal
            if let Ok(num) = t1.parse::<u16>() {
                wires.insert(t3, num);
                continue;
            }
            // load wire
            ops.push(Op {
                lhs: t1,
                rhs: "",
                op: 'W',
                to: t3,
            });
            continue;
        }

        // unary op (only NOT)
        if t3 == "->" {
            ops.push(Op {
                lhs: t2,
                rhs: "",
                op: 'N',
                to: tokens.next().unwrap()
            });
            continue;
        }

        // binary op
        assert_eq!(tokens.next().unwrap(), "->");

        ops.push(Op {
            lhs: t1,
            rhs: t3,
            op: t2.chars().next().unwrap(),
            to: tokens.next().unwrap(),
        });

    }

    let mut i = 0;
    while !ops.is_empty() {
        let inst = &ops[i];

        let lhs = inst.lhs.parse().ok().or_else(|| wires.get(inst.lhs).copied());
        let rhs =  inst.rhs.parse().ok().or_else(|| wires.get(inst.rhs).copied());
        if lhs.is_some() && (inst.op == 'N' || inst.op == 'W') {
            wires.insert(inst.to, inst.execute(lhs.unwrap(), 0));
            ops.swap_remove(i);
        } else if lhs.is_some() && rhs.is_some() {
            wires.insert(inst.to, inst.execute(lhs.unwrap(), rhs.unwrap()));
            ops.swap_remove(i);
        }


        i += 1;
        if i >= ops.len() {
            i = 0;
        }
    }

    *wires.get("a").unwrap() as usize
}

pub fn p2(data: String) -> usize {
        let mut wires: HashMap<&str, u16> = HashMap::new();
    let mut ops = Vec::new();

    // not super fond of this 2 stage approach
    for line in data.lines() {
        let mut tokens = line.split_whitespace();

        let t1 = tokens.next().unwrap();
        let t2 = tokens.next().unwrap();
        let t3 = tokens.next().unwrap();

        if t2 == "->" {
            // load literal
            if let Ok(num) = t1.parse::<u16>() {
                wires.insert(t3, num);
                continue;
            }
            // load wire
            ops.push(Op {
                lhs: t1,
                rhs: "",
                op: 'W',
                to: t3,
            });
            continue;
        }

        // unary op (only NOT)
        if t3 == "->" {
            ops.push(Op {
                lhs: t2,
                rhs: "",
                op: 'N',
                to: tokens.next().unwrap()
            });
            continue;
        }

        // binary op
        assert_eq!(tokens.next().unwrap(), "->");

        ops.push(Op {
            lhs: t1,
            rhs: t3,
            op: t2.chars().next().unwrap(),
            to: tokens.next().unwrap(),
        });

    }

    wires.insert("b", 16076);
    let mut i = 0;
    while !ops.is_empty() {
        let inst = &ops[i];

        let lhs = inst.lhs.parse().ok().or_else(|| wires.get(inst.lhs).copied());
        let rhs =  inst.rhs.parse().ok().or_else(|| wires.get(inst.rhs).copied());
        if lhs.is_some()  && (inst.op == 'N' || inst.op == 'W') {
            wires.insert(inst.to, inst.execute(lhs.unwrap(), 0));
            ops.swap_remove(i);
        } else if lhs.is_some() && rhs.is_some() {
            wires.insert(inst.to, inst.execute(lhs.unwrap(), rhs.unwrap()));
            ops.swap_remove(i);
        }

        i += 1;
        if i >= ops.len() {
            i = 0;
        }
    }

    *wires.get("a").unwrap() as usize
}

#[test]
fn test_d7() {
    let data = get_data(7);
    assert_eq!(p1(data.clone()), 16076);
    assert_eq!(p2(data), 2797);
}
