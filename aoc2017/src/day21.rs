#![allow(unused_imports)]

use itertools::Itertools;
use std::str::FromStr;

use crate::utils::Map;

use crate::get_data;

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
// struct Pattern(Vec<Vec<bool>>);

// impl Pattern {
//     pub fn starter() -> Self {
//         Self(vec![vec![false, true, false], vec![false, false, true], vec![true, true, true]])
//     }

//     pub fn mirror(&self) -> Self {
//         Self(
//             self.0
//                 .iter()
//                 .map(|x| x.iter().cloned().rev().collect::<Vec<_>>())
//                 .collect::<Vec<_>>(),
//         )
//     }

//     pub fn rotate(&self) -> Self {
//         let mut result = self.clone();

//         for i in 0..self.0.len() {
//             for j in 0..self.0[0].len() {
//                 result.0[j][i] = self.0[i][j];
//             }
//         }

//         result
//     }
// }

// impl FromStr for Pattern {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut result = Vec::new();

//         for l in s.split('/') {
//             result.push(Vec::new());
//             for b in l.as_bytes() {
//                 let b = matches!(b, b'#');
//                 result.last_mut().unwrap().push(b);
//             }
//         }

//         Ok(Self(result))
//     }
// }

#[derive(Clone, PartialEq, Eq, Hash)]
enum Pattern {
    Two([[bool; 2]; 2]),
    Three([[bool; 3]; 3]),
    Four([[bool; 4]; 4]),
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Two(x) => {
                for val in x {
                    for b in val {
                        write!(f, "{}", if *b { '#' } else { '.' })?;
                    }
                    writeln!(f)?
                }
                Ok(())
            }
            Self::Three(x) => {
                for val in x {
                    for b in val {
                        write!(f, "{}", if *b { '#' } else { '.' })?;
                    }
                    writeln!(f)?
                }
                Ok(())
            },
            Self::Four(x) => {
                for val in x {
                    for b in val {
                        write!(f, "{}", if *b { '#' } else { '.' })?;
                    }
                    writeln!(f)?
                }
                Ok(())
            },
        }
    }
}

impl Pattern {
    // pub fn start() -> Pattern {
    //     Self::Three([
    //         [false, true, false],
    //         [false, false, true],
    //         [true, true, true],
    //     ])
    // }

    #[rustfmt::skip]
    pub fn mirror(&self) -> Pattern {
        match self {
            Pattern::Two(x) => Pattern::Two([
                [x[0][1], x[0][0]],
                [x[1][1], x[1][0]]
                ]),
            Pattern::Three(x) => Pattern::Three([
                [x[0][2], x[0][1], x[0][0]],
                [x[1][2], x[1][1], x[1][0]],
                [x[2][2], x[2][1], x[2][0]],
            ]),
            Pattern::Four(x) => {
                Pattern::Four([
                    [x[0][3], x[0][2], x[0][1], x[0][0]],
                    [x[1][3], x[1][2], x[1][1], x[1][0]],
                    [x[2][3], x[2][2], x[2][1], x[2][0]],
                    [x[3][3], x[3][2], x[3][1], x[3][0]],
                ])
            },
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn rotate(&self) -> Pattern {
        match self {
            Pattern::Two(x) => {
                let mut result: [[bool; 2]; 2] = Default::default();

                for i in 0..x.len() {
                    for j in 0..x.len() {
                        result[x.len() - j - 1][i] = x[i][j]
                    }
                }

                Pattern::Two(result)
            }
            Pattern::Three(x) => {
                let mut result: [[bool; 3]; 3] = Default::default();

                for i in 0..x.len() {
                    for j in 0..x.len() {
                        result[x.len() - j - 1][i] = x[i][j]
                    }
                }

                Pattern::Three(result)
            }
            Pattern::Four(x) => {
                let mut result: [[bool; 4]; 4] = Default::default();

                for i in 0..x.len() {
                    for j in 0..x.len() {
                        result[x.len() - j - 1][i] = x[i][j]
                    }
                }

                Pattern::Four(result)
            }
        }
    }
}

impl FromStr for Pattern {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s
            .as_bytes()
            .iter()
            .find_position(|x| **x == b'/')
            .unwrap()
            .0;
        match len {
            2 => {
                let mut result = [[false; 2]; 2];
                for (i, part) in s.split('/').enumerate() {
                    for (j, char) in part.as_bytes().iter().enumerate() {
                        result[i][j] = *char == b'#'
                    }
                }
                Ok(Pattern::Two(result))
            }
            3 => {
                let mut result = [[false; 3]; 3];
                for (i, part) in s.split('/').enumerate() {
                    for (j, char) in part.as_bytes().iter().enumerate() {
                        result[i][j] = *char == b'#'
                    }
                }
                Ok(Pattern::Three(result))
            }
            4 => {
                let mut result = [[false; 4]; 4];
                for (i, part) in s.split('/').enumerate() {
                    for (j, char) in part.as_bytes().iter().enumerate() {
                        result[i][j] = *char == b'#'
                    }
                }
                Ok(Pattern::Four(result))
            }
            _ => panic!("too long"),
        }
    }
}

fn insert_at(pattern: &Pattern, result: &mut [Vec<bool>], start_idx: (usize, usize)) {
    match pattern {
        Pattern::Two(x) => {
            for i in 0..x.len() {
                for j in 0..x.len() {
                    result[start_idx.0 + i][start_idx.1 + j] = x[i][j];
                }
            }
        }
        Pattern::Three(x) => {
            for i in 0..x.len() {
                for j in 0..x.len() {
                    result[start_idx.0 + i][start_idx.1 + j] = x[i][j];
                }
            }
        }
        Pattern::Four(x) => {
            for i in 0..x.len() {
                for j in 0..x.len() {
                    result[start_idx.0 + i][start_idx.1 + j] = x[i][j];
                }
            }
        }
    }
}

fn enhance(current: &mut Vec<Vec<bool>>, map: &Map<Pattern, Pattern>) {
    let len = current.len();
    if len % 2 == 0 {
        let new_len = (len / 2) * 3;
        let mut result = vec![vec![false; new_len]; new_len];
        for i in (0..len - 1).step_by(2) {
            for j in (0..len - 1).step_by(2) {
                let search = Pattern::Two([
                    [current[i][j], current[i][j + 1]],
                    [current[i + 1][j], current[i + 1][j + 1]],
                ]);
                let pattern = map.get(&search).unwrap();

                insert_at(pattern, &mut result, ((i / 2) * 3, (j / 2) * 3))
            }
        }

        std::mem::swap(current, &mut result);
    } else {
        let new_len = (len / 3) * 4;
        let mut result = vec![vec![false; new_len]; new_len];

        for i in (0..len - 2).step_by(3) {
            for j in (0..len - 2).step_by(3) {
                let search = Pattern::Three([
                    [current[i][j], current[i][j + 1], current[i][j + 2]],
                    [
                        current[i + 1][j],
                        current[i + 1][j + 1],
                        current[i + 1][j + 2],
                    ],
                    [
                        current[i + 2][j],
                        current[i + 2][j + 1],
                        current[i + 2][j + 2],
                    ],
                ]);

                // println!("{:?}", &search);
                let pattern = map.get(&search).unwrap();

                insert_at(pattern, &mut result, ((i / 3) * 4, (j / 3) * 4));
            }
        }

        std::mem::swap(current, &mut result);
    }
}

pub fn p1(data: String) -> usize {
    let mut result: Vec<Vec<bool>> = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let mut translations = Map::default();
    for line in data.lines() {
        let (pat, trans) = line.split_once(" => ").unwrap();
        // w a s t e f u l =(
        let pat = pat.parse::<Pattern>().unwrap();
        let trans = trans.parse::<Pattern>().unwrap();
        let flipped = pat.mirror();
        translations.insert(pat.clone(), trans.clone());
        translations.insert(flipped.clone(), trans.clone());

        // pre-rotate and flip all patterns so it doesn't need to be done later
        let mut pat = pat;
        let mut flipped = flipped;
        for _ in 0..4 {
            pat = pat.rotate();
            flipped = flipped.rotate();
            translations.insert(pat.clone(), trans.clone());
            translations.insert(flipped.clone(), trans.clone());
        }
    }

    // let start = Pattern::start();
    // println!("{:?}", start);
    // println!("{:?}", start.rotate());
    // println!("{:?}", start.rotate().rotate());
    // println!("{:?}", start.rotate().rotate().rotate());

    for _i in 0..5 {
        enhance(&mut result, &translations);
        // println!("--{i}--");
        // for val in &result {
        //     for b in val {
        //         print!("{}", if *b { '#' } else { '.' });
        //     }
        //     println!();
        // }
    }

    result.iter().flatten().filter(|x| **x).count()
}

pub fn p2(data: String) -> usize {
    let mut result: Vec<Vec<bool>> = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let mut translations = Map::default();
    for line in data.lines() {
        let (pat, trans) = line.split_once(" => ").unwrap();
        // w a s t e f u l =(
        let pat = pat.parse::<Pattern>().unwrap();
        let trans = trans.parse::<Pattern>().unwrap();
        let flipped = pat.mirror();
        translations.insert(pat.clone(), trans.clone());
        translations.insert(flipped.clone(), trans.clone());

        // pre-rotate and flip all patterns so it doesn't need to be done later
        let mut pat = pat;
        let mut flipped = flipped;
        for _ in 0..4 {
            pat = pat.rotate();
            flipped = flipped.rotate();
            translations.insert(pat.clone(), trans.clone());
            translations.insert(flipped.clone(), trans.clone());
        }
    }

    for _i in 0..18 {
        enhance(&mut result, &translations);
    }

    result.iter().flatten().filter(|x| **x).count()
}

#[test]
fn test_d21() {
    let data = get_data(21);
    assert_eq!(p1(data.clone()), 197);
    assert_eq!(p2(data), 3081737);
}
