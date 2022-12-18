use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let cubes: HashSet<(i64, i64, i64)> = include_str!("../../../input/day18.txt")
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let parsed = l.split(",").map(|s| s.parse().unwrap()).collect_vec();
            (parsed[0], parsed[1], parsed[2])
        })
        .collect();

    let (&min_x, &max_x) = cubes
        .iter()
        .map(|(x, _, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (&min_y, &max_y) = cubes
        .iter()
        .map(|(_, y, _)| y)
        .minmax()
        .into_option()
        .unwrap();
    let (&min_z, &max_z) = cubes
        .iter()
        .map(|(_, _, z)| z)
        .minmax()
        .into_option()
        .unwrap();

    let part1 = cubes
        .iter()
        .map(|d| {
            [
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, -1),
                (0, 0, -1),
            ]
            .iter()
            .copied()
            .map(|(x, y, z)| (d.0 + x, d.1 + y, d.2 + z))
            .filter(|c| !cubes.contains(c))
            .count()
        })
        .sum::<usize>();
    dbg!(&part1);

    let mut outside = HashSet::new();
    let mut to_visit = vec![(min_x, min_y, min_z)];
    while let Some((x, y, z)) = to_visit.pop() {
        if x < min_x - 1
            || x > max_x + 1
            || y < min_y - 1
            || y > max_y + 1
            || z < min_z - 1
            || z > max_z + 1
        {
            continue;
        }
        if cubes.contains(&(x, y, z)) || outside.contains(&(x, y, z)) {
            continue;
        }
        outside.insert((x, y, z));
        to_visit.push((x + 1, y, z));
        to_visit.push((x - 1, y, z));
        to_visit.push((x, y + 1, z));
        to_visit.push((x, y - 1, z));
        to_visit.push((x, y, z + 1));
        to_visit.push((x, y, z - 1));
    }

    let part1 = cubes
        .iter()
        .map(|d| {
            [
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, -1),
                (0, 0, -1),
            ]
            .iter()
            .copied()
            .map(|(x, y, z)| (d.0 + x, d.1 + y, d.2 + z))
            .filter(|c| outside.contains(c))
            .count()
        })
        .sum::<usize>();
    dbg!(&part1);
    //dbg!(&part1);
    ////let part2 = play_rock_tetris(&input, 1000000000000, false);
    //dbg!(&part);
}
