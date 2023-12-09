use aoc_runner_derive::aoc;

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
fn part1(s: &str) -> i32 {
    let mut input = parse_input(s);
    input.iter_mut().map(|row| extrapolate(row)).sum()
}

#[aoc(day9, part2)]
fn part2(s: &str) -> i32 {
    let mut input = parse_input(s);
    input.iter_mut().map(|row| extrapolate_back(row)).sum()
}

#[aoc(day9, part2, reverse)]
fn part2_reverse(s: &str) -> i32 {
    let mut input = parse_input(s);
    input
        .iter_mut()
        .map(|row| {
            row.reverse();
            extrapolate(row)
        })
        .sum()
}

fn extrapolate(row: &mut [i32]) -> i32 {
    for i in 0..row.len() {
        if is_constant(&row[i..]) {
            return row[0..=i].iter().sum();
        }

        let last = *row.last().unwrap();
        for j in (i + 1..row.len()).rev() {
            row[j] -= row[j - 1];
        }
        row[i] = last;
    }
    unreachable!()
}

fn extrapolate_back(row: &mut [i32]) -> i32 {
    for i in 0..row.len() {
        if is_constant(&row[i..]) {
            return row[0..=i]
                .iter()
                .enumerate()
                .map(|(i, n)| if i % 2 == 0 { *n } else { -*n })
                .sum();
        }

        for j in (i + 1..row.len()).rev() {
            row[j] -= row[j - 1];
        }
    }
    unreachable!()
}

fn is_constant(row: &[i32]) -> bool {
    if row.len() < 2 {
        return true;
    }

    let first = row[0];
    row[1..].iter().all(|&n| n == first)
}
