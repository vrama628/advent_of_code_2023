use std::{collections::HashMap, io::stdin, str::FromStr};

enum Direction {
    Left,
    Right,
}
struct Directions(Vec<Direction>);

impl FromStr for Directions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Invalid direction"),
                })
                .collect(),
        ))
    }
}

impl Directions {
    fn iter(&self) -> impl Iterator<Item = &Direction> {
        self.0.iter().cycle()
    }
}

type Node = String;
struct Edges {
    left: Node,
    right: Node,
}

impl FromStr for Edges {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();
        Ok(Self {
            left: left.to_owned(),
            right: right.to_owned(),
        })
    }
}

struct Input {
    directions: Directions,
    graph: HashMap<Node, Edges>,
}

impl Input {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(|line| line.unwrap());
        let directions = lines.next().unwrap().parse().unwrap();
        lines.next();
        let graph = lines
            .map(|line| {
                let (node, edges_s) = line.split_once(" = ").unwrap();
                (node.to_owned(), edges_s.parse().unwrap())
            })
            .collect();

        Self { directions, graph }
    }

    fn solve(&self) -> usize {
        let mut node = "AAA";
        for (i, direction) in self.directions.iter().enumerate() {
            if node == "ZZZ" {
                return i;
            }
            let edges = self.graph.get(node).unwrap();
            node = match direction {
                Direction::Left => &edges.left,
                Direction::Right => &edges.right,
            }
        }
        0
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
