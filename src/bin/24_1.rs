use std::{io::stdin, str::FromStr};

use itertools::Itertools;
use num_rational::BigRational;

struct Dimensions {
    x: BigRational,
    y: BigRational,
}

impl FromStr for Dimensions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, _) = s
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { x, y })
    }
}

impl Dimensions {
    fn is_in_test_area(&self) -> bool {
        BigRational::from_integer(200000000000000i64.into()) <= self.x
            && self.x <= BigRational::from_integer(400000000000000i64.into())
            && BigRational::from_integer(200000000000000i64.into()) <= self.y
            && self.y <= BigRational::from_integer(400000000000000i64.into())
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

impl Hailstone {
    fn intersection(a: &Hailstone, b: &Hailstone) -> Option<Dimensions> {
        let numerator = &a.p.y - &b.p.y + (&b.v.y * &b.p.x / &b.v.x) - (&a.v.y * &a.p.x / &a.v.x);
        let denominator = &b.v.y / &b.v.x - &a.v.y / &a.v.x;
        if denominator == BigRational::from_integer(0.into()) {
            None
        } else {
            let x = numerator / denominator;
            let y = &a.p.y + &a.v.y * ((&x - &a.p.x) / &a.v.x);
            Some(Dimensions { x, y })
        }
    }

    fn is_future(&self, point: &Dimensions) -> bool {
        (&point.x - &self.p.x) * &self.v.x > BigRational::from_integer(0.into())
            && (&point.y - &self.p.y) * &self.v.y > BigRational::from_integer(0.into())
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

    fn solve(&self) -> usize {
        self.0
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                Hailstone::intersection(a, b).is_some_and(|point| {
                    a.is_future(&point) && b.is_future(&point) && point.is_in_test_area()
                })
            })
            .count()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
