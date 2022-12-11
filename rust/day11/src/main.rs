use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
enum Op {
    Sq,
    Mul(i64),
    Add(i64),
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: VecDeque<i64>,
    operation: Op,
    test: i64,
    if_true: i64,
    if_false: i64,
    inspections: i64,
}

fn solve_monkey(monkeys: &mut [Monkey], rounds: i64, divide_by_3: bool) -> i64 {
    let cool_number: i64 = monkeys.iter().map(|m| m.test).product();

    for _r in 0..rounds {
        for cur_idx in 0..monkeys.len() {
            while let Some(old) = monkeys[cur_idx].starting_items.pop_front() {
                monkeys[cur_idx].inspections += 1;
                let mut new = match monkeys[cur_idx].operation {
                    Op::Sq => old * old,
                    Op::Mul(x) => old * x,
                    Op::Add(x) => old + x,
                };
                if divide_by_3 {
                    new /= 3;
                }
                new %= cool_number;
                if new % monkeys[cur_idx].test == 0 {
                    let tmp = monkeys[cur_idx].if_true as usize;
                    monkeys[tmp].starting_items.push_back(new);
                } else {
                    let tmp = monkeys[cur_idx].if_false as usize;
                    monkeys[tmp].starting_items.push_back(new);
                }
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product::<i64>()
}

fn main() {
    let input = include_str!("../../../input/day11.txt");

    let number = Regex::new(r"\d+").unwrap();
    let op_regex = Regex::new(r"([+*])\s*(\d+)").unwrap();

    let monkeys = input.split("\n\n");
    let mut monkeys = monkeys
        .map(|m| {
            let mut it = m.split('\n');
            it.next();
            let starting_items = number
                .captures_iter(it.next().unwrap())
                .map(|n| n[0].parse().unwrap())
                .collect();
            let op_line = it.next().unwrap();
            let operation = if op_line.ends_with("old * old") {
                Op::Sq
            } else {
                op_regex
                    .captures(op_line)
                    .map(|m| match &m[1] {
                        "*" => Op::Mul(m[2].parse().unwrap()),
                        "+" => Op::Add(m[2].parse().unwrap()),
                        _ => unreachable!(),
                    })
                    .unwrap()
            };
            let test = number.captures(it.next().unwrap()).unwrap()[0]
                .parse()
                .unwrap();
            let if_true = number.captures(it.next().unwrap()).unwrap()[0]
                .parse()
                .unwrap();
            let if_false = number.captures(it.next().unwrap()).unwrap()[0]
                .parse()
                .unwrap();
            Monkey {
                starting_items,
                operation,
                test,
                if_true,
                if_false,
                inspections: 0,
            }
        })
        .collect_vec();

    let part1 = solve_monkey(&mut monkeys.clone(), 20, true);
    dbg!(&part1);
    let part2 = solve_monkey(&mut monkeys, 10_000, false);
    dbg!(&part2);
}
