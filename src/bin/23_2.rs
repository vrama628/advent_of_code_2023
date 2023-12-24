use std::io::stdin;

use im::HashSet;
use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq)]
enum MapCell {
    Path,
    Forest,
    Slope(Direction),
}

type Map = Vec<Vec<MapCell>>;
type Pos = (usize, usize);

struct Input {
    map: Map,
    start: Pos,
    end: Pos,
}

impl Input {
    fn parse() -> Self {
        let map: Map = stdin()
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|c| match c {
                        '.' => MapCell::Path,
                        '#' => MapCell::Forest,
                        '<' => MapCell::Slope(Direction::Left),
                        '>' => MapCell::Slope(Direction::Right),
                        '^' => MapCell::Slope(Direction::Up),
                        'v' => MapCell::Slope(Direction::Down),
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect();
        let start = (
            0,
            map[0]
                .iter()
                .find_position(|c| matches!(c, MapCell::Path))
                .unwrap()
                .0,
        );
        let end = (
            map.len() - 1,
            map[map.len() - 1]
                .iter()
                .find_position(|c| matches!(c, MapCell::Path))
                .unwrap()
                .0,
        );
        Self { map, start, end }
    }

    fn neighbors(&self, pos: Pos, seen: &HashSet<Pos>) -> Vec<Pos> {
        let mut neighbors = vec![];
        if pos.0 > 0 {
            neighbors.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.map.len() - 1 {
            neighbors.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            neighbors.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.map[0].len() - 1 {
            neighbors.push((pos.0, pos.1 + 1));
        }
        neighbors.retain(|neighbor| {
            !seen.contains(neighbor) && self.map[neighbor.0][neighbor.1] != MapCell::Forest
        });
        neighbors
    }

    fn solve_from(&self, pos: Pos, seen: &HashSet<Pos>) -> Option<usize> {
        if pos == self.end {
            Some(seen.len())
        } else {
            let seen = seen.update(pos);
            self.neighbors(pos, &seen)
                .into_iter()
                .filter_map(|neighbor| self.solve_from(neighbor, &seen))
                .max()
        }
    }

    fn solve(&self) -> usize {
        self.solve_from(self.start, &HashSet::new()).unwrap()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
