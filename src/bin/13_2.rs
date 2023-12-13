use std::io::stdin;

enum Reflection {
    Row(usize),
    Col(usize),
}

impl Reflection {
    fn value(&self) -> usize {
        match self {
            Reflection::Row(r) => 100 * *r,
            Reflection::Col(c) => *c,
        }
    }
}

struct Pattern(Vec<Vec<bool>>);

impl Pattern {
    fn solve(&self) -> Reflection {
        if let Some(row) = (1..self.0.len()).find(|r| {
            (0..*r)
                .map(|i| {
                    let rev_i = 2 * r - i - 1;
                    if rev_i < self.0.len() {
                        self.0[i]
                            .iter()
                            .zip(self.0[rev_i].iter())
                            .filter(|(a, b)| a != b)
                            .count()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
                == 1
        }) {
            Reflection::Row(row)
        } else if let Some(col) = (1..self.0[0].len()).find(|c| {
            (0..*c)
                .map(|i| {
                    let rev_i = 2 * c - i - 1;
                    if rev_i < self.0[0].len() {
                        self.0.iter().filter(|row| row[i] != row[rev_i]).count()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
                == 1
        }) {
            Reflection::Col(col)
        } else {
            panic!("No reflection found")
        }
    }
}

struct Input(Vec<Pattern>);

impl Input {
    fn parse() -> Self {
        let mut patterns: Vec<Pattern> = vec![];
        let mut current_pattern: Vec<Vec<bool>> = vec![];
        for line in stdin().lines().map(|line| line.unwrap()) {
            if line.is_empty() {
                patterns.push(Pattern(current_pattern));
                current_pattern = vec![];
            } else {
                current_pattern.push(
                    line.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!("Invalid character"),
                        })
                        .collect(),
                );
            }
        }
        patterns.push(Pattern(current_pattern));
        Input(patterns)
    }

    fn solve(&self) -> usize {
        self.0.iter().map(|pattern| pattern.solve().value()).sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
