use itertools::Itertools;
use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
struct Instruction {
    howmany: u64,
    from: u64,
    to: u64,
}

//fn all_different(chars: char[]) -> bool {
//let set: HashSet<_> = chars.iter().unique().collect();
//set.count() == chars.len()
//}

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
