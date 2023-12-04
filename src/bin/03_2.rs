use std::{collections::HashSet, io::stdin, str::FromStr};

struct Number {
    n: usize,
    len: usize,
    coords: (i64, i64),
}

impl Number {
    fn surrounding_coordinates(&self) -> HashSet<(i64, i64)> {
        (self.coords.0 - 1..=self.coords.0 + 1)
            .flat_map(|r| {
                [
                    (r, self.coords.1 - 1),
                    (r, self.coords.1 + i64::try_from(self.len).unwrap()),
                ]
            })
            .chain(
                (self.coords.1..self.coords.1 + i64::try_from(self.len).unwrap())
                    .flat_map(|c| [(self.coords.0 - 1, c), (self.coords.0 + 1, c)]),
            )
            .collect()
    }
}

fn main() {
    let mut numbers: Vec<Number> = vec![];
    let mut gears: Vec<(i64, i64)> = vec![];
    for (r, mut line) in stdin().lines().map(|line| line.unwrap()).enumerate() {
        let mut current_number: Option<Vec<char>> = None;
        line.push('.');
        for (c, char) in line.char_indices() {
            if char.is_ascii_digit() {
                match &mut current_number {
                    Some(number) => number.push(char),
                    None => current_number = Some(vec![char]),
                }
            } else {
                match &mut current_number {
                    None => (),
                    Some(number) => {
                        numbers.push(Number {
                            n: usize::from_str(&number.iter().collect::<String>()).unwrap(),
                            len: number.len(),
                            coords: (
                                r.try_into().unwrap(),
                                (c - (number.len())).try_into().unwrap(),
                            ),
                        });
                        current_number = None;
                    }
                }
                if char == '*' {
                    gears.push((r.try_into().unwrap(), c.try_into().unwrap()));
                }
            }
        }
    }
    let mut sum = 0;
    for gear in gears {
        let adjacent_numbers = numbers
            .iter()
            .filter(|number| number.surrounding_coordinates().contains(&gear))
            .collect::<Vec<&Number>>();
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0].n * adjacent_numbers[1].n;
        }
    }
    println!("{sum}")
}
