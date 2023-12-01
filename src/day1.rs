use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &[u8]) -> u32 {
    input
        .split(|&b| b == b'\n')
        .map(|line| {
            let first_digit = line.iter().find(|b| b.is_ascii_digit()).unwrap() - b'0';
            let last_digit = line.iter().rfind(|b| b.is_ascii_digit()).unwrap() - b'0';
            (first_digit * 10 + last_digit) as u32
        })
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u8]) -> u32 {
    input
        .split(|&b| b == b'\n')
        .map(|line| {
            let first_digit = (0..line.len())
                .map(|i| &line[i..])
                .find_map(is_digit)
                .unwrap();
            let last_digit = (0..line.len())
                .rev()
                .map(|i| &line[i..])
                .find_map(is_digit)
                .unwrap();
            (first_digit * 10 + last_digit) as u32
        })
        .sum()
}

fn is_digit(b: &[u8]) -> Option<u8> {
    match b {
        [ch @ b'0'..=b'9', ..] => Some(ch - b'0'),
        [b'o', b'n', b'e', ..] => Some(1),
        [b't', b'w', b'o', ..] => Some(2),
        [b't', b'h', b'r', b'e', b'e', ..] => Some(3),
        [b'f', b'o', b'u', b'r', ..] => Some(4),
        [b'f', b'i', b'v', b'e', ..] => Some(5),
        [b's', b'i', b'x', ..] => Some(6),
        [b's', b'e', b'v', b'e', b'n', ..] => Some(7),
        [b'e', b'i', b'g', b'h', b't', ..] => Some(8),
        [b'n', b'i', b'n', b'e', ..] => Some(9),
        _ => None,
    }
}
