#![allow(unused_imports)]

use std::str::FromStr;

use itertools::Itertools;

use crate::get_data;

#[derive(Debug, Clone, Default, Copy, PartialOrd, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Debug, Clone, Default)]
struct Particle {
    pos: Pos,
    vel: Pos,
    acc: Pos,
}

impl Particle {
    fn dist(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn tick(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

impl FromStr for Particle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace();
        let mut particle: Self = Self::default();

        for (p, d) in std::iter::zip(
            parts,
            [&mut particle.pos, &mut particle.vel, &mut particle.acc],
        ) {
            // lmao rust is so sick
            let mut nums = p
                .strip_prefix(|x| matches!(x, 'p' | 'v' | 'a'))
                .unwrap()
                .strip_prefix("=<")
                .map(|x| {
                    x.strip_suffix(">,")
                        .unwrap_or_else(|| x.strip_suffix(">").unwrap())
                })
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap());

            d.x = nums.next().unwrap();
            d.y = nums.next().unwrap();
            d.z = nums.next().unwrap();
        }

        Ok(particle)
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.dist() == other.dist()
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist().partial_cmp(&other.dist())
    }
}

pub fn p1(data: String) -> usize {
    /* closest to center in the long term is the one with
        1. the least acceleration
        2. if accel is tied, the least starting velocity
        3. if starting velocity is tied, the closest starting position

        this isn't strictly true in edge cases if the acceleration is in the opposite distance of
        the starting velocity/position, but here's to hoping my input doesn't have that particular
        quality cuz it'd make it a lot more complicated (or i'd just have to run the simulation)

        edit: lets goooooooo
    */
    let particles = data
        .lines()
        .map(|x| x.parse::<Particle>().unwrap())
        .collect::<Vec<_>>();

    let smallest_acc = particles
        .iter()
        .enumerate()
        .min_set_by(|a, b| a.1.acc.dist().cmp(&b.1.acc.dist()));
    let smallest_vel = smallest_acc
        .iter()
        .min_set_by(|a, b| a.1.vel.dist().cmp(&b.1.vel.dist()));
    let smallest = smallest_vel
        .iter()
        .min_by(|a, b| a.1.pos.dist().cmp(&b.1.pos.dist()))
        .unwrap();

    smallest.0
}

pub fn p2(data: String) -> usize {
    let mut particles = data
        .lines()
        .map(|x| x.parse::<Particle>().ok())
        .collect::<Vec<_>>();

    let mut collisions = Vec::new();

    // hopefully 1000 runs is enough? I could do raycasting but I don't remember that algorithm off
    // the top of my head
    for _ in 0..1000 {
        for part in &mut particles {
            if let Some(p) = part.as_mut() {
                p.tick();
            }
        }

        for i in 0..particles.len() - 1 {
            if particles[i].as_ref().is_some_and(|x| particles[i + 1].as_ref().is_some_and(|y| x.pos == y.pos)) {
                collisions.push(i);
                collisions.push(i + 1);
            }
        }

        for c in &collisions {
            particles[*c] = None;
        }
        collisions.clear()
    }

    particles.iter().flatten().count()
}

#[test]
fn test_d20() {
    let data = get_data(20);
    assert_eq!(p1(data.clone()), 457);
    assert_eq!(p2(data), 448);
}
