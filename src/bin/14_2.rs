use std::{collections::HashMap, io::stdin};

#[derive(PartialEq, Eq, Hash, Clone)]
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

    fn tilt_west(&mut self) {
        loop {
            let mut changed = false;
            for r in 0..self.0.len() {
                for c in 1..self.0[r].len() {
                    if let (None, Some(Rock::Round)) = (&self.0[r][c - 1], &self.0[r][c]) {
                        self.0[r][c - 1] = Some(Rock::Round);
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

    fn tilt_south(&mut self) {
        loop {
            let mut changed = false;
            for r in (0..self.0.len() - 1).rev() {
                for c in 0..self.0[r].len() {
                    if let (None, Some(Rock::Round)) = (&self.0[r + 1][c], &self.0[r][c]) {
                        self.0[r + 1][c] = Some(Rock::Round);
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

    fn tilt_east(&mut self) {
        loop {
            let mut changed = false;
            for r in 0..self.0.len() {
                for c in (0..self.0[r].len() - 1).rev() {
                    if let (None, Some(Rock::Round)) = (&self.0[r][c + 1], &self.0[r][c]) {
                        self.0[r][c + 1] = Some(Rock::Round);
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

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn north_load(&self) -> usize {
        let height = self.0.len();
        self.0
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.into_iter()
                    .filter(|cell| matches!(cell, Some(Rock::Round)))
                    .count()
                    * (height - i)
            })
            .sum()
    }

    fn solve(&mut self) -> usize {
        let mut seen: HashMap<Vec<Vec<Option<Rock>>>, usize> = HashMap::new();
        loop {
            if let Some(j) = seen.get(&self.0) {
                let period = seen.len() - j;
                let final_offset = (1000000000 - seen.len()) % period;
                for _ in 0..final_offset {
                    self.spin_cycle();
                }
                return self.north_load();
            } else {
                seen.insert(self.0.clone(), seen.len());
                self.spin_cycle();
            }
        }
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
