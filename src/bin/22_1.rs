use std::{collections::HashMap, io::stdin, str::FromStr};

use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(|coord| coord.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { x, y, z })
    }
}

impl Coordinate {
    fn range(a: Self, b: Self) -> Vec<Self> {
        if a.x != b.x {
            (a.x..=b.x).map(|x| Self { x, ..a }).collect()
        } else if a.y != b.y {
            (a.y..=b.y).map(|y| Self { y, ..a }).collect()
        } else {
            (a.z..=b.z).map(|z| Self { z, ..a }).collect()
        }
    }

    fn fall(&self) -> Option<Self> {
        (self.z > 0).then(|| Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Brick(usize);

#[derive(Clone)]
struct Bricks {
    coordinate_to_brick: HashMap<Coordinate, Brick>,
    brick_to_coordinate: HashMap<Brick, Vec<Coordinate>>,
}

impl Bricks {
    fn new() -> Self {
        Self {
            coordinate_to_brick: HashMap::new(),
            brick_to_coordinate: HashMap::new(),
        }
    }

    fn add(&mut self, brick: Brick, coordinates: Vec<Coordinate>) {
        for coordinate in &coordinates {
            self.coordinate_to_brick
                .insert(coordinate.clone(), brick.clone());
        }
        self.brick_to_coordinate.insert(brick, coordinates);
    }

    fn remove(&mut self, brick: &Brick) -> Vec<Coordinate> {
        let coordinates = self.brick_to_coordinate.remove(brick).unwrap();
        for coordinate in &coordinates {
            self.coordinate_to_brick.remove(coordinate);
        }
        coordinates
    }

    fn can_fall(&self) -> Option<Brick> {
        self.brick_to_coordinate
            .iter()
            .find(|(brick, coordinates)| {
                coordinates.iter().all(|coordinate| {
                    coordinate.fall().is_some_and(|fallen| {
                        let fallen_location = self.coordinate_to_brick.get(&fallen);
                        fallen_location.is_none() || fallen_location == Some(brick)
                    })
                })
            })
            .map(|(brick, _)| brick.clone())
    }

    fn fall(&mut self) {
        while let Some(brick) = self.can_fall() {
            let coordinates = self.remove(&brick);
            self.add(
                brick,
                coordinates
                    .iter()
                    .map(|coordinate| coordinate.fall().unwrap())
                    .collect(),
            );
        }
    }
}

struct Input(Bricks);

impl Input {
    fn parse() -> Self {
        let mut bricks = Bricks::new();
        for (i, line) in stdin().lines().map(|line| line.unwrap()).enumerate() {
            let (a, b) = line.split_once('~').unwrap();
            bricks.add(
                Brick(i),
                Coordinate::range(a.parse().unwrap(), b.parse().unwrap()),
            );
        }
        Self(bricks)
    }

    fn solve(&mut self) -> usize {
        self.0.fall();
        self.0
            .brick_to_coordinate
            .keys()
            .filter(|brick| {
                let mut clone = self.0.clone();
                clone.remove(brick);
                clone.can_fall().is_none()
            })
            .count()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
