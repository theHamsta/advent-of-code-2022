use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

fn main() {
    let input = include_str!("../../../input/day10.txt");

    let mut instructions = Vec::new();

    for instruction in input.lines() {
        match instruction.split(' ').collect_vec()[..] {
            ["addx", v] => instructions.push(Instruction::Addx(v.parse().unwrap())),
            ["noop"] => instructions.push(Instruction::Noop),
            _ => (),
        }
    }
    let mut x = 1i64;
    let mut cycle = 1i64;
    let mut next_mul20 = 20;
    let mut part1 = 0;

    //let pipeline = Vec::new();
    for instruction in instructions.iter() {
        match &instruction {
            Instruction::Noop => {
                cycle += 1;
            }
            Instruction::Addx(v) => {
                cycle += 2;
                if cycle > next_mul20 {
                    dbg!(&instruction);
                    part1 += x * next_mul20;
                    next_mul20 += 40;
                }
                x += v;
                if cycle == next_mul20 {
                    part1 += x * next_mul20;
                    next_mul20 += 40;
                }
            }
        }
    }
    dbg!(&part1);

    let mut x = 1i64;
    let mut instruction_in_flight = None;
    let mut cycles_left_till_next_effect = 0;
    let mut ip = 0;
    for _ in 0..6 {
        for pos in 0..40 {
            if cycles_left_till_next_effect == 0 {
                match instruction_in_flight {
                    Some(Instruction::Addx(v)) => x += v,
                    _ => (),
                }
                instruction_in_flight = instructions.get(ip).cloned();
                ip += 1;
                match instruction_in_flight {
                    Some(Instruction::Noop) => cycles_left_till_next_effect += 1,
                    Some(Instruction::Addx(_)) => cycles_left_till_next_effect += 2,
                    None => (),
                }
            }
            cycles_left_till_next_effect -= 1;

            if (x - pos).abs() <= 1 {
                //print!("#");
                print!("█");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
