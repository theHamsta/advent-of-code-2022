use std::collections::HashMap;

use itertools::Itertools;
fn main() {
    let input = include_str!("../../../input/day08.txt");

    let input: Vec<Vec<i8>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|l| {
            l.trim()
                .as_bytes()
                .iter()
                .map(|&b| (b - b'0') as i8)
                .collect()
        })
        .collect_vec();

    let width = input[0].len();
    let height = input.len();
    let mut visible = HashMap::new();

    for i in 0..height {
        let mut max = -1;
        for j in 0..width {
            let cell = input[i][j];
            if cell > max {
                visible.insert((i, j), cell);
                max = cell;
            }
        }
    }
    for i in 0..height {
        let mut max = -1;
        for j in (0..width).rev() {
            let cell = input[i][j];
            if cell > max {
                visible.insert((i, j), cell);
                max = cell;
            }
        }
    }

    for i in 0..width {
        let mut max = -1;
        for j in 0..height {
            let cell = input[j][i];
            if cell > max {
                visible.insert((j, i), cell);
                max = cell;
            }
        }
    }
    for i in 0..width {
        let mut max = -1;
        for j in (0..height).rev() {
            let cell = input[j][i];
            if cell > max {
                visible.insert((j, i), cell);
                max = cell;
            }
        }
    }
    let part1 = visible.iter().count();
    dbg!(&part1);

    let mut max = 0;
    for i in 0..height {
        for j in 0..width {
            let score = scenic_score(&input, i, j);
            if score > max {
                max = score;
            }
        }
    }
    let part2 = max;
    dbg!(&part2);
}

fn scenic_score(input: &Vec<Vec<i8>>, i: usize, j: usize) -> usize {
    let cell = input[i][j];

    let mut a = 0;
    for i in (i + 1)..input.len() {
        a += 1;
        if input[i][j] >= cell {
            break;
        }
    }

    let mut b = 0;
    for i in (0..i).rev() {
        b += 1;
        if input[i][j] >= cell {
            break;
        }
    }

    let mut c = 0;
    for j in (j + 1)..input[0].len() {
        c += 1;
        if input[i][j] >= cell {
            break;
        }
    }
    let mut d = 0;
    for j in (0..j).rev() {
        d += 1;
        if input[i][j] >= cell {
            break;
        }
    }
    a * b * c * d
}
