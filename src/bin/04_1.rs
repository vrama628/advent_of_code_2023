use std::{collections::HashSet, io::stdin, str::FromStr};

struct Card {
    winning_numbers: HashSet<usize>,
    numbers_you_have: Vec<usize>,
}

impl Card {
    fn value(&self) -> usize {
        let num_matches = self
            .numbers_you_have
            .iter()
            .filter(|number_you_have| self.winning_numbers.contains(number_you_have))
            .count();
        if num_matches == 0 {
            0
        } else {
            2usize.pow((num_matches - 1) as u32)
        }
    }
}

fn main() {
    let cards = stdin().lines().map(|line| line.unwrap()).map(|line| {
        let (winning, you_have) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_numbers = winning
            .split_ascii_whitespace()
            .map(|n| usize::from_str(n).unwrap())
            .collect::<HashSet<usize>>();
        let numbers_you_have = you_have
            .split_ascii_whitespace()
            .map(|n| usize::from_str(n).unwrap())
            .collect::<Vec<usize>>();
        Card {
            winning_numbers,
            numbers_you_have,
        }
    });
    let sum: usize = cards.map(|card| card.value()).sum();
    println!("{sum}")
}
