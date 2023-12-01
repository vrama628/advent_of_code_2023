use std::io::stdin;

fn main() {
    let mut sum = 0;
    for line in stdin().lines().map(|line| line.unwrap()) {
        let first_digit_index = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let last_digit_index = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        sum += line
            .chars()
            .nth(first_digit_index)
            .unwrap()
            .to_digit(10)
            .unwrap()
            * 10
            + line
                .chars()
                .nth(last_digit_index)
                .unwrap()
                .to_digit(10)
                .unwrap();
    }
    println!("{sum}")
}
