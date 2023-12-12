use itertools::Itertools;
use std::io::stdin;

struct Input(Vec<Vec<bool>>);

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(|line| {
                    line.unwrap()
                        .chars()
                        .map(|cell| cell == '#')
                        .collect::<Vec<bool>>()
                })
                .collect(),
        )
    }

    fn expanded_rows(&self) -> Vec<usize> {
        (0..self.0.len())
            .filter(|r| self.0[*r].iter().all(|cell| *cell == false))
            .collect()
    }

    fn expanded_cols(&self) -> Vec<usize> {
        (0..self.0[0].len())
            .filter(|c| self.0.iter().all(|row| row[*c] == false))
            .collect()
    }

    fn galaxies(&self) -> Vec<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().filter_map(
                    move |(c, cell)| {
                        if *cell {
                            Some((r, c))
                        } else {
                            None
                        }
                    },
                )
            })
            .collect()
    }

    fn solve(&self) -> usize {
        let expanded_rows = self.expanded_rows();
        let expanded_cols = self.expanded_cols();
        self.galaxies()
            .into_iter()
            .tuple_combinations()
            .map(|((r1, c1), (r2, c2))| {
                let (r1, r2) = if r1 < r2 { (r1, r2) } else { (r2, r1) };
                let (c1, c2) = if c1 < c2 { (c1, c2) } else { (c2, c1) };
                let relevant_expanded_rows = expanded_rows
                    .iter()
                    .filter(|r| r1 <= **r && **r <= r2)
                    .count();
                let relevant_expanded_cols = expanded_cols
                    .iter()
                    .filter(|c| c1 <= **c && **c <= c2)
                    .count();
                r2 - r1 + c2 - c1
                    + relevant_expanded_rows * 999999
                    + relevant_expanded_cols * 999999
            })
            .sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
