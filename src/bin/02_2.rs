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

impl SetOfCubes {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn fewest(set1: SetOfCubes, set2: SetOfCubes) -> SetOfCubes {
    SetOfCubes {
        red: set1.red.max(set2.red),
        green: set1.green.max(set2.green),
        blue: set1.blue.max(set2.blue),
    }
}

fn main() {
    let mut sum = 0;
    for line in stdin().lines().map(|line| line.unwrap()) {
        let (_, sets_of_cubes) = line.split_once(": ").unwrap();
        sum += sets_of_cubes
            .split("; ")
            .map(|set_of_cubes| set_of_cubes.parse::<SetOfCubes>().unwrap())
            .reduce(fewest)
            .unwrap()
            .power()
    }
    println!("{}", sum)
}
