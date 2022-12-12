use itertools::Itertools;
use priority_queue::PriorityQueue;

fn get<'a, T>(map: &'a [&'a [T]], idx: (i32, i32)) -> &'a T {
    &map[idx.0 as usize][idx.1 as usize]
}

fn get_vec<T>(map: &Vec<Vec<T>>, idx: (i32, i32)) -> &T {
    &map[idx.0 as usize][idx.1 as usize]
}
fn get_vec_mut<T>(map: &mut Vec<Vec<T>>, idx: (i32, i32)) -> &mut T {
    &mut map[idx.0 as usize][idx.1 as usize]
}

fn move_ok(a: (i32, i32), delta: (i32, i32), map: &[&[u8]]) -> bool {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    if a.0 + delta.0 >= 0 && a.0 + delta.0 < height && a.1 + delta.1 >= 0 && a.1 + delta.1 < width {
        let from_height = map[a.0 as usize][a.1 as usize];
        let to_height = map[(a.0 + delta.0) as usize][(a.1 + delta.1) as usize];
        if from_height == b'S' || (to_height == b'E' && from_height == b'z') {
            return true;
        }
        if to_height == b'E' {
            return false;
        }
        if (to_height as i64 - from_height as i64) <= 1 {
            return true;
        }
    }
    false
}

fn neighbors(pos: (i32, i32), map: &[&[u8]]) -> Vec<(i32, i32)> {
    let mut rtn = Vec::new();

    for next in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if move_ok(pos, next, map) {
            rtn.push((pos.0 + next.0, pos.1 + next.1));
        }
    }
    rtn
}

fn dijkstra(map: &[&[u8]], start_pos: (usize, usize)) -> Option<i64> {
    let mut shortest_path = map
        .iter()
        .map(|&l| l.iter().map(|_| None).collect_vec())
        .collect_vec();
    shortest_path[start_pos.0][start_pos.1] = Some(0i64);
    let mut next_tiles = PriorityQueue::new();
    next_tiles.push((start_pos.0 as i32, start_pos.1 as i32), 0);
    //let mut goal = None;
    let mut distance = None;

    while let Some((current_pos, path_length)) = next_tiles.pop() {
        let path_length = -path_length;
        if *get(&map, current_pos) == b'E' {
            //goal = Some(current_pos);
            distance = Some(path_length);
            next_tiles.clear();
            break;
        }
        for neighbor in neighbors(current_pos, &map) {
            let neighbor_length = get_vec(&shortest_path, neighbor);
            if neighbor_length.is_none() || path_length + 1 < neighbor_length.unwrap() {
                next_tiles.push(neighbor, -(path_length + 1));
                *get_vec_mut(&mut shortest_path, neighbor) = Some(path_length + 1);
                //*get_vec_mut(&mut predecessor, neighbor) = Some(current_pos);
            }
        }
    }
    distance
}

fn main() {
    let input = include_str!("../../../input/day12.txt");

    let map = input.lines().map(|l| l.as_bytes()).collect_vec();

    let height = map.len();
    let width = map[0].len();
    let start_pos = (0..height)
        .cartesian_product(0..width)
        .find(|(y, x)| map[*y][*x] == b'S')
        .unwrap();

    let part1 = dijkstra(&map, start_pos);
    dbg!(&part1);

    // Too lazy to changes rules for single search from 'E' to 'a'
    let part2 = (0..height)
        .cartesian_product(0..width)
        .filter(|&(y, x)| map[y][x] == b'a')
        .flat_map(|(y, x)| dijkstra(&map, (y, x)))
        .min()
        .unwrap();
    dbg!(&part2);
}
