use itertools::Itertools;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("../../../input/day15.txt");

    let regex = Regex::new(
        r"Sensor\s*at\s*x[=]([-]?\d+),\s*y[=]([-]?\d+):\s*closest\s*beacon\s*is\s*at\s*x[=]([-]?\d+),\s*y[=]([-]?\d+)",
    )
    .unwrap();
    let input: Vec<((i64, i64), (i64, i64))> = input
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(|l| {
            regex.captures(l).map(|c| {
                (
                    (c[1].parse().unwrap(), c[2].parse().unwrap()),
                    (c[3].parse().unwrap(), c[4].parse().unwrap()),
                )
            })
        })
        .collect();

    let mut map = HashMap::new();
    for &(sensor, beacon) in input.iter() {
        map.insert(sensor, 'S');
        map.insert(beacon, 'B');
    }
    let target_y = 2000000;
    let mut part1 = 0;

    let search_max = 4000000i64;

    let mut marked = vec![];

    for &(sensor, beacon) in input.iter() {
        let (sx, sy) = sensor;
        let (bx, by) = beacon;
        let dx = (sx - bx).abs();
        let dy = (sy - by).abs();
        let dist = dx + dy;

        let target_dist = sy - target_y;
        if target_dist.abs() <= dist {
            let left_dist = dist - target_dist.abs();
            for x in (sx - left_dist)..=(sx + left_dist) {
                match map.insert((x, target_y), '#') {
                    None => part1 += 1,
                    _ => (),
                }
            }
        }

        for dy in (-dist)..=dist {
            let dx = dist.abs() - dy.abs();

            let line = dy + sy;
            if line >= 0 && line < search_max {
                let x_interval = (
                    (sx - dx).max(0) + line * search_max,
                    (sx + dx).min(search_max - 1) + line * search_max,
                );
                marked.push(rust_lapper::Interval {
                    start: x_interval.0 as u64,
                    stop: x_interval.1 as u64,
                    val: true,
                });
            }
        }
    }
    dbg!(&part1);

    let mut data = rust_lapper::Lapper::new(marked);
    data.merge_overlaps();
    for (a, b) in data.iter().collect::<Vec<_>>().iter().tuple_windows() {
        if a.stop + 1 != b.start {
            let gap: i64 = a.stop as i64 + 1;
            let (gap_x, gap_y) = (gap % search_max, gap / search_max);
            let part2 = gap_x * 4000000 + gap_y;
            dbg!(&part2);
        }
    }
}
