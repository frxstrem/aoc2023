use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug, Default)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Clone, Debug)]
struct Number {
    i: usize,
    j: usize,
    l: usize,
    n: u32,
}

impl Number {
    fn is_adjacent_to(&self, symbol: &Symbol) -> bool {
        symbol.i + 1 >= self.i
            && symbol.i <= self.i + 1
            && symbol.j + 1 >= self.j
            && symbol.j <= self.j + self.l
    }
}

#[derive(Clone, Debug)]
struct Symbol {
    i: usize,
    j: usize,
    s: char,
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
                schematic.numbers.push(Number {
                    i,
                    j,
                    l: num.len(),
                    n: num.parse::<u32>().unwrap(),
                });
                line = rest;
                j += num.len();
            } else if line.starts_with(|ch: char| ch != '.') {
                let mut c = line.chars();
                let s = c.next().unwrap();
                schematic.symbols.push(Symbol { i, j, s });
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
        .filter(|num| input.symbols.iter().any(|sym| num.is_adjacent_to(sym)))
        .map(|num| num.n)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Schematic) -> u32 {
    input
        .symbols
        .iter()
        .filter(|sym| sym.s == '*')
        .filter_map(|sym| {
            let mut part_numbers = input.numbers.iter().filter(|num| num.is_adjacent_to(sym));

            let pn1 = part_numbers.next()?.n;
            let pn2 = part_numbers.next()?.n;
            if part_numbers.next().is_some() {
                return None;
            }

            Some(pn1 * pn2)
        })
        .sum()
}
