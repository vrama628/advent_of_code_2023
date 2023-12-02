use std::io::stdin;

const DIGIT_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let mut sum = 0;
    for line in stdin().lines().map(|line| line.unwrap()) {
        let digits = (0..line.len())
            .map(|i| &line[i..])
            .filter_map(|suffix| {
                for digit in 0..10 {
                    if suffix.starts_with(&digit.to_string())
                        || suffix.starts_with(DIGIT_WORDS[digit])
                    {
                        return Some(digit);
                    }
                }
                None
            })
            .collect::<Vec<usize>>();
        let first_digit = digits[0];
        let last_digit = digits[digits.len() - 1];
        sum += first_digit * 10 + last_digit;
    }
    println!("{sum}")
}
