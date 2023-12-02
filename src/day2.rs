use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

struct Game {
    id: u32,
    subsets: Vec<ColorSet>,
}

#[derive(Clone, Debug, Default)]
struct ColorSet {
    colors: HashMap<Color, u32>,
}

impl ColorSet {
    fn get(&self, color: Color) -> u32 {
        self.colors.get(&color).copied().unwrap_or(0)
    }

    fn max(&self, other: &Self) -> Self {
        let colors = [Red, Green, Blue]
            .into_iter()
            .map(|color| (color, self.get(color).max(other.get(color))))
            .filter(|(_, n)| *n > 0)
            .collect();
        Self { colors }
    }

    fn power(&self) -> u32 {
        self.get(Red) * self.get(Green) * self.get(Blue)
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
                    let colors = subset
                        .trim()
                        .split(',')
                        .map(|color| {
                            let (count, color) = color.trim().split_once(' ').unwrap();

                            let count = count.parse().unwrap();
                            let color = color.parse().unwrap();
                            (color, count)
                        })
                        .collect();
                    ColorSet { colors }
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
                .cloned()
                .reduce(|l, r| l.max(&r))
                .unwrap_or_default()
        })
        .map(|colors| colors.power())
        .sum()
}
