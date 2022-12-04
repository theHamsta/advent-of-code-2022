use regex::Regex;

fn main() {
    let input = include_str!("../../../input/day04.txt");
    let regex = Regex::new(r"(\d+)[-](\d+),(\d+)[-](\d+)").unwrap();

    let assignments: Vec<(u64, u64, u64, u64)> = regex
        .captures_iter(input)
        .map(|captures| {
            (
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
                captures[4].parse().unwrap(),
            )
        })
        .collect();

    let part1 = assignments
        .iter()
        .filter(|(a, b, c, d)| {
            ((a..=b).contains(&c) && (a..=b).contains(&d))
                || ((c..=d).contains(&a) && (c..=d).contains(&b))
        })
        .count();
    dbg!(&part1);

    let part2: i64 = assignments
        .iter()
        .filter(|(a, b, c, d)| {
            for i in *c..=*d {
                if (*a..=*b).contains(&i) {
                    return true;
                }
            }
            false
        })
        .count()
        .try_into()
        .unwrap();
    dbg!(&part2);
}
