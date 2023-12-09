use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(s: &[Vec<i32>]) -> i32 {
    let mut s = s.to_vec();
    s.iter_mut().map(|row| extrapolate(row)).sum()
}

#[aoc(day9, part2)]
fn part2(s: &[Vec<i32>]) -> i32 {
    let mut s = s.to_vec();
    s.iter_mut()
        .map(|row| {
            row.reverse();
            extrapolate(row)
        })
        .sum()
}

fn extrapolate(row: &mut [i32]) -> i32 {
    let mut row = row.to_vec();
    let mut i = 0;
    let mut n = loop {
        assert!(i < row.len());

        if is_constant(&row[i..]) {
            break row[i];
        }

        diff(&mut row[i..]);
        i += 1;
    };
    while i > 0 {
        i -= 1;
        cuml(&mut row[i..]);
        n += row.last().unwrap();
    }

    n
}

fn is_constant(row: &[i32]) -> bool {
    if row.len() < 2 {
        return true;
    }

    let first = row[0];
    row[1..].iter().all(|&n| n == first)
}

/// Turn a row of integers into a row of differences.
///
/// The first element stays the same, whereas for every other element
/// `a[i] = a[i] - a[i-1]`.
fn diff(row: &mut [i32]) {
    for i in (1..row.len()).rev() {
        row[i] -= row[i - 1];
    }
}

/// Turn a row of integers into a row of cumulative sums.
///
/// This is the inverse of `diff`.
fn cuml(row: &mut [i32]) {
    for i in 1..row.len() {
        row[i] += row[i - 1];
    }
}
