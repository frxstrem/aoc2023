use std::fmt::Write as _;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::binary_search::binary_search_range;

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

#[aoc(day6, part1, direct_search)]
fn part1_direct_search(input: &[Race]) -> u64 {
    input
        .iter()
        .map(|race| race.count_ways_to_win_direct_search())
        .product()
}

#[aoc(day6, part1, binary_search)]
fn part1_binary_search(input: &[Race]) -> u64 {
    input
        .iter()
        .map(|race| race.count_ways_to_win_binary_search())
        .product()
}
#[aoc(day6, part1, solve_quadratic)]
fn part1_solve_quadratic(input: &[Race]) -> u64 {
    input
        .iter()
        .map(|race| race.count_ways_to_win_solve_quadratic())
        .product()
}

#[aoc(day6, part2, direct_search)]
fn part2_direct_search(input: &[Race]) -> u64 {
    parse_concat_race(input).count_ways_to_win_direct_search()
}

#[aoc(day6, part2, binary_search)]
fn part2_binary_search(input: &[Race]) -> u64 {
    parse_concat_race(input).count_ways_to_win_binary_search()
}
#[aoc(day6, part2, solve_quadratic)]
fn part2_solve_quadratic(input: &[Race]) -> u64 {
    parse_concat_race(input).count_ways_to_win_solve_quadratic()
}

fn parse_concat_race(races: &[Race]) -> Race {
    let (time, dist) = races.iter().fold(
        (String::new(), String::new()),
        |(mut time, mut dist), race| {
            write!(time, "{}", race.time).unwrap();
            write!(dist, "{}", race.dist).unwrap();
            (time, dist)
        },
    );

    Race {
        time: time.parse().unwrap(),
        dist: dist.parse().unwrap(),
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn count_ways_to_win_direct_search(&self) -> u64 {
        // p(t) is the distance traveled during the race if the button is held
        // down for t seconds
        let p = |t| (self.time - t) * t;

        let mut range = 1..self.time;

        let first = range
            .find(|t| p(*t) > self.dist)
            .unwrap_or_else(|| panic!("race has no possible time to beat record (race={self:?})"));

        // Take advantage of the fact that p(t) is symmetric about t = self.time / 2
        let last = self.time - first;

        last - first + 1
    }

    fn count_ways_to_win_binary_search(&self) -> u64 {
        // p(t) is the distance traveled during the race if the button is held
        // down for t seconds
        let p = |t| (self.time - t) * t;

        // Take advantage of the fact that p(t) is symmetric about t = self.time / 2,
        // we can binary search for the time at which p(t) is positive.
        let i = binary_search_range(0, (self.time + 1) / 2, |&t| p(t).cmp(&self.dist))
            .unwrap_or_else(|i| i)
            + 1;

        self.time - 2 * i + 1
    }

    fn count_ways_to_win_solve_quadratic(&self) -> u64 {
        // p(t) = (self.time - t) * t
        // p(t) - d = - t**2 + self.time * t - d
        // solve quadratic:
        //   a = -1
        //   b = self.time
        //   c = -d
        //   t = (-b +- sqrt(b**2 - 4ac)) / 2a
        //     = (self.time +- sqrt(self.time**2 - 4d)) / 2
        //     = (self.time / 2) +- sqrt((self.time / 2)**2 - d)

        let mt = self.time as f64 / 2.0;
        let root = (mt * mt - self.dist as f64).sqrt();

        let first: u64 = (mt - root + 1.0).floor() as u64;

        self.time - 2 * first + 1
    }
}
