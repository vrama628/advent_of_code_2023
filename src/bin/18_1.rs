use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    io::stdin,
    str::FromStr,
};

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

struct Boundary {
    column: i64,
    width: i64,
}

impl Ord for Boundary {
    fn cmp(&self, other: &Self) -> Ordering {
        self.column.cmp(&other.column)
    }
}

impl PartialOrd for Boundary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Boundary {
    fn eq(&self, other: &Self) -> bool {
        self.column == other.column
    }
}

impl Eq for Boundary {}

/** rows of boundaries */
struct Lagoon(BTreeMap<i64, BTreeSet<Boundary>>);

impl Lagoon {
    fn add_boundary(&mut self, (row, column): (i64, i64), width: i64) {
        self.0
            .entry(row)
            .or_default()
            .insert(Boundary { column, width });
    }

    fn dig(digs: &[Dig]) -> Self {
        let mut position = (0, 0);
        let mut this = Self(BTreeMap::new());
        // up/down is exclusive on both ends; left/right is inclusive on both ends
        // assumes the digs alternate between up/down and left/right
        for dig in digs {
            match dig.direction {
                Direction::Up => {
                    for r in position.0 - dig.distance + 1..position.0 {
                        this.add_boundary((r, position.1), 1);
                    }
                    position.0 -= dig.distance;
                }
                Direction::Down => {
                    for r in position.0 + 1..position.0 + dig.distance {
                        this.add_boundary((r, position.1), 1);
                    }
                    position.0 += dig.distance;
                }
                Direction::Left => {
                    position.1 -= dig.distance;
                    this.add_boundary(position, dig.distance + 1);
                }
                Direction::Right => {
                    this.add_boundary(position, dig.distance + 1);
                    position.1 += dig.distance;
                }
            }
        }
        this
    }

    fn solve(&self) -> i64 {
        self.0
            .values()
            .map(|row| {
                println!("ROW");
                let res =
                    row.iter()
                        .inspect(|Boundary { column, width }| println!("{column} {width}"))
                        .fold((0, None), |(sum, interior), Boundary { column, width }| {
                            match interior {
                                Some(interior) => (sum + width + column - interior, None),
                                None => (sum + width, Some(column + width)),
                            }
                        })
                        .0;
                println!("--> {res}");
                res
            })
            .sum()
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

    fn solve(&self) -> i64 {
        Lagoon::dig(&self.0).solve()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
