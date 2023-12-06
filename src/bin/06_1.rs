use std::{io::stdin, str::FromStr};

struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    fn solve(&self) -> f64 {
        let upper_bound = ((self.time + (self.time.powi(2) - 4. * self.distance).sqrt()) / 2.
            - f64::EPSILON * 16.)
            .floor();
        let lower_bound = ((self.time - (self.time.powi(2) - 4. * self.distance).sqrt()) / 2.
            + f64::EPSILON * 16.)
            .ceil();
        return upper_bound - lower_bound + 1.;
    }
}

struct Input(Vec<Race>);

impl Input {
    fn parse() -> Self {
        let mut lines = stdin().lines().map(|res| res.unwrap());
        let time_line = lines.next().unwrap();
        let distance_line = lines.next().unwrap();
        let times = time_line
            .strip_prefix("Time:")
            .unwrap()
            .split_ascii_whitespace()
            .map(|t| f64::from_str(t).unwrap());
        let distances = distance_line
            .strip_prefix("Distance:")
            .unwrap()
            .split_ascii_whitespace()
            .map(|d| f64::from_str(d).unwrap());
        Self(
            times
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect(),
        )
    }

    fn solve(&self) -> f64 {
        self.0.iter().map(|race| race.solve()).product()
    }
}

fn main() {
    println!("{}", Input::parse().solve())
}
