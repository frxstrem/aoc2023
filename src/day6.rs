use std::fmt::Write as _;

use aoc_runner_derive::{aoc, aoc_generator};

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
        let p = |t| (self.time - t) * t;

        let mut range = 1..self.time;

        let first = range
            .find(|t| p(*t) > self.dist)
            .unwrap_or_else(|| panic!("race has no possible time to beat record (race={self:?})"));
        let last = range.rfind(|t| p(*t) > self.dist).unwrap_or(first);

        last - first + 1
    }
}
