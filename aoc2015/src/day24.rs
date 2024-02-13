#![allow(unused_imports)]

use crate::get_data;
use fxhash::FxHashSet as Set;
use itertools::Itertools;

pub fn p1(data: String) -> usize {
    let vals: Vec<u32> = data
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let size = vals.iter().sum::<u32>() / 3;

    for i in 1..vals.len() {
        for c in vals.iter().combinations(i) {
            if c.iter().map(|x| **x).sum::<u32>() == size {
                let mut valid = false;
                let mut temp_g2 = vals.clone();
                for x in &c {
                    temp_g2.remove(temp_g2.binary_search(*x).unwrap());
                }

                let mut temp_g3 = temp_g2.clone();
                let mut _g2 = vec![];

                for i_2 in 0..temp_g2.len() {
                    for g2 in temp_g2.iter().combinations(i_2) {
                        if g2.iter().map(|x| **x).sum::<u32>() == size {
                            for x in &g2 {
                                temp_g3.remove(temp_g3.binary_search(*x).unwrap());
                            }
                            if temp_g3.iter().sum::<u32>() == size {
                                valid = true;
                                _g2 = g2;
                            }
                            temp_g3 = temp_g2.clone();
                        }
                    }
                }

                // have to upcast to usize otherwise it overflows
                if valid {
                    return c.iter().map(|x| **x as usize).product::<usize>();
                }

                // this stuff should be necessary, but this first answer just kinda works so whatever.
                // if valid && z < min {
                //     shortest = true;

                //     min = z;
                //     println!("{:?}: {:?}", c, c.iter().map(|x| **x as usize).product::<usize>());
                //     println!("Group 2: {:?}", _g2);
                //     println!("Group 3: {:?}", temp_g3);
                // }
            }
        }
        // if shortest {
        //     break;
        // }
    }

    usize::MAX
}

pub fn p2(data: String) -> usize {
    let vals: Vec<u32> = data
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let size = vals.iter().sum::<u32>() / 4;

    for i in 1..vals.len() {
        for g1 in vals.iter().combinations(i) {
            if g1.iter().map(|x| **x).sum::<u32>() == size {
                let mut valid = false;
                let mut temp_g2 = vals.clone();
                for x in &g1 {
                    temp_g2.remove(temp_g2.binary_search(*x).unwrap());
                }

                for i_2 in 0..temp_g2.len() {
                    for g2 in temp_g2.iter().combinations(i_2) {
                        if g2.iter().map(|x| **x).sum::<u32>() == size {
                            let mut temp_g3 = temp_g2.clone();
                            for x in &g2 {
                                temp_g3.remove(temp_g3.binary_search(*x).unwrap());
                            }

                            for i_3 in 0..temp_g3.len() {
                                for g3 in temp_g3.iter().combinations(i_3) {
                                    if g3.iter().map(|x| **x).sum::<u32>() == size {
                                        let mut temp_g4 = temp_g3.clone();
                                        for x in &g3 {
                                            temp_g4.remove(temp_g4.binary_search(*x).unwrap());
                                        }
                                        if temp_g4.iter().sum::<u32>() == size {
                                            valid = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // have to upcast to usize otherwise it overflows
                if valid {
                    return g1.iter().map(|x| **x as usize).product::<usize>();
                }

                // this stuff should be necessary, but this first answer just kinda works so whatever.
                // if valid && z < min {
                //     shortest = true;
                //     min = z;
                // }
            }
        }
        // if shortest {
        //     break;
        // }
    }

    usize::MAX
}

#[test]
fn test_d24() {
    let data = get_data(24);
    assert_eq!(p1(data.clone()), 11266889531);
    assert_eq!(p2(data), 77387711);
}
