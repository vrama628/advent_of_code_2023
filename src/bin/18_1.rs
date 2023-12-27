use itertools::Itertools;
use std::{collections::HashSet, io::stdin, str::FromStr};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.strip_prefix("(#").unwrap().strip_suffix(")").unwrap();
        let red = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let green = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let blue = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Ok(Self { red, green, blue })
    }
}

struct Dig {
    direction: Direction,
    distance: i64,
    color: Color,
}

impl FromStr for Dig {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let direction = parts.next().unwrap().parse().unwrap();
        let distance = parts.next().unwrap().parse().unwrap();
        let color = parts.next().unwrap().parse().unwrap();
        Ok(Self {
            direction,
            distance,
            color,
        })
    }
}

/** rows of boundaries */
struct Lagoon(HashSet<(i64, i64)>);

impl Lagoon {
    fn dig(digs: &[Dig]) -> Self {
        let mut position = (0, 0);
        let mut boundaries = HashSet::from([position]);
        for dig in digs {
            match dig.direction {
                Direction::Up => {
                    for _ in 0..dig.distance {
                        position.0 -= 1;
                        boundaries.insert(position);
                    }
                }
                Direction::Down => {
                    for _ in 0..dig.distance {
                        position.0 += 1;
                        boundaries.insert(position);
                    }
                }
                Direction::Left => {
                    for _ in 0..dig.distance {
                        position.1 -= 1;
                        boundaries.insert(position);
                    }
                }
                Direction::Right => {
                    for _ in 0..dig.distance {
                        position.1 += 1;
                        boundaries.insert(position);
                    }
                }
            }
        }
        Self(boundaries)
    }

    fn solve(&self) -> usize {
        let min_r = self.0.iter().map(|(r, _)| r).min().unwrap() - 1;
        let max_r = self.0.iter().map(|(r, _)| r).max().unwrap() + 2;
        let min_c = self.0.iter().map(|(_, c)| c).min().unwrap() - 1;
        let max_c = self.0.iter().map(|(_, c)| c).max().unwrap() + 2;

        let mut seen = HashSet::from([(min_r, min_c)]);
        let mut stack = vec![(min_r, min_c)];
        while let Some(node) = stack.pop() {
            for neighbor in [
                (node.0 - 1, node.1),
                (node.0 + 1, node.1),
                (node.0, node.1 - 1),
                (node.0, node.1 + 1),
            ]
            .into_iter()
            .filter(|neighbor| {
                min_r <= neighbor.0
                    && neighbor.0 < max_r
                    && min_c <= neighbor.1
                    && neighbor.1 < max_c
                    && !seen.contains(neighbor)
                    && !self.0.contains(neighbor)
            })
            .collect_vec()
            {
                seen.insert(neighbor);
                stack.push(neighbor);
            }
        }
        ((max_r - min_r) * (max_c - min_c)) as usize - seen.len()
    }
}

struct Input(Vec<Dig>);

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
        Lagoon::dig(&self.0).solve()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
