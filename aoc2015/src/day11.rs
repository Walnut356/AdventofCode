#![allow(unused_imports)]

use crate::get_data;

fn valid(val: &str) -> bool {
    let mut sequ = false;
    // let mut iol = false;
    let mut two_pair = false;
    let mut pair = false;
    let mut pair_i = 0;

    let bytes = val.as_bytes();
    for i in 0..bytes.len() {
        if !sequ
            // minor monstrosity to safely check for a sequence of 3 values without out of bounds access
            && bytes.get(i + 2).is_some_and(|x| {
                bytes
                    .get(i + 1)
                    .is_some_and(|y| *x == y + 1 && bytes[i] + 1 == *y)
            })
        {
            sequ = true;
        }

        // TODO do these have an exploitable bit pattern?
        if [b'i', b'o', b'l'].contains(&bytes[i]) {
            return false;
        }

        if !two_pair && bytes.get(i + 1).is_some_and(|x| *x == bytes[i]) {
            if pair && i > pair_i {
                two_pair = true;
            } else {
                pair = true;
                pair_i = i + 1;
            }
        }
    }

    sequ && two_pair
}

fn incr(val: &mut String) {
    let bytes = unsafe { val.as_bytes_mut() };
    let mut i = bytes.len() - 1;

    bytes[i] += 1;
    if [b'i', b'o', b'l'].contains(&bytes[i]) {
        bytes[i] += 1;
    }
    let mut carry = bytes[i] > b'z';

    while carry && i > 0 {
        bytes[i] = b'a';
        i -= 1;
        bytes[i] += 1;
        if [b'i', b'o', b'l'].contains(&bytes[i]) {
        bytes[i] += 1;
    }
        carry = bytes[i] > b'z';
    }

    if carry {
        val.push('a')
    }
}

pub fn p1(mut data: String) -> String {
    while !valid(&data) {
        // dbg!(&data);
        incr(&mut data);
    }

    data
}

pub fn p2(data: String) -> String {
    let mut initial = p1(data);
    incr(&mut initial);
    p1(initial)
}

#[test]
fn test_d11() {
    let data = get_data(11);
    assert_eq!(p1(data.clone()), "vzbxxyzz");
    assert_eq!(p2(data), "");
}
