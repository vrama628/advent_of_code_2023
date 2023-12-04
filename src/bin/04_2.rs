use std::{collections::HashSet, io::stdin, str::FromStr};

struct Card {
    winning_numbers: HashSet<usize>,
    numbers_you_have: Vec<usize>,
}

impl Card {
    fn num_matches(&self) -> usize {
        self.numbers_you_have
            .iter()
            .filter(|number_you_have| self.winning_numbers.contains(number_you_have))
            .count()
    }
}

fn main() {
    let cards = stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
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
        })
        .collect::<Vec<Card>>();

    let mut counts = cards.iter().map(|_| 1).collect::<Vec<usize>>();
    for (i, card) in cards.iter().enumerate() {
        for j in (i + 1)..(i + 1 + card.num_matches()) {
            counts[j] += counts[i]
        }
    }
    println!("{}", counts.iter().sum::<usize>())
}
