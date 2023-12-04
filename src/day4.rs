use std::collections::{BTreeSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

struct Card {
    winning: BTreeSet<u32>,
    ticket: BTreeSet<u32>,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (winning, ticket) = line.split_once(':').unwrap().1.split_once('|').unwrap();

            let winning = winning
                .split_whitespace()
                .map(|n| n.trim().parse().unwrap())
                .collect();
            let ticket = ticket
                .split_whitespace()
                .map(|n| n.trim().parse().unwrap())
                .collect();

            Card { winning, ticket }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Card]) -> usize {
    input
        .iter()
        .map(|card| card.winning.intersection(&card.ticket).count())
        .map(|count| if count > 0 { 1 << (count - 1) } else { 0 })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> usize {
    let mut sum = 0;
    let mut copies = VecDeque::new();

    for card in input {
        let n = 1 + copies.pop_front().unwrap_or(0);
        let c = card.winning.intersection(&card.ticket).count();
        sum += n;

        for i in 0..c {
            if i < copies.len() {
                copies[i] += n;
            } else {
                copies.push_back(n);
            }
        }
    }

    sum
}
