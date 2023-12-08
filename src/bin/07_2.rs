use itertools::Itertools;
use std::{io::stdin, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand(Vec<Card>);

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                'J' => Card::Joker,
                _ => unreachable!(),
            })
            .collect();
        Ok(Self(cards))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn type_of_hand(&self) -> HandType {
        let counts = self.0.iter().filter(|c| c != &&Card::Joker).counts();
        let num_jokers = self.0.iter().filter(|c| c == &&Card::Joker).count();
        let mut descending_counts = counts.values().sorted().rev();
        match descending_counts.next().unwrap_or(&0) + num_jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match descending_counts.next().unwrap() {
                2 => HandType::FullHouse,
                1 => HandType::ThreeOfAKind,
                _ => unreachable!(),
            },
            2 => match descending_counts.next().unwrap() {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => unreachable!(),
            },
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.type_of_hand().cmp(&other.type_of_hand()) {
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct InputLine {
    hand: Hand,
    bid: usize,
}

impl FromStr for InputLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_s, bid_s) = s.split_once(' ').unwrap();
        let hand = hand_s.parse().unwrap();
        let bid = usize::from_str(bid_s).unwrap();
        Ok(Self { hand, bid })
    }
}

struct Input(Vec<InputLine>);

impl Input {
    fn parse() -> Self {
        Self(
            stdin()
                .lines()
                .map(|line| line.unwrap().parse().unwrap())
                .collect(),
        )
    }

    fn solve(&self) -> usize {
        self.0
            .iter()
            .sorted_by_key(|input_line| &input_line.hand)
            .enumerate()
            .map(|(i, input_line)| input_line.bid * (i + 1))
            .sum()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
