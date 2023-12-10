use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

#[derive(PartialEq, Eq)]
enum Tile {
    Start,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'S' => Some(Self::Start),
            '|' => Some(Self::NorthSouth),
            '-' => Some(Self::EastWest),
            'L' => Some(Self::NorthEast),
            'J' => Some(Self::NorthWest),
            '7' => Some(Self::SouthWest),
            'F' => Some(Self::SouthEast),
            _ => None,
        }
    }

    fn adjacent(&self, (r, c): (usize, usize)) -> [(usize, usize); 2] {
        match self {
            Tile::Start => unreachable!(),
            Tile::NorthSouth => [(r - 1, c), (r + 1, c)],
            Tile::EastWest => [(r, c + 1), (r, c - 1)],
            Tile::NorthEast => [(r - 1, c), (r, c + 1)],
            Tile::NorthWest => [(r - 1, c), (r, c - 1)],
            Tile::SouthWest => [(r + 1, c), (r, c - 1)],
            Tile::SouthEast => [(r + 1, c), (r, c + 1)],
        }
    }
}

struct Input(HashMap<(usize, usize), Tile>);

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .enumerate()
                .flat_map(|(r, line)| {
                    line.unwrap()
                        .char_indices()
                        .filter_map(|(c, tile_c)| {
                            Tile::from_char(tile_c).map(|tile| ((r, c), tile))
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        )
    }

    fn start(&self) -> (usize, usize) {
        self.0
            .iter()
            .find_map(|(pos, tile)| {
                if tile == &Tile::Start {
                    Some(pos)
                } else {
                    None
                }
            })
            .unwrap()
            .clone()
    }

    fn next_from_start(&self) -> (usize, usize) {
        let (r, c) = self.start();
        let mut neighbors = vec![(r, c + 1), (r + 1, c)];
        if r > 0 {
            neighbors.push((r - 1, c));
        }
        if c > 0 {
            neighbors.push((r, c - 1));
        }
        neighbors
            .into_iter()
            .find(|pos| self.0.get(pos).unwrap().adjacent(*pos).contains(&(r, c)))
            .unwrap()
    }

    fn length(&self) -> usize {
        let mut pos = self.start();
        let mut seen = HashSet::from([(pos)]);
        pos = self.next_from_start();
        seen.insert(pos);
        loop {
            if let Some(next) = self
                .0
                .get(&pos)
                .unwrap()
                .adjacent(pos)
                .into_iter()
                .filter(|pos| !seen.contains(pos))
                .next()
            {
                pos = next;
                seen.insert(pos);
            } else {
                return seen.len();
            }
        }
    }

    fn solve(&self) -> usize {
        self.length() / 2
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
