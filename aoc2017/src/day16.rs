#![allow(unused_imports)]

use crate::{get_data, utils::Map};

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Op {
    Spin(u8),
    Exchange(u8, u8),
    Partner(u8, u8),
}

pub fn p1(data: String) -> String {
    let mut ops = Vec::new();
    for op in data.trim().split(',') {
        match op.as_bytes().first().unwrap() {
            b's' => ops.push(Op::Spin(op.get(1..).unwrap().parse().unwrap())),
            b'x' => {
                let (lhs, rhs) = op.get(1..).unwrap().split_once('/').unwrap();
                ops.push(Op::Exchange(lhs.parse().unwrap(), rhs.parse().unwrap()));
            }
            b'p' => {
                let (lhs, rhs) = op.get(1..).unwrap().split_once('/').unwrap();
                ops.push(Op::Partner(lhs.as_bytes()[0], rhs.as_bytes()[0]));
            }
            _ => panic!("unreachable"),
        }
    }

    let mut ring = VecDeque::with_capacity(15);
    for i in b'a'..=b'p' {
        ring.push_back(i);
    }

    for op in ops {
        match op {
            Op::Spin(x) => ring.rotate_right(x as usize),
            Op::Exchange(x, y) => {
                let x = &mut ring[x as usize] as *mut u8;
                let y = &mut ring[y as usize] as *mut u8;
                // can't use mem::swap because taking 2 mutable references to ring isn't
                // allowed. Pretty trivially safe though since it would panic if x or y was out of
                // bounds and we don't manipulate the pointer at all before dereferencing it.
                unsafe { std::ptr::swap(x, y) };
            }
            Op::Partner(x, y) => {
                let x = ring.iter_mut().find(|val| **val == x).unwrap() as *mut u8;
                let y = ring.iter_mut().find(|val| **val == y).unwrap() as *mut u8;
                unsafe { std::ptr::swap(x, y) };
            }
        }
    }

    String::from_utf8(std::convert::Into::<Vec<u8>>::into(ring)).unwrap()
}

fn dance(state: [u8; 16], ops: &[Op], map: &mut Map<[u8; 16], [u8; 16]>) -> Vec<u8> {
    if map.contains_key(&state) {
        return map[&state].to_vec();
    }

    let mut ring = VecDeque::from(state);

    for &op in ops {
        match op {
            Op::Spin(x) => ring.rotate_right(x as usize),
            Op::Exchange(x, y) => {
                let x = &mut ring[x as usize] as *mut u8;
                let y = &mut ring[y as usize] as *mut u8;
                // can't use mem::swap because taking 2 mutable references to ring isn't
                // allowed. Pretty trivially safe though since it would panic if x or y was out of
                // bounds and we don't manipulate the pointer at all before dereferencing it.
                unsafe { std::ptr::swap(x, y) };
            }
            Op::Partner(x, y) => {
                let x = ring.iter_mut().find(|val| **val == x).unwrap() as *mut u8;
                let y = ring.iter_mut().find(|val| **val == y).unwrap() as *mut u8;
                unsafe { std::ptr::swap(x, y) };
            }
        }
    }

    let val: Vec<u8> = ring.into();
    map.insert(state, val.as_slice().try_into().unwrap());
    val

}

pub fn p2(data: String) -> String {
    let mut ops = Vec::new();
    for op in data.trim().split(',') {
        match op.as_bytes().first().unwrap() {
            b's' => ops.push(Op::Spin(op.get(1..).unwrap().parse().unwrap())),
            b'x' => {
                let (lhs, rhs) = op.get(1..).unwrap().split_once('/').unwrap();
                ops.push(Op::Exchange(lhs.parse().unwrap(), rhs.parse().unwrap()));
            }
            b'p' => {
                let (lhs, rhs) = op.get(1..).unwrap().split_once('/').unwrap();
                ops.push(Op::Partner(lhs.as_bytes()[0], rhs.as_bytes()[0]));
            }
            _ => panic!("unreachable"),
        }
    }

    let mut ring = Vec::with_capacity(15);
    for i in b'a'..=b'p' {
        ring.push(i);
    }

    let mut map = Map::default();

    for _ in 0..1_000_000_000 {
        ring = dance(ring.as_slice().try_into().unwrap(), &ops, &mut map);
    }

    String::from_utf8(ring).unwrap()
}

#[test]
fn test_d16() {
    let data = get_data(16);
    assert_eq!(p1(data.clone()), "gkmndaholjbfcepi");
    assert_eq!(p2(data), "abihnfkojcmegldp");
}
