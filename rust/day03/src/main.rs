use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../../input/day03.txt");

    let line_pairs = input
        .lines()
        .map(|line| line.as_bytes().split_at(line.len() / 2))
        .collect_vec();

    let mut sum = 0u64;

    for (a, b) in line_pairs.iter() {
        let set_a: HashSet<u8> = a.iter().copied().collect();
        let set_b: HashSet<u8> = b.iter().copied().collect();
        let common = set_a.intersection(&set_b).next();

        match common {
            Some(c @ b'a'..=b'z') => {
                sum += (c - b'a' + 1) as u64;
            }
            Some(c @ b'A'..=b'Z') => {
                sum += (c - b'A' + 1 + 26) as u64;
            }
            _ => {}
        }
    }
    let part1 = sum;
    dbg!(&part1);

    let sets = input
        .lines()
        .map(|line| line.as_bytes().iter().cloned().collect::<HashSet<_>>())
        .collect_vec();

    let part2 = sets
        .chunks(3)
        .map(|chunks| {
            chunks[0]
                .intersection(&chunks[1])
                .copied()
                .collect::<HashSet<u8>>()
                .intersection(&chunks[2])
                .copied()
                .next()
        })
        .map(|common| match common {
            Some(c @ b'a'..=b'z') => (c - b'a' + 1) as u64,
            Some(c @ b'A'..=b'Z') => (c - b'A' + 1 + 26) as u64,
            _ => 0,
        })
        .sum::<u64>();
    dbg!(&part2);
}
