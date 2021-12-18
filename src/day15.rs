use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day15").expect("Unable to read input");
    let map = parse_lines(&content);

    println!("Day 15");
    println!("  Problem 1: {}", problem1(&map));
    println!("  Problem 2: {}", problem2(&map));
}

fn parse_lines(raw: &str) -> Vec<Vec<u32>> {
    let mut map = Vec::new();
    for line in raw.lines() {
        map.push(line.chars().map(|c| (c as u8 - b'0') as u32).collect());
    }

    map
}

fn problem1(map: &[Vec<u32>]) -> u32 {
    dijkstra(map)
}

fn problem2(map: &[Vec<u32>]) -> u32 {
    const COPIES: u32 = 5;
    let mut embiggened_map: Vec<Vec<u32>> = Vec::new();
    for row_copy in 0..COPIES {
        for row in map.iter() {
            let mut embiggened_row = Vec::new();
            for col_copy in 0..COPIES {
                for col in row.iter() {
                    let risk = (row_copy + col_copy + *col as u32 - 1) % 9 + 1;
                    embiggened_row.push(risk);
                }
            }
            embiggened_map.push(embiggened_row);
        }
    }

    dijkstra(&embiggened_map)
}

fn dijkstra(map: &[Vec<u32>]) -> u32 {
    let height = map.len();
    let width = map[0].len();

    let mut best_risk: Vec<Vec<u32>> = (0..height)
        .map(|_| (0..width).map(|_| u32::MAX).collect())
        .collect();
    best_risk[0][0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        row: 0,
        col: 0,
        total_risk: 0,
    });

    while let Some(State {
        row,
        col,
        total_risk,
    }) = heap.pop()
    {
        if row == height - 1 && col == width - 1 {
            return total_risk;
        }

        if total_risk > best_risk[row][col] {
            continue;
        }

        const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in DELTAS {
            let next_row = (row as isize) + dr;
            let next_col = (col as isize) + dc;

            if next_row < 0
                || next_row >= height as isize
                || next_col < 0
                || next_col >= width as isize
            {
                continue;
            }

            let next = State {
                row: next_row as usize,
                col: next_col as usize,
                total_risk: total_risk + map[next_row as usize][next_col as usize],
            };
            if next.total_risk < best_risk[next.row][next.col] {
                best_risk[next.row][next.col] = next.total_risk;
                heap.push(next);
            }
        }
    }

    unreachable!()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    row: usize,
    col: usize,
    total_risk: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .total_risk
            .cmp(&self.total_risk)
            .then_with(|| (self.row, self.col).cmp(&(other.row, other.col)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn example1() {
        let map = parse_lines(SAMPLE);
        assert_eq!(problem1(&map), 40);
    }

    #[test]
    fn example2() {
        let map = parse_lines(SAMPLE);
        assert_eq!(problem2(&map), 315);
    }
}
