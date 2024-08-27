#![allow(unused_imports)]

use std::str::FromStr;

use strum::EnumString;

use crate::get_data;

// ahhhhh i missed this =) so convenient
#[derive(Debug, Clone, Copy, EnumString)]
#[strum(serialize_all="lowercase")]
enum Dir {
    N,
    S,
    NE,
    NW,
    SE,
    SW
}

// I use a 3 axis system because it's cooler
#[derive(Debug, Clone)]
struct Pos {
    x: i16,
    y: i16,
    z: i16,
}

pub fn p1(data: String) -> usize {
    let mut pos = Pos {x: 0, y: 0, z: 0};

    data.split(',').for_each(|x| {
        let dir = Dir::from_str(x).unwrap();
        match dir {
            Dir::N => {
                pos.x += 1;
                pos.y -= 1;
            },
            Dir::S => {
                pos.x -= 1;
                pos.y += 1;
            },
            Dir::NE => {
                pos.x += 1;
                pos.z -= 1;
            },
            Dir::NW => {
                pos.y -= 1;
                pos.z += 1;
            },
            Dir::SE => {
                pos.y += 1;
                pos.z -= 1;
            },
            Dir::SW => {
                pos.x -= 1;
                pos.z += 1;
            },
        }
    });

    (pos.x.abs() + pos.y.abs() + pos.z.abs()) as usize / 2 // need to div 2 because each move modifies 2 parts of the coordinate
}

pub fn p2(data: String) -> usize {
    let mut pos = Pos {x: 0, y: 0, z: 0};
    let mut result = 0;

    data.split(',').for_each(|x| {
        let dir = Dir::from_str(x).unwrap();
        match dir {
            Dir::N => {
                pos.x += 1;
                pos.y -= 1;
            },
            Dir::S => {
                pos.x -= 1;
                pos.y += 1;
            },
            Dir::NE => {
                pos.x += 1;
                pos.z -= 1;
            },
            Dir::NW => {
                pos.y -= 1;
                pos.z += 1;
            },
            Dir::SE => {
                pos.y += 1;
                pos.z -= 1;
            },
            Dir::SW => {
                pos.x -= 1;
                pos.z += 1;
            },
        }

        result = result.max((pos.x.abs() + pos.y.abs() + pos.z.abs()) as usize / 2);
    });

    result
}

#[test]
fn test_d11() {
    let data = get_data(11);
    assert_eq!(p1(data.clone()), 707);
    assert_eq!(p2(data), 1490);
}
