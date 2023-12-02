use aoc_runner_derive::{aoc, aoc_generator};

struct Game {
    id: u32,
    subsets: Vec<Colors>,
}

#[derive(Copy, Clone, Default)]
struct Colors {
    red: u32,
    green: u32,
    blue: u32,
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
                    let mut colors = Colors::default();
                    for color in subset.trim().split(',') {
                        let (count, color) = color.trim().split_once(' ').unwrap();
                        let count = count.parse().unwrap();
                        match color {
                            "red" => colors.red = count,
                            "green" => colors.green = count,
                            "blue" => colors.blue = count,
                            _ => panic!("unknown color: {color}"),
                        }
                    }
                    colors
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
            game.subsets
                .iter()
                .all(|subset| subset.red <= 12 && subset.green <= 13 && subset.blue <= 14)
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
                .reduce(|l, r| Colors {
                    red: l.red.max(r.red),
                    green: l.green.max(r.green),
                    blue: l.blue.max(r.blue),
                })
                .unwrap_or_default()
        })
        .map(|colors| colors.red * colors.green * colors.blue)
        .sum()
}
