use std::io::stdin;

enum Rock {
    Round,
    Cube,
}

struct Input(Vec<Vec<Option<Rock>>>);

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(|line| {
                    line.unwrap()
                        .chars()
                        .map(|c| match c {
                            'O' => Some(Rock::Round),
                            '#' => Some(Rock::Cube),
                            _ => None,
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn tilt_north(&mut self) {
        loop {
            let mut changed = false;
            for r in 1..self.0.len() {
                for c in 0..self.0[r].len() {
                    if let (None, Some(Rock::Round)) = (&self.0[r - 1][c], &self.0[r][c]) {
                        self.0[r - 1][c] = Some(Rock::Round);
                        self.0[r][c] = None;
                        changed = true;
                    }
                }
            }
            if !changed {
                return;
            }
        }
    }

    fn solve(mut self) -> usize {
        self.tilt_north();
        let height = self.0.len();
        self.0
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.into_iter()
                    .filter(|cell| matches!(cell, Some(Rock::Round)))
                    .count()
                    * (height - i)
            })
            .sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
