use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

struct Game {
    id: u32,
    subsets: Vec<ColorSet>,
}

#[derive(Copy, Clone, Debug, Default)]
struct ColorSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl ColorSet {
    fn get(&self, color: Color) -> u32 {
        match color {
            Red => self.red,
            Green => self.green,
            Blue => self.blue,
        }
    }

    fn set(&mut self, color: Color, count: u32) {
        match color {
            Red => self.red = count,
            Green => self.green = count,
            Blue => self.blue = count,
        }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromIterator<(Color, u32)> for ColorSet {
    fn from_iter<I: IntoIterator<Item = (Color, u32)>>(iter: I) -> Self {
        let mut colors = Self::default();
        for (color, count) in iter {
            colors.set(color, count);
        }
        colors
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

use Color::{Blue, Green, Red};

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Red),
            "green" => Ok(Green),
            "blue" => Ok(Blue),
            _ => Err(format!("unknown color: {s}")),
        }
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (game_id, subsets) = line.split_once(": ").unwrap();
            let id = game_id.strip_prefix("Game ").unwrap().parse().unwrap();

            let subsets = subsets
                .split(';')
                .map(|subset| {
                    subset
                        .trim()
                        .split(',')
                        .map(|color| {
                            let (count, color) = color.trim().split_once(' ').unwrap();

                            let count = count.parse().unwrap();
                            let color = color.parse().unwrap();
                            (color, count)
                        })
                        .collect()
                })
                .collect();

            Game { id, subsets }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|game| {
            game.subsets.iter().all(|subset| {
                subset.get(Red) <= 12 && subset.get(Green) <= 13 && subset.get(Blue) <= 14
            })
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            game.subsets
                .iter()
                .copied()
                .reduce(|l, r| l.max(&r))
                .unwrap_or_default()
        })
        .map(|colors| colors.power())
        .sum()
}
