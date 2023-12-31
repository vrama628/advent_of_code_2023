use std::{
    io::stdin,
    ops::{Mul, SubAssign},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug)]
struct Dimensions {
    x: f64,
    y: f64,
    z: f64,
}

impl FromStr for Dimensions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { x, y, z })
    }
}

impl SubAssign for Dimensions {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Dimensions {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

struct Hailstone {
    p: Dimensions,
    v: Dimensions,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" @ ").unwrap();
        Ok(Self {
            p: position.parse().unwrap(),
            v: velocity.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Solution {
    p: Dimensions,
    v: Dimensions,
    t: Vec<f64>,
}

impl Solution {
    fn new(hailstones: &[Hailstone]) -> Self {
        Self {
            p: Dimensions {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            v: Dimensions {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            t: hailstones.iter().map(|_| 0.).collect(),
        }
    }

    fn error(&self, hailstones: &[Hailstone]) -> f64 {
        self.t
            .iter()
            .zip(hailstones)
            .map(|(t, hailstone)| {
                ((self.v.x * t + self.p.x) - (hailstone.v.x * t + hailstone.p.x)).powi(2)
                    + ((self.v.y * t + self.p.y) - (hailstone.v.y * t + hailstone.p.y)).powi(2)
                    + ((self.v.z * t + self.p.z) - (hailstone.v.z * t + hailstone.p.z)).powi(2)
            })
            .sum()
    }

    fn gradient(&self, hailstones: &[Hailstone]) -> Self {
        let p = Dimensions {
            x: self
                .t
                .iter()
                .zip(hailstones)
                .map(|(t, hailstone)| {
                    2. * (self.v.x * t + self.p.x - (hailstone.v.x * t + hailstone.p.x))
                })
                .sum(),
            y: self
                .t
                .iter()
                .zip(hailstones)
                .map(|(t, hailstone)| {
                    2. * (self.v.y * t + self.p.y - (hailstone.v.y * t + hailstone.p.y))
                })
                .sum(),
            z: self
                .t
                .iter()
                .zip(hailstones)
                .map(|(t, hailstone)| {
                    2. * (self.v.z * t + self.p.z - (hailstone.v.z * t + hailstone.p.z))
                })
                .sum(),
        };
        let v = Dimensions {
            x: self
                .t
                .iter()
                .zip(hailstones)
                .map(|(t, hailstone)| {
                    2. * (self.v.x * t + self.p.x - (hailstone.v.x * t + hailstone.p.x)) * t
                })
                .sum(),
            y: self
                .t
                .iter()
                .zip(hailstones)
                .map(|(t, hailstone)| {
                    2. * (self.v.y * t + self.p.y - (hailstone.v.y * t + hailstone.p.y)) * t
                })
                .sum(),
            z: self
                .t
                .iter()
                .zip(hailstones)
                .map(|(t, hailstone)| {
                    2. * (self.v.z * t + self.p.z - (hailstone.v.z * t + hailstone.p.z)) * t
                })
                .sum(),
        };
        let t = self
            .t
            .iter()
            .zip(hailstones)
            .map(|(t, hailstone)| {
                2. * (self.v.x * t + self.p.x - (hailstone.v.x * t + hailstone.p.x))
                    * (self.v.x - hailstone.v.x)
                    + 2. * (self.v.y * t + self.p.y - (hailstone.v.y * t + hailstone.p.y))
                        * (self.v.y - hailstone.v.y)
                    + 2. * (self.v.z * t + self.p.z - (hailstone.v.z * t + hailstone.p.z))
                        * (self.v.z - hailstone.v.z)
            })
            .collect();
        Self { p, v, t }
    }
}

impl SubAssign for Solution {
    fn sub_assign(&mut self, rhs: Self) {
        self.p -= rhs.p;
        self.v -= rhs.v;
        self.t.iter_mut().zip(rhs.t).for_each(|(t, rhs)| *t -= rhs);
    }
}

impl Mul<f64> for Solution {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            p: self.p * rhs,
            v: self.v * rhs,
            t: self.t.into_iter().map(|t| t * rhs).collect(),
        }
    }
}

struct Input(Vec<Hailstone>);

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(|line| line.unwrap().parse().unwrap())
                .collect(),
        )
    }

    fn solve(&self) -> f64 {
        let mut solution = Solution::new(&self.0);
        let learning_rate = 0.001;
        loop {
            let error = solution.error(&self.0);
            println!("{error}");
            if error.is_nan() {
                panic!()
            }
            if error < f64::EPSILON {
                println!("{solution:#?}");
                return (solution.p.x + solution.p.y + solution.p.z).round();
            }
            solution -= solution.gradient(&self.0) * learning_rate;
        }
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
