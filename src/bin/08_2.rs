use std::{
    collections::{BTreeSet, HashMap},
    io::stdin,
    str::FromStr,
};

use itertools::Itertools;
use num::integer::lcm;

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

#[derive(Debug, PartialEq, Eq)]
struct Cycle {
    start: usize,
    hits_before_cycle: BTreeSet<usize>,
    period: usize,
    hits_in_cycle: BTreeSet<usize>,
}

impl Cycle {
    fn increase_start(self, n: usize) -> Self {
        let start = self.start + n;
        let mut new_hits_before_cycle = self.hits_in_cycle;
        let mut hits_in_cycle = new_hits_before_cycle.split_off(&n);
        hits_in_cycle.extend(new_hits_before_cycle.iter().map(|hit| hit + self.period));
        let hits_in_cycle = hits_in_cycle.into_iter().map(|hit| hit - n).collect();
        let mut hits_before_cycle = self.hits_before_cycle;
        hits_before_cycle.extend(new_hits_before_cycle.iter().map(|hit| hit + self.start));
        Self {
            start,
            hits_before_cycle,
            period: self.period,
            hits_in_cycle,
        }
    }

    fn multiply_cycle(self, n: usize) -> Self {
        let period = self.period * n;
        let hits_in_cycle = (0..n)
            .flat_map(|i| {
                self.hits_in_cycle
                    .iter()
                    .map(|hit| hit + i * self.period)
                    .collect_vec()
            })
            .collect();
        Self {
            start: self.start,
            hits_before_cycle: self.hits_before_cycle,
            period,
            hits_in_cycle,
        }
    }

    fn first(&self) -> usize {
        self.hits_before_cycle
            .first()
            .cloned()
            .unwrap_or_else(|| self.hits_in_cycle.first().unwrap() + self.start)
    }

    fn intersect(a: Self, b: Self) -> Self {
        let (a_start, b_start, a_period, b_period) = (a.start, b.start, a.period, b.period);
        let start = a.start.max(b.start);
        let (a, b) = (
            a.increase_start(start - a_start),
            b.increase_start(start - b_start),
        );
        let period = lcm(a.period, b.period);
        let (a, b) = (
            a.multiply_cycle(period / a_period),
            b.multiply_cycle(period / b_period),
        );
        let hits_before_cycle = a
            .hits_before_cycle
            .intersection(&b.hits_before_cycle)
            .cloned()
            .collect();
        let hits_in_cycle = a
            .hits_in_cycle
            .intersection(&b.hits_in_cycle)
            .cloned()
            .collect();
        Self {
            start,
            hits_before_cycle,
            period,
            hits_in_cycle,
        }
    }
}

#[test]
fn test_increase_start() {
    let cycle = Cycle {
        start: 3,
        hits_before_cycle: vec![0, 2].into_iter().collect(),
        period: 5,
        hits_in_cycle: vec![0, 2, 4].into_iter().collect(),
    };
    assert_eq!(
        cycle.increase_start(1),
        Cycle {
            start: 4,
            hits_before_cycle: vec![0, 2, 3].into_iter().collect(),
            period: 5,
            hits_in_cycle: vec![1, 3, 4].into_iter().collect(),
        }
    );
}

#[test]
fn test_multiply_cycle() {
    let cycle = Cycle {
        start: 3,
        hits_before_cycle: vec![0, 2].into_iter().collect(),
        period: 5,
        hits_in_cycle: vec![0, 2, 4].into_iter().collect(),
    };
    assert_eq!(
        cycle.multiply_cycle(2),
        Cycle {
            start: 3,
            hits_before_cycle: vec![0, 2].into_iter().collect(),
            period: 10,
            hits_in_cycle: vec![0, 2, 4, 5, 7, 9].into_iter().collect(),
        }
    );
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

    fn cycle_from(&self, start: Node) -> Cycle {
        let mut seen: HashMap<(usize, String), usize> = HashMap::new();
        let mut hits = BTreeSet::new();
        let mut node = start;
        for (overall_i, (direction_i, direction)) in
            self.directions.0.iter().enumerate().cycle().enumerate()
        {
            if let Some(last_seen) = seen.insert((direction_i, node.clone()), overall_i) {
                let hits_in_cycle = hits
                    .split_off(&last_seen)
                    .into_iter()
                    .map(|hit| hit - last_seen)
                    .collect();
                return Cycle {
                    start: last_seen,
                    hits_before_cycle: hits,
                    period: overall_i - last_seen,
                    hits_in_cycle,
                };
            }
            if node.ends_with('Z') {
                hits.insert(overall_i);
            }
            node = match direction {
                Direction::Left => self.graph.get(&node).unwrap().left.clone(),
                Direction::Right => self.graph.get(&node).unwrap().right.clone(),
            }
        }
        unreachable!()
    }

    fn solve(&self) -> usize {
        self.graph
            .keys()
            .filter(|node| node.ends_with("A"))
            .map(|node| self.cycle_from(node.clone()))
            .reduce(|a, b| Cycle::intersect(a, b))
            .unwrap()
            .first()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
