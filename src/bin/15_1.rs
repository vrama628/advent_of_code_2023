use std::io::stdin;

struct Step(String);

impl Step {
    fn hash(&self) -> usize {
        self.0
            .chars()
            .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
    }
}

struct Input(Vec<Step>);

impl Input {
    fn parse() -> Self {
        let line = stdin().lines().next().unwrap().unwrap();
        Self(line.split(',').map(|s| Step(s.to_owned())).collect())
    }

    fn solve(&self) -> usize {
        self.0.iter().map(|step| step.hash()).sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
