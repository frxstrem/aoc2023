use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    R,
    L,
}

use Dir::{L, R};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Node([u8; 3]);

impl Node {
    const START: Self = Self(*b"AAA");
    const END: Self = Self(*b"ZZZ");
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_bytes()
            .try_into()
            .map(Self)
            .map_err(|_| "invalid node")
    }
}

#[derive(Clone, Debug)]
struct Map {
    instrs: Vec<Dir>,
    node_map: HashMap<Node, (Node, Node)>,
}

#[aoc_generator(day8)]
fn parse_input(s: &str) -> Map {
    let mut lines = s.lines();

    let instrs = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            'R' => R,
            'L' => L,
            _ => panic!("unknown instruction: {ch:?}"),
        })
        .collect();

    assert_eq!(lines.next().unwrap(), "");

    let node_map = lines
        .map(|line| {
            let (node, children) = line.split_once('=').unwrap();
            let node = node.trim().parse().unwrap();

            let (left, right) = children
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(',')
                .unwrap();

            let left = left.trim().parse().unwrap();
            let right = right.trim().parse().unwrap();

            (node, (left, right))
        })
        .collect();

    Map { instrs, node_map }
}

#[aoc(day8, part1)]
fn part1(map: &Map) -> usize {
    let mut current = Node::START;
    let mut count = 0;

    while current != Node::END {
        current = traverse(current, &map.node_map, &map.instrs);
        count += map.instrs.len();
    }

    count
}

fn traverse(start: Node, map: &HashMap<Node, (Node, Node)>, instrs: &[Dir]) -> Node {
    let mut current = start;

    for instr in instrs {
        let (left, right) = map.get(&current).copied().unwrap();

        current = match instr {
            R => right,
            L => left,
        };
    }

    current
}
