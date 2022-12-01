use itertools::Itertools;

fn main() {
    let input = include_str!("../../../input/day01.txt");

    let calories: Vec<Vec<i64>> = input
        .split("\n\n")
        .map(|group| group.split("\n").flat_map(|c| c.parse::<i64>()).collect())
        .collect();
    dbg!(&calories.iter().map(|inner| inner.iter().sum::<i64>()).max());
    dbg!(&calories
        .iter()
        .map(|inner| inner.iter().sum::<i64>())
        .sorted()
        .rev()
        .take(3)
        .sum::<i64>());
}
