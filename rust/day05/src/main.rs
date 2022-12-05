use itertools::Itertools;
use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
struct Instruction {
    howmany: u64,
    from: u64,
    to: u64,
}

fn main() {
    let input = include_str!("../../../input/day05.txt");

    let stack_num = 9;

    let stacks = input
        .lines()
        .take_while(|l| !l.starts_with(" 1"))
        .collect_vec();
    let stacks = stacks.iter().rev().collect_vec();
    dbg!(&stacks);
    let mut stack = vec![Vec::<char>::new(); stack_num];
    for line in stacks.iter() {
        for i in 0..stack_num {
            if (i * 4 + 1) < line.len() {
                let c = line.as_bytes()[i * 4 + 1] as char;
                if c != ' ' {
                    stack[i].push(c);
                }
            }
        }
    }

    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let instructions: Vec<Instruction> = regex
        .captures_iter(input)
        .map(|cap| Instruction {
            howmany: cap[1].parse().unwrap(),
            from: cap[2].parse().unwrap(),
            to: cap[3].parse().unwrap(),
        })
        .collect();

    let mut part1 = stack.clone();
    for instruction in instructions.iter() {
        for _ in 0..instruction.howmany {
            if let Some(from) = part1[instruction.from as usize - 1].pop() {
                part1[instruction.to as usize - 1].push(from);
            } else {
                println!("no more disks to move: {:?} {:?}", instruction, part1);
            }
        }
    }
    let part1: String = part1.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect();
    dbg!(&part1);

    let mut part2 = stack.clone();
    for instruction in instructions.iter() {
        let from = &mut part2[instruction.from as usize - 1];
        let mut tmp = from.split_off(from.len() - instruction.howmany as usize);
        part2[instruction.to as usize - 1].extend(tmp.drain(..));
    }
    let part2: String = part2.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect();
    dbg!(&part2);
}
