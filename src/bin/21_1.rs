use std::{collections::HashSet, io::stdin};

struct Map(Vec<Vec<bool>>);

impl Map {
    fn step(&self, (r, c): (usize, usize)) -> Vec<(usize, usize)> {
        let mut adj = vec![];
        if r > 0 {
            adj.push((r - 1, c));
        }
        if c > 0 {
            adj.push((r, c - 1));
        }
        if r < self.0.len() - 1 {
            adj.push((r + 1, c));
        }
        if c < self.0[0].len() - 1 {
            adj.push((r, c + 1));
        }
        adj.retain(|(r, c)| self.0[*r][*c]);
        adj
    }

    fn step_many(&self, locations: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        locations
            .into_iter()
            .flat_map(|loc| self.step(loc))
            .collect()
    }
}

struct Input {
    map: Map,
    start: (usize, usize),
}

impl Input {
    fn parse() -> Self {
        let mut start = (0, 0);
        let map = Map(stdin()
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.unwrap()
                    .char_indices()
                    .map(|(c, ch)| match ch {
                        'S' => {
                            start = (r, c);
                            true
                        }
                        '.' => true,
                        '#' => false,
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect());
        Self { map, start }
    }

    fn solve(&self) -> usize {
        let mut locations = HashSet::new();
        locations.insert(self.start);
        for _ in 0..64 {
            locations = self.map.step_many(locations);
        }
        locations.len()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
