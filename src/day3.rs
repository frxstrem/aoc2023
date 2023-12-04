use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, Default)]
struct Schematic {
    numbers: Vec<(usize, usize, usize, u32)>,
    symbols: Vec<(usize, usize, char)>,
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Schematic {
    let mut schematic = Schematic::default();

    for (i, mut line) in input.lines().enumerate() {
        let mut j = 0;
        while !line.is_empty() {
            if line.starts_with(|ch: char| ch.is_ascii_digit()) {
                let (num, rest) = line.split_at(
                    line.find(|ch: char| !ch.is_ascii_digit())
                        .unwrap_or(line.len()),
                );
                schematic
                    .numbers
                    .push((i, j, num.len(), num.parse::<u32>().unwrap()));
                line = rest;
                j += num.len();
            } else if line.starts_with(|ch: char| ch != '.') {
                let mut c = line.chars();
                let symbol = c.next().unwrap();
                schematic.symbols.push((i, j, symbol));
                line = c.as_str();
                j += 1;
            }

            match line.find(|ch: char| ch != '.') {
                Some(index) => {
                    line = &line[index..];
                    j += index;
                }
                None => break,
            }
        }
    }

    schematic
}

#[aoc(day3, part1)]
fn part1(input: &Schematic) -> u32 {
    input
        .numbers
        .iter()
        .filter(|(i, j, l, _)| {
            input
                .symbols
                .iter()
                .any(|(si, sj, _)| *si + 1 >= *i && *si <= *i + 1 && *sj + 1 >= *j && *sj <= *j + l)
        })
        .map(|(_, _, _, n)| n)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Schematic) -> u32 {
    input
        .symbols
        .iter()
        .filter(|(_, _, s)| *s == '*')
        .filter_map(|(si, sj, _)| {
            let mut part_numbers = input.numbers.iter().filter(|(i, j, l, _)| {
                *si + 1 >= *i && *si <= *i + 1 && *sj + 1 >= *j && *sj <= *j + l
            });

            let pn1 = part_numbers.next()?.3;
            let pn2 = part_numbers.next()?.3;
            if part_numbers.next().is_some() {
                return None;
            }

            Some(pn1 * pn2)
        })
        .sum()
}
