use itertools::Itertools;
use std::{io::stdin, str::FromStr};

struct History(Vec<i64>);

impl FromStr for History {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_whitespace().map(|s| s.parse().unwrap()).collect(),
        ))
    }
}

impl History {
    fn derivative(&self) -> Self {
        Self(self.0.iter().tuple_windows().map(|(a, b)| b - a).collect())
    }

    fn prev(&self) -> i64 {
        if self.0.iter().all(|v| *v == 0) {
            0
        } else {
            *self.0.first().unwrap() - self.derivative().prev()
        }
    }
}

struct Input(Vec<History>);

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
        self.0.iter().map(|history| history.prev()).sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
