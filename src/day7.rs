use std::{fmt::Debug, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day7)]
fn parse_input(s: &str) -> Vec<(Hand, usize)> {
    s.lines()
        .map(|line| {
            let (hand, bid) = line.split_whitespace().collect_tuple().unwrap();
            (hand.parse().unwrap(), bid.parse().unwrap())
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[(Hand, usize)]) -> usize {
    let mut hands = input.to_vec();
    hands.sort_by_cached_key(|(hand, _)| *hand);

    hands
        .iter()
        .enumerate()
        .map(|(n, (_, bid))| (n + 1) * bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[(Hand, usize)]) -> usize {
    let mut hands = input
        .iter()
        .map(|(hand, bid)| (hand.with_joker(), *bid))
        .collect::<Vec<_>>();
    hands.sort_by_cached_key(|(hand, _)| *hand);

    hands
        .iter()
        .enumerate()
        .map(|(n, (_, bid))| (n + 1) * bid)
        .sum()
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Card(u8);

impl Card {
    const JACK: Card = Card(11);
    const JOKER: Card = Card(0);

    fn from_char(ch: char) -> Self {
        match ch {
            '2'..='9' => Self(ch.to_digit(10).unwrap() as u8),
            'T' => Self(10),
            'J' => Self(11),
            'Q' => Self(12),
            'K' => Self(13),
            'A' => Self(14),
            _ => panic!("invalid card rank: {ch:?}"),
        }
    }

    fn as_char(&self) -> char {
        match self.0 {
            0 => '*',
            2..=9 => std::char::from_digit(self.0 as u32, 10).unwrap(),
            10 => 'T',
            11 => 'J',
            12 => 'Q',
            13 => 'K',
            14 => 'A',
            _ => panic!("invalid card rank: {:?}", self.0),
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Rank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Rank {
    fn from_cards(cards: [Card; 5]) -> Self {
        let mut counts = [0; 5];
        for (count, card) in counts.iter_mut().zip(&cards) {
            *count = cards
                .iter()
                .filter(|&c| c == card || *c == Card::JOKER)
                .count();
        }
        counts.sort();

        let has_jokers = cards.iter().any(|&card| card == Card::JOKER);

        match (counts, has_jokers) {
            ([_, _, _, _, 5], _) => Self::FiveOfAKind,
            ([_, _, _, _, 4], _) => Self::FourOfAKind,
            ([2, 2, 3, 3, 3], false) | ([1, 3, 3, 3, 3], true) => Self::FullHouse,
            ([_, _, _, _, 3], _) => Self::ThreeOfAKind,
            ([1, 2, 2, 2, 2], false) => Self::TwoPairs,
            ([_, _, _, _, 2], _) => Self::OnePair,
            ([_, _, _, _, 1], _) => Self::HighCard,

            _ => panic!("invalid counts: {counts:?}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    rank: Rank,
    cards: [Card; 5],
}

impl Hand {
    fn with_joker(self) -> Self {
        let cards = self.cards.map(|card| {
            if card == Card::JACK {
                Card::JOKER
            } else {
                card
            }
        });

        Self {
            rank: Rank::from_cards(cards),
            cards,
        }
    }
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card(0); 5];
        let mut chars = s.chars();
        for card in cards.iter_mut() {
            *card = Card::from_char(chars.next().ok_or("Not enough cards")?);
        }
        if chars.next().is_none() {
            let rank = Rank::from_cards(cards);

            Ok(Self { rank, cards })
        } else {
            Err("Too many cards")
        }
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card.as_char())?;
        }
        Ok(())
    }
}
