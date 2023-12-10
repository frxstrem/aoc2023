use std::{
    collections::HashMap,
    fmt::{self, Debug},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::num::lcm;

use self::Dir::{L, R};

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
        current = traverse(current, map);
        count += map.instrs.len();
    }

    count
}

#[aoc(day8, part2)]
fn part2(map: &Map) -> usize {
    let start_nodes = map
        .node_map
        .keys()
        .copied()
        .filter(|node| node.is_start())
        .collect::<Vec<_>>();

    start_nodes
        .iter()
        .map(|&start| {
            // for each start node, compute distance to reach an end node
            let mut node = start;
            let mut n = 0;
            while !node.is_end() {
                node = traverse(node, map);
                n += 1;
            }
            (node, n)
        })
        .map(|(end, n)| {
            // for each end node, compute distance to reach another end node
            let mut node = traverse(end, map);
            let mut m = 1;
            while !node.is_end() {
                node = traverse(node, map);
                m += 1;
            }

            // input check: end nodes are in a loop with no other end nodes
            assert_eq!(end, node);
            // input check: end node loop lengths are equal to each starting
            // node's distance to the end node loop
            assert_eq!(n, m);

            (end, n)
        })
        .map(|(_, n)| n as usize)
        .reduce(lcm)
        .unwrap_or(1)
        * map.instrs.len()
}

fn traverse(start: Node, map: &Map) -> Node {
    let mut current = start;

    for instr in &map.instrs {
        let (left, right) = map.node_map.get(&current).copied().unwrap();

        current = match instr {
            R => right,
            L => left,
        };
    }

    current
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    R,
    L,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node([u8; 3]);

impl Node {
    const START: Self = Self(*b"AAA");
    const END: Self = Self(*b"ZZZ");
}

impl Node {
    fn is_start(self) -> bool {
        self.0[2] == b'A'
    }

    fn is_end(self) -> bool {
        self.0[2] == b'Z'
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.escape_ascii())
    }
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
