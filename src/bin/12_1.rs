use std::{io::stdin, str::FromStr};

type Spring = Option<bool>;

fn falsable(spring: &Spring) -> bool {
    !spring.unwrap_or(false)
}

fn truable(spring: &Spring) -> bool {
    spring.unwrap_or(true)
}

fn solve(springs: &[Spring], groups: &[usize]) -> usize {
    if groups.is_empty() {
        if springs.iter().all(falsable) {
            1
        } else {
            0
        }
    } else {
        if springs.len() < groups[0] {
            0
        } else if springs.len() == groups[0] {
            if springs.iter().all(truable) && groups.len() == 1 {
                1
            } else {
                0
            }
        } else {
            let true_arrangements =
                if springs[..groups[0]].iter().all(truable) && falsable(&springs[groups[0]]) {
                    solve(&springs[(groups[0] + 1)..], &groups[1..])
                } else {
                    0
                };
            let false_arrangements = if falsable(&springs[0]) {
                solve(&springs[1..], groups)
            } else {
                0
            };
            true_arrangements + false_arrangements
        }
    }
}

struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Row {
    fn solve(&self) -> usize {
        solve(&self.springs, &self.groups)
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs_s, groups_s) = s.split_once(' ').unwrap();
        let springs = springs_s
            .chars()
            .map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                '?' => None,
                _ => panic!(),
            })
            .collect();
        let groups = groups_s
            .split(',')
            .map(|group_s| usize::from_str(group_s).unwrap())
            .collect();
        Ok(Self { springs, groups })
    }
}

struct Input(Vec<Row>);

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
        self.0.iter().map(|row| row.solve()).sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
