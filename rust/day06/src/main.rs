use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("../../../input/day06.txt");

    let part1 = input
        .as_bytes()
        .windows(4)
        .position(|chars| chars.iter().unique().count() == chars.len())
        .unwrap()
        + 4;
    dbg!(&part1);

    let part2 = input
        .as_bytes()
        .windows(14)
        .position(|chars| chars.iter().unique().count() == chars.len())
        .unwrap()
        + 14;
    dbg!(&part2);
}
