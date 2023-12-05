use std::{io::stdin, iter::once, str::FromStr};

struct RangeMap {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl FromStr for RangeMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s
            .split_ascii_whitespace()
            .map(|n| usize::from_str(n).unwrap());
        let destination_range_start = numbers.next().unwrap();
        let source_range_start = numbers.next().unwrap();
        let range_length = numbers.next().unwrap();
        Ok(RangeMap {
            destination_range_start,
            source_range_start,
            range_length,
        })
    }
}

impl RangeMap {
    fn apply(&self, n: usize) -> Option<usize> {
        if n < self.source_range_start {
            return None;
        }
        let delta = n - self.source_range_start;
        if delta < self.range_length {
            Some(self.destination_range_start + delta)
        } else {
            None
        }
    }
}

struct Map(Vec<RangeMap>);

impl Map {
    fn apply(&self, n: usize) -> usize {
        self.0
            .iter()
            .find_map(|range_map| range_map.apply(n))
            .unwrap_or(n)
    }
}

struct Maps(Vec<Map>);

impl Maps {
    fn apply(&self, n: usize) -> usize {
        self.0.iter().fold(n, |acc, map| map.apply(acc))
    }
}

struct SeedRange {
    start: usize,
    length: usize,
}

impl SeedRange {
    fn iter(&self) -> impl Iterator<Item = usize> {
        self.start..self.start + self.length
    }
}

struct Seeds(Vec<SeedRange>);

impl FromStr for Seeds {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .map(|seed| usize::from_str(seed).unwrap());
        let mut seeds = vec![];
        while let Some(start) = numbers.next() {
            let length = numbers.next().unwrap();
            seeds.push(SeedRange { start, length })
        }
        Ok(Seeds(seeds))
    }
}

impl Seeds {
    fn iter<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.0.iter().flat_map(|seed_range| {
            println!("seed range {} {}", seed_range.start, seed_range.length);
            seed_range.iter()
        })
    }
}

struct Input {
    seeds: Seeds,
    maps: Maps,
}

impl Input {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(|res| res.unwrap());
        let seeds = lines.next().unwrap().parse().unwrap();

        let mut maps: Maps = Maps(vec![]);
        let mut current_map: Map = Map(vec![]);
        for line in lines.chain(once(String::new())) {
            if line.is_empty() || line.ends_with(":") {
                maps.0.push(current_map);
                current_map = Map(vec![])
            } else {
                current_map.0.push(line.parse().unwrap())
            }
        }
        Self { seeds, maps }
    }

    fn solve(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.maps.apply(seed))
            .min()
            .unwrap()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
