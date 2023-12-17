use std::{
    collections::{BinaryHeap, HashSet},
    io::stdin,
};

struct Input(Vec<Vec<usize>>);

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn perpendicular(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}

struct Position {
    r: usize,
    c: usize,
    direction: Direction,
    moved_in_direction: usize,
    total_heat_loss: usize,
}

struct Seen(HashSet<(usize, usize, Direction, usize)>);

impl Seen {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn insert(&mut self, position: &Position) -> bool {
        self.0.insert((
            position.r,
            position.c,
            position.direction,
            position.moved_in_direction,
        ))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.total_heat_loss.cmp(&self.total_heat_loss)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Position {}

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(|line| {
                    line.unwrap()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        )
    }

    fn move_position(&self, position: &Position, direction: Direction) -> Option<Position> {
        let (r, c) = match direction {
            Direction::Up => (position.r > 0).then(|| (position.r - 1, position.c)),
            Direction::Down => {
                (position.r < self.0.len() - 1).then(|| (position.r + 1, position.c))
            }
            Direction::Left => (position.c > 0).then(|| (position.r, position.c - 1)),
            Direction::Right => {
                (position.c < self.0[0].len() - 1).then(|| (position.r, position.c + 1))
            }
        }?;
        let moved_in_direction = if position.direction == direction {
            position.moved_in_direction + 1
        } else {
            1
        };
        let total_heat_loss = position.total_heat_loss + self.0[r][c];
        Some(Position {
            r,
            c,
            direction,
            moved_in_direction,
            total_heat_loss,
        })
    }

    fn adjacent(&self, position: &Position) -> Vec<Position> {
        let mut directions = vec![];
        if position.moved_in_direction >= 4 {
            directions.extend(position.direction.perpendicular())
        }
        if position.moved_in_direction < 10 {
            directions.push(position.direction)
        }
        directions
            .into_iter()
            .flat_map(|direction| self.move_position(position, direction))
            .collect()
    }

    fn solve(&self) -> usize {
        let mut queue: BinaryHeap<Position> = BinaryHeap::new();
        let mut seen = Seen::new();
        queue.push(Position {
            r: 0,
            c: 0,
            direction: Direction::Right,
            moved_in_direction: 0,
            total_heat_loss: 0,
        });
        loop {
            let position = queue.pop().unwrap();
            if position.r == self.0.len() - 1
                && position.c == self.0[0].len() - 1
                && position.moved_in_direction >= 4
            {
                return position.total_heat_loss;
            }
            for adjacent in self.adjacent(&position) {
                if seen.insert(&adjacent) {
                    queue.push(adjacent);
                }
            }
        }
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
