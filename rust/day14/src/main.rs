use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};

use itertools::{Itertools, MinMaxResult};
use regex::Regex;

fn main() {
    let input = include_str!("../../../input/day14.txt");

    let regex = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut rocks: Vec<Vec<(i64, i64)>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            regex
                .captures_iter(l)
                .map(|c| (c[1].parse().unwrap(), c[2].parse().unwrap()))
                .collect()
        })
        .collect();

    let mut map = HashMap::<(i64, i64), char>::new();
    if let MinMaxResult::MinMax(&_min_x, &_max_x) = rocks.iter().flatten().map(|(x, _y)| x).minmax() {
        if let MinMaxResult::MinMax(&_min_y, &max_y) =
            rocks.iter().flatten().map(|(_x, y)| y).minmax()
        {
            rocks.iter().for_each(|line| {
                line.iter().tuple_windows().for_each(|(a, b)| {
                    for x in (a.0.min(b.0))..=(a.0.max(b.0)) {
                        for y in (a.1.min(b.1))..=(a.1.max(b.1)) {
                            map.insert((x, y), 'x');
                        }
                    }
                })
            });

            let mut part1 = -1;
            for s in 0.. {
                //println!("---------------------");
                //for y in _min_y..=max_y {
                //for x in _min_x..=_max_x {
                //print!("{}", map.get(&(x, y)).unwrap_or(&' '));
                //}
                //println!();
                //}
                //println!("---------------------");

                let mut sand_pos = (500, 0);
                while sand_pos.1 < max_y {
                    if !map.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
                        sand_pos = (sand_pos.0, sand_pos.1 + 1);
                    } else if !map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                        sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
                    } else if !map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                        sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
                    } else {
                        map.insert(sand_pos, 'o');
                        break;
                    }
                }
                if sand_pos.1 == max_y {
                    part1 = s;
                    break;
                }
            }
            dbg!(&part1);
        }
    }

    let mut map = HashMap::<(i64, i64), char>::new();

    if let MinMaxResult::MinMax(&min_x, &_max_x) = rocks.iter().flatten().map(|(x, _y)| x).minmax() {
        let max_y = rocks.iter().flatten().map(|(_x, y)| y).max().unwrap();
        rocks.push(vec![(min_x - 300, max_y + 2), (min_x + 300, max_y + 2)]);
        if let MinMaxResult::MinMax(&_min_y, &max_y) =
            rocks.iter().flatten().map(|(_x, y)| y).minmax()
        {
            rocks.iter().for_each(|line| {
                line.iter().tuple_windows().for_each(|(a, b)| {
                    for x in (a.0.min(b.0))..=(a.0.max(b.0)) {
                        for y in (a.1.min(b.1))..=(a.1.max(b.1)) {
                            map.insert((x, y), 'x');
                        }
                    }
                })
            });

            let mut part2 = -1;
            for s in 0.. {
                //println!("---------------------");
                //for y in 0..=_max_y {
                //for x in (min_x - 30)..=(_max_x + 30) {
                //print!("{}", map.get(&(x, y)).unwrap_or(&' '));
                //}
                //println!();
                //}
                //println!("---------------------");

                let mut sand_pos = (500, 0);
                if map.contains_key(&sand_pos) {
                    part2 = s;
                    break;
                }
                while sand_pos.1 < max_y {
                    if !map.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
                        sand_pos = (sand_pos.0, sand_pos.1 + 1);
                    } else if !map.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                        sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
                    } else if !map.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                        sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
                    } else {
                        map.insert(sand_pos, 'o');
                        break;
                    }
                }
                if sand_pos.1 == max_y {
                    part2 = s;
                    break;
                }
            }
            dbg!(&part2);
        }
    }
}
