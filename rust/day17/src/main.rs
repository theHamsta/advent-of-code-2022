use std::{
    collections::{HashMap, VecDeque},
    iter::Cloned,
    ops::{Index, IndexMut},
    process::exit,
};

fn make_rocks() -> [Vec<(usize, usize)>; 5] {
    let mut rocks = Vec::new();

    rocks.push(vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
    // .#.
    // ###
    // .#.
    rocks.push(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]);
    // ..#
    // ..#
    // ###
    rocks.push(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]);
    rocks.push(vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
    rocks.push(vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
    rocks.try_into().unwrap()
}

const BOARD_WIDTH: usize = 7;
const ROCK_START_X: usize = 2;
const ROCK_DELTA_Y: usize = 4;

#[derive(Debug, Clone)]
struct ForgettingBoard<T> {
    inner: VecDeque<T>,
    forgotten: usize,
    limit: usize,
}

impl<T> ForgettingBoard<T>
where
    T: Clone,
{
    fn new() -> Self {
        Self {
            inner: VecDeque::new(),
            forgotten: 0,
            limit: 50 * BOARD_WIDTH,
        }
    }

    fn resize(&mut self, size: usize, value: T) {
        let virutal_size = self.inner.len() + self.forgotten;
        let to_add = size - virutal_size;
        //for _ in 0..to_add {
            self.inner.resize(self.inner.len() + to_add, value.clone());
        //}
        if self.inner.len() > self.limit {
            let to_forget = self.inner.len() - self.limit;
            self.inner.drain(0..to_forget);
            self.forgotten += to_forget
        }
    }
}

impl<T> Index<usize> for ForgettingBoard<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        //if index < self.forgotten {
        //panic!(
        //"Invalid index: {index:?} (forgotten {}, size {})",
        //self.forgotten,
        //self.inner.len()
        //);
        //}
        &self.inner[index - self.forgotten]
    }
}

impl<T> IndexMut<usize> for ForgettingBoard<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        //if index < self.forgotten {
        //panic!(
        //"Invalid index: {index:?} (forgotten {}, size {})",
        //self.forgotten,
        //self.inner.len()
        //);
        //}
        &mut self.inner[index - self.forgotten]
    }
}

type Board = ForgettingBoard<u8>;
//type Board = Vec<u8>;

fn can_move_here(board: &Board, (rock_x, rock_y): (usize, usize), rock: &[(usize, usize)]) -> bool {
    if rock_x >= BOARD_WIDTH {
        return false;
    }
    for r in rock.iter() {
        if (rock_x + r.0) >= BOARD_WIDTH
            || board[(rock_y + r.1) * BOARD_WIDTH + rock_x + r.0] != b' '
        {
            return false;
        }
    }
    return true;
}

fn advect(
    board: &Board,
    direction: u8,
    pos: (usize, usize),
    rock: &[(usize, usize)],
) -> (usize, usize) {
    let new_pos = match direction {
        b'<' => (pos.0.checked_sub(1).unwrap_or(0), pos.1),
        b'>' => (pos.0 + 1, pos.1),
        c => panic!("Invalid streaming character: {c:?}"),
    };
    if can_move_here(board, new_pos, rock) {
        new_pos
    } else {
        pos
    }
}

fn fall(board: &Board, pos: (usize, usize), rock: &[(usize, usize)]) -> Option<(usize, usize)> {
    let new_pos = (pos.0, pos.1.checked_sub(1)?);
    can_move_here(board, new_pos, rock).then_some(new_pos)
}

fn play_one_round(
    board: &mut Board,
    stream_pattern: &mut impl Iterator<Item = u8>,
    block: &[(usize, usize)],
    height: &mut usize,
    stream_cyle: &mut usize,
    print: bool,
) {
    board.resize((*height + ROCK_DELTA_Y + 10) * BOARD_WIDTH, b' ');
    let mut pos = (ROCK_START_X, *height + ROCK_DELTA_Y);

    while let Some(new_pos) = fall(&board, pos, block) {
        if print {
            print_board(&board, *height + 4, block, pos);
        }
        pos = advect(&board, stream_pattern.next().unwrap(), new_pos, block);
        *stream_cyle += 1;
        if print {
            print_board(&board, *height + 4, block, pos);
        }
    }
    manifest_block(board, block, pos, b'#');

    for i in *height.. {
        if ((i * BOARD_WIDTH)..((i + 1) * BOARD_WIDTH))
            .map(|i| board[i])
            .all(|p| p == b' ')
        {
            break;
        }

        *height = i + 1;
    }
    if print {
        print_board(&board, *height + 4, block, pos);
    }
}

fn play_rock_tetris(stream_pattern: &[u8], number_of_rocks: usize, print: bool) -> usize {
    let blocks = once_cell::unsync::Lazy::new(make_rocks);

    let mut height = 0;
    let mut blocks_it = blocks.iter().cycle();
    let mut stream_pattern_it = stream_pattern.iter().cloned().cycle();
    let mut board = Board::new();
    let mut stream_cycle = 0;

    let mut hedgehog_height = 0;
    let mut hedgehog_blocks = blocks.iter().cycle();
    let mut hedgehog_stream_pattern_it = stream_pattern.iter().cloned().cycle();
    let mut hedgehog_board = Board::new();
    let mut hedgehog_stream_cycle = 0;
    let mut hedgehog_block_cycle = 0;

    let mut bar = pbr::ProgressBar::new(number_of_rocks as u64 / 10000);

    //let mut hedgehog_interesting_chars = None;
    for i in 0..number_of_rocks {
        //if i % 2 == 0 {
        //play_one_round(
        //&mut hedgehog_board,
        //&mut hedgehog_stream_pattern_it,
        //hedgehog_blocks.next().unwrap(),
        //&mut hedgehog_height,
        //&mut hedgehog_stream_cycle,
        //print,
        //);
        //hedgehog_interesting_chars = Some(interesting_chars(&board, height));
        //}
        play_one_round(
            &mut board,
            &mut stream_pattern_it,
            blocks_it.next().unwrap(),
            &mut height,
            &mut stream_cycle,
            print,
        );

        //if stream_cycle == stream_pattern.len() {
        //stream_cycle = 0;
        //}
        //if i != 0 {
        //if hedgehog_block_cycle == i % 5 && hedgehog_stream_cycle == stream_cycle {
        //let interesting_chars = interesting_chars(&board, height);
        //if &interesting_chars == hedgehog_interesting_chars.as_ref().unwrap() {
        //println!("Cycle");

        //exit(0);
        //}
        //}
        //if i % 2 == 0 {
        //hedgehog_block_cycle %= 5;
        //}
        //}
        if i % 10000 == 0 {
            bar.inc();
        }
    }
    bar.finish();

    height
}

fn can_move_to_other_side(
    board: &Board,
    pos: (usize, usize),
    max: usize,
    min: usize,
) -> Option<usize> {
    if pos.0 == BOARD_WIDTH {
        return Some(pos.1.min(min));
    }
    if board[pos.0 + pos.1 * BOARD_WIDTH] == b' ' {
        return None;
    }
    can_move_to_other_side(board, (pos.0 + 1, pos.1), max.max(pos.1), min.min(pos.1)).or_else(
        || {
            (pos.1 > 0)
                .then(|| {
                    can_move_to_other_side(
                        board,
                        (pos.0, pos.1 - 1),
                        max.max(pos.1),
                        min.min(pos.1 - 1),
                    )
                })
                .flatten()
        },
    )
    //.or_else(|| {
    //(pos.1 < max)
    //.then(|| {
    //can_move_to_other_side(
    //board,
    //(pos.0, pos.1 - 1),
    //max.max(pos.1),
    //min.min(pos.1 - 1),
    //)
    //})
    //.flatten()
    //})
}

fn interesting_chars(board: &ForgettingBoard<u8>, height: usize) -> Vec<u8> {
    let mut rtn = Vec::new();
    //let min_line = (0..=height)
    //.rev()
    //.flat_map(|i| can_move_to_other_side(board, (0, i), height, i))
    //.next()
    //.map(|i| i + 1)
    //.unwrap_or(0);
    for y in (height - 5)..height {
        for x in 0..BOARD_WIDTH {
            rtn.push(board[y * BOARD_WIDTH + x]);
        }
    }
    rtn
}

fn manifest_block(
    board: &mut Board,
    block: &[(usize, usize)],
    pos: (usize, usize),
    manifest_char: u8,
) {
    for b in block {
        board[pos.0 + b.0 + (pos.1 + b.1) * BOARD_WIDTH] = manifest_char;
    }
}

fn print_board(board: &Board, height: usize, block: &[(usize, usize)], pos: (usize, usize)) {
    println!("---------------------");
    let mut board = Board::clone(board);
    manifest_block(&mut board, block, pos, b'.');
    for y in (0..height).rev() {
        for x in 0..BOARD_WIDTH {
            print!("{}", board[y * BOARD_WIDTH + x] as char);
        }
        println!();
    }
    println!("---------------------");
}

fn main() {
    let input: Vec<_> = include_str!("../../../input/day17_test.txt")
        .bytes()
        .filter(|&b| b == b'<' || b == b'>')
        .collect();

    let part1 = play_rock_tetris(&input, 2022, false);
    dbg!(&part1);
    let part2 = play_rock_tetris(&input, 100000000, false);
    //let part2 = play_rock_tetris(&input, 1000000000000, false);
    dbg!(&part2);
}
