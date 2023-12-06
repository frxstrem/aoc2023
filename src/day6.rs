use std::fmt::Write as _;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::binary_search::binary_search_range;

#[derive(Copy, Clone, Debug, Default)]
struct Race {
    time: u64,
    dist: u64,
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();

    let mut times = lines
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(str::trim);
    let mut distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .map(str::trim);

    times
        .by_ref()
        .zip(&mut distances)
        .map(|(time, dist)| Race {
            time: time.parse().unwrap(),
            dist: dist.parse().unwrap(),
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Race]) -> u64 {
    input.iter().map(|race| race.count_ways_to_win()).product()
}

#[aoc(day6, part2)]
fn part2(input: &[Race]) -> u64 {
    let (time, dist) = input.iter().fold(
        (String::new(), String::new()),
        |(mut time, mut dist), race| {
            write!(time, "{}", race.time).unwrap();
            write!(dist, "{}", race.dist).unwrap();
            (time, dist)
        },
    );

    let race = Race {
        time: time.parse().unwrap(),
        dist: dist.parse().unwrap(),
    };

    race.count_ways_to_win()
}

impl Race {
    fn count_ways_to_win(&self) -> u64 {
        // p(t) is the distance traveled during the race if the button is held
        // down for t seconds
        let p = |t| (self.time - t) * t;

        // Observation: p(t) is symmetric about t = self.time / 2
        // We can binary search for the time at which p(t) is positive.
        let i = binary_search_range(1, (self.time + 1) / 2, |&t| p(t).cmp(&self.dist))
            .unwrap_or_else(|i| i)
            + 1;

        self.time - 2 * i + 1
    }
}
