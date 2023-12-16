use std::{collections::HashSet, io::stdin};

enum Cell {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterVertical,
    SplitterHorizontal,
}

struct Input(Vec<Vec<Cell>>);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (usize, usize);
type PositionAndDirection = (Position, Direction);

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(|line| {
                    line.unwrap()
                        .chars()
                        .map(|c| match c {
                            '.' => Cell::Empty,
                            '/' => Cell::MirrorForward,
                            '\\' => Cell::MirrorBackward,
                            '|' => Cell::SplitterVertical,
                            '-' => Cell::SplitterHorizontal,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn move_(&self, (r, c): Position, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::Up => (r > 0).then(|| (r - 1, c)),
            Direction::Down => (r < self.0.len() - 1).then(|| (r + 1, c)),
            Direction::Left => (c > 0).then(|| (r, c - 1)),
            Direction::Right => (c < self.0[0].len() - 1).then(|| (r, c + 1)),
        }
    }

    fn adjacent(&self, (pos, dir): PositionAndDirection) -> Vec<PositionAndDirection> {
        match (&self.0[pos.0][pos.1], dir) {
            (Cell::Empty, _)
            | (Cell::SplitterVertical, Direction::Up | Direction::Down)
            | (Cell::SplitterHorizontal, Direction::Left | Direction::Right) => self
                .move_(pos, &dir)
                .into_iter()
                .map(|pos| (pos, dir))
                .collect(),
            (Cell::MirrorForward, Direction::Up) | (Cell::MirrorBackward, Direction::Down) => self
                .move_(pos, &Direction::Right)
                .into_iter()
                .map(|pos| (pos, Direction::Right))
                .collect(),
            (Cell::MirrorForward, Direction::Down) | (Cell::MirrorBackward, Direction::Up) => self
                .move_(pos, &Direction::Left)
                .into_iter()
                .map(|pos| (pos, Direction::Left))
                .collect(),
            (Cell::MirrorForward, Direction::Left) | (Cell::MirrorBackward, Direction::Right) => {
                self.move_(pos, &Direction::Down)
                    .into_iter()
                    .map(|pos| (pos, Direction::Down))
                    .collect()
            }
            (Cell::MirrorForward, Direction::Right) | (Cell::MirrorBackward, Direction::Left) => {
                self.move_(pos, &Direction::Up)
                    .into_iter()
                    .map(|pos| (pos, Direction::Up))
                    .collect()
            }
            (Cell::SplitterVertical, Direction::Right | Direction::Left) => self
                .move_(pos, &Direction::Up)
                .into_iter()
                .map(|pos| (pos, Direction::Up))
                .chain(
                    self.move_(pos, &Direction::Down)
                        .into_iter()
                        .map(|pos| (pos, Direction::Down)),
                )
                .collect(),
            (Cell::SplitterHorizontal, Direction::Up | Direction::Down) => self
                .move_(pos, &Direction::Left)
                .into_iter()
                .map(|pos| (pos, Direction::Left))
                .chain(
                    self.move_(pos, &Direction::Right)
                        .into_iter()
                        .map(|pos| (pos, Direction::Right)),
                )
                .collect(),
        }
    }

    fn dfs(&self, node: PositionAndDirection, seen: &mut HashSet<PositionAndDirection>) {
        if seen.insert(node) {
            for next in self.adjacent(node) {
                self.dfs(next, seen);
            }
        }
    }

    fn solve(self) -> usize {
        let mut seen: HashSet<PositionAndDirection> = HashSet::new();
        self.dfs(((0, 0), Direction::Right), &mut seen);
        let visited: HashSet<Position> = seen.into_iter().map(|(pos, _)| pos).collect();
        visited.len()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
