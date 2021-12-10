use std::fs;

const BOARD_SIZE: usize = 5;

pub fn run() {
    let content = fs::read_to_string("input/day4").expect("Unable to read input");
    let (calls, boards) = parse_lines(&content);

    println!("Day 4");
    println!("  Problem 1: {}", problem1(calls.clone(), boards.clone()));
    println!("  Problem 2: {}", problem2(calls, boards));
}

#[allow(clippy::needless_range_loop)]
fn parse_lines(raw: &str) -> (Vec<u8>, Vec<Board>) {
    let mut lines = raw.lines();
    let calls = lines
        .next()
        .expect("Calls not in input")
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        let mut rows = [[Value::Uninitialized; BOARD_SIZE]; BOARD_SIZE];
        let mut cols = [[Value::Uninitialized; BOARD_SIZE]; BOARD_SIZE];
        for r in 0..BOARD_SIZE {
            let line = lines.next().expect("Incomplete board");
            for (c, cell) in line.split_ascii_whitespace().enumerate() {
                let parsed = cell.parse().unwrap();
                rows[r][c] = Value::Uncalled(parsed);
                cols[c][r] = Value::Uncalled(parsed);
            }
        }
        let b = Board { rows, cols };
        if b.is_initialized() {
            boards.push(b);
        } else {
            panic!("Unitialized board");
        }
    }

    (calls, boards)
}

#[derive(Clone)]
struct Board {
    pub rows: [[Value; BOARD_SIZE]; BOARD_SIZE],
    pub cols: [[Value; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn is_initialized(&self) -> bool {
        !self
            .rows
            .iter()
            .flatten()
            .any(|&v| v == Value::Uninitialized)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Value {
    Uninitialized,
    Uncalled(u8),
    Called,
}

impl Value {
    fn content(&self) -> Option<u8> {
        match &self {
            Self::Uncalled(n) => Some(*n),
            Self::Called => None,
            Self::Uninitialized => panic!(),
        }
    }

    fn is_called(&self) -> bool {
        self == &Self::Called
    }
}

fn problem1(calls: Vec<u8>, mut boards: Vec<Board>) -> u32 {
    let (board_indices, last_call) = play_bingo(&mut calls.iter(), &mut boards);
    let uncalled_sum = sum_uncalled(&boards[board_indices[0]]);
    uncalled_sum * (last_call as u32)
}

fn problem2(calls: Vec<u8>, mut boards: Vec<Board>) -> u32 {
    let mut call_iter = calls.iter();

    loop {
        let (board_indices, last_call) = play_bingo(&mut call_iter, &mut boards);

        if boards.len() == 1 {
            let uncalled_sum = sum_uncalled(&boards[0]);
            return uncalled_sum * (last_call as u32);
        }

        for (offset, index) in board_indices.into_iter().enumerate() {
            // `remove` shifts elements down.  `board_indices` is already sorted so a zero-based
            // offset is all that's needed.
            boards.remove(index - offset);
        }
    }
}

fn play_bingo<'a, C>(calls: &mut C, boards: &mut [Board]) -> (Vec<usize>, u8)
where
    C: Iterator<Item = &'a u8>,
{
    let mut winners = Vec::new();
    for call in calls {
        for (b, board) in boards.iter_mut().enumerate() {
            let did_row_win = apply_call(&mut board.rows, *call);
            let did_col_win = apply_call(&mut board.cols, *call);

            if did_row_win || did_col_win {
                winners.push(b);
            }
        }

        if !winners.is_empty() {
            return (winners, *call);
        }
    }

    unreachable!()
}

fn apply_call(groups: &mut [[Value; BOARD_SIZE]], called_value: u8) -> bool {
    let mut modified_index = None;
    'outer: for (i, group) in groups.iter_mut().enumerate() {
        for value in group.iter_mut() {
            if *value == Value::Uncalled(called_value) {
                *value = Value::Called;
                modified_index = Some(i);
                break 'outer;
            }
        }
    }

    if let Some(i) = modified_index {
        return groups[i].iter().all(|n| n.is_called());
    }

    false
}

fn sum_uncalled(board: &Board) -> u32 {
    board
        .rows
        .into_iter()
        .map(|row| row.into_iter())
        .flatten()
        .filter_map(|c| c.content())
        .map(|c| c as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn example1() {
        let (calls, boards) = parse_lines(SAMPLE);
        assert_eq!(problem1(calls, boards), 4512);
    }

    #[test]
    fn example2() {
        let (calls, boards) = parse_lines(SAMPLE);
        assert_eq!(problem2(calls, boards), 1924);
    }
}
