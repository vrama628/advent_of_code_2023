use std::{io::stdin, str::FromStr};

struct SetOfCubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for SetOfCubes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set_of_cubes = SetOfCubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        for entry in s.split(", ") {
            let (n, color) = entry.split_once(" ").unwrap();
            let n: usize = n.parse().unwrap();
            match color {
                "red" => set_of_cubes.red += n,
                "green" => set_of_cubes.green += n,
                "blue" => set_of_cubes.blue += n,
                _ => panic!("Unknown color"),
            }
        }
        Ok(set_of_cubes)
    }
}

fn is_possible(SetOfCubes { red, green, blue }: SetOfCubes) -> bool {
    red <= 12 && green <= 13 && blue <= 14
}

fn main() {
    let mut sum = 0;
    for (i, line) in stdin().lines().map(|line| line.unwrap()).enumerate() {
        let (_, sets_of_cubes) = line.split_once(": ").unwrap();
        if sets_of_cubes
            .split("; ")
            .map(|set_of_cubes| set_of_cubes.parse::<SetOfCubes>().unwrap())
            .all(is_possible)
        {
            sum += i + 1;
        }
    }
    println!("{}", sum)
}
