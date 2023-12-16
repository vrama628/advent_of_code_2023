use std::{io::stdin, str::FromStr};

struct Label(String);

impl Label {
    fn hash(&self) -> usize {
        self.0
            .chars()
            .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
    }
}

enum Operation {
    Remove,
    Upsert(usize),
}

struct Step {
    label: Label,
    operation: Operation,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, focal_length) = s.split_once(['=', '-']).unwrap();
        let label = Label(label.to_owned());
        Ok(Self {
            label,
            operation: if focal_length.is_empty() {
                Operation::Remove
            } else {
                Operation::Upsert(focal_length.parse().unwrap())
            },
        })
    }
}

struct Box(Vec<(String, usize)>);

impl Box {
    fn remove(&mut self, label: &str) {
        self.0.retain(|(l, _)| l != label);
    }

    fn upsert(&mut self, label: &str, focal_length: usize) {
        if let Some((i, _)) = self.0.iter().enumerate().find(|(_, (l, _))| l == label) {
            self.0[i] = (label.to_owned(), focal_length);
        } else {
            self.0.push((label.to_owned(), focal_length));
        }
    }
}

struct Input(Vec<Step>);

impl Input {
    fn parse() -> Self {
        let line = stdin().lines().next().unwrap().unwrap();
        Self(line.split(',').map(|s| s.parse().unwrap()).collect())
    }

    fn solve(self) -> usize {
        let mut boxes: Vec<Box> = (0..256).map(|_| Box(vec![])).collect();
        for step in self.0 {
            let box_i = step.label.hash();
            match step {
                Step {
                    label,
                    operation: Operation::Remove,
                } => {
                    boxes[box_i].remove(&label.0);
                }
                Step {
                    label,
                    operation: Operation::Upsert(focal_length),
                } => {
                    boxes[box_i].upsert(&label.0, focal_length);
                }
            }
        }
        boxes
            .into_iter()
            .enumerate()
            .flat_map(|(box_i, box_)| {
                box_.0
                    .into_iter()
                    .enumerate()
                    .map(|(lens_i, (_, focal_length))| (box_i + 1) * (lens_i + 1) * focal_length)
                    .collect::<Vec<_>>()
            })
            .sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
