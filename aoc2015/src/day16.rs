#![allow(unused_imports)]

use crate::get_data;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Sue {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

impl Sue {
    const CANON_SUE: Sue = Sue {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    fn could_be(&self) -> bool {
        if let Some(c) = self.children {
            if c != Self::CANON_SUE.children.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.cats {
            if c != Self::CANON_SUE.cats.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.samoyeds {
            if c != Self::CANON_SUE.samoyeds.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.pomeranians {
            if c != Self::CANON_SUE.pomeranians.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.akitas {
            if c != Self::CANON_SUE.akitas.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.vizslas {
            if c != Self::CANON_SUE.vizslas.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.goldfish {
            if c != Self::CANON_SUE.goldfish.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.trees {
            if c != Self::CANON_SUE.trees.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.cars {
            if c != Self::CANON_SUE.cars.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.perfumes {
            if c != Self::CANON_SUE.perfumes.unwrap() {
                return false;
            }
        }

        true
    }

    fn could_be_2(&self) -> bool {
        if let Some(c) = self.children {
            if c != Self::CANON_SUE.children.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.cats {
            if c <= Self::CANON_SUE.cats.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.samoyeds {
            if c != Self::CANON_SUE.samoyeds.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.pomeranians {
            if c >= Self::CANON_SUE.pomeranians.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.akitas {
            if c != Self::CANON_SUE.akitas.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.vizslas {
            if c != Self::CANON_SUE.vizslas.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.goldfish {
            if c >= Self::CANON_SUE.goldfish.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.trees {
            if c <= Self::CANON_SUE.trees.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.cars {
            if c != Self::CANON_SUE.cars.unwrap() {
                return false;
            }
        }
        if let Some(c) = self.perfumes {
            if c != Self::CANON_SUE.perfumes.unwrap() {
                return false;
            }
        }

        true
    }
}

pub fn p1(data: String) -> usize {
    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();

        tokens.next();
        let num = tokens
            .next()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut sue = Sue::default();

        while let Some(t) = tokens.next() {
            match t {
                "children:" => {
                    sue.children = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "cats:" => {
                    sue.cats = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "samoyeds:" => {
                    sue.samoyeds = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "pomeranians:" => {
                    sue.pomeranians = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "akitas:" => {
                    sue.akitas = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "vizslas:" => {
                    sue.vizslas = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "goldfish:" => {
                    sue.goldfish = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "trees:" => {
                    sue.trees = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "cars:" => {
                    sue.cars = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "perfumes:" => {
                    sue.perfumes = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                _ => panic!(),
            }
        }

        if sue.could_be() {
            return num;
        }
    }

    0
}

pub fn p2(data: String) -> usize {
    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();

        tokens.next();
        let num = tokens
            .next()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut sue = Sue::default();

        while let Some(t) = tokens.next() {
            match t {
                "children:" => {
                    sue.children = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "cats:" => {
                    sue.cats = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "samoyeds:" => {
                    sue.samoyeds = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "pomeranians:" => {
                    sue.pomeranians = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "akitas:" => {
                    sue.akitas = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "vizslas:" => {
                    sue.vizslas = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "goldfish:" => {
                    sue.goldfish = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "trees:" => {
                    sue.trees = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "cars:" => {
                    sue.cars = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                "perfumes:" => {
                    sue.perfumes = Some(
                        tokens
                            .next()
                            .unwrap()
                            .trim_end_matches(',')
                            .parse::<u8>()
                            .unwrap(),
                    )
                }
                _ => panic!(),
            }
        }

        if sue.could_be_2() {
            return num;
        }
    }

    0
}

#[test]
fn test_d16() {
    let data = get_data(16);
    assert_eq!(p1(data.clone()), 103);
    assert_eq!(p2(data), 405);
}
