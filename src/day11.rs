use std::collections::HashSet;
use std::fs;

const FLASH_THRESHOLD: u8 = 9;
const DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn run() {
    let content = fs::read_to_string("input/day11").expect("Unable to read input");
    let lines = parse_lines(&content);

    println!("Day 11");
    println!("  Problem 1: {}", problem1(lines.clone()));
    println!("  Problem 2: {}", problem2(lines));
}

fn parse_lines(raw: &str) -> Vec<Vec<u8>> {
    raw.lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

fn problem1(mut lines: Vec<Vec<u8>>) -> u32 {
    let height = lines.len();
    let width = lines[0].len();
    let mut flash_count = 0;

    for _ in 0..100 {
        let mut cascade = Vec::new();
        let mut flashed = HashSet::new();
        for r in 0..height {
            for c in 0..width {
                lines[r][c] += 1;
                if lines[r][c] <= FLASH_THRESHOLD {
                    continue;
                }

                flashed.insert((r, c));
                for (dr, dc) in DELTAS {
                    let new_r = (r as isize).wrapping_add(dr) as usize;
                    let new_c = (c as isize).wrapping_add(dc) as usize;
                    if new_r < height && new_c < width {
                        cascade.push((new_r, new_c));
                    }
                }
            }
        }

        while let Some((r, c)) = cascade.pop() {
            if flashed.contains(&(r, c)) {
                continue;
            }

            lines[r][c] += 1;
            if lines[r][c] <= FLASH_THRESHOLD {
                continue;
            }

            flashed.insert((r, c));
            for (dr, dc) in DELTAS {
                let new_r = (r as isize).wrapping_add(dr) as usize;
                let new_c = (c as isize).wrapping_add(dc) as usize;
                if new_r < height && new_c < width {
                    cascade.push((new_r, new_c));
                }
            }
        }

        for r in 0..height {
            for c in 0..width {
                if lines[r][c] > FLASH_THRESHOLD {
                    lines[r][c] = 0;
                    flash_count += 1;
                }
            }
        }
    }

    flash_count
}

fn problem2(mut lines: Vec<Vec<u8>>) -> usize {
    let height = lines.len();
    let width = lines[0].len();

    for step in 1.. {
        let mut cascade = Vec::new();
        let mut flashed = HashSet::new();
        for r in 0..height {
            for c in 0..width {
                lines[r][c] += 1;
                if lines[r][c] <= FLASH_THRESHOLD {
                    continue;
                }

                flashed.insert((r, c));
                for (dr, dc) in DELTAS {
                    let new_r = (r as isize).wrapping_add(dr) as usize;
                    let new_c = (c as isize).wrapping_add(dc) as usize;
                    if new_r < height && new_c < width {
                        cascade.push((new_r, new_c));
                    }
                }
            }
        }

        while let Some((r, c)) = cascade.pop() {
            if flashed.contains(&(r, c)) {
                continue;
            }

            lines[r][c] += 1;
            if lines[r][c] <= FLASH_THRESHOLD {
                continue;
            }

            flashed.insert((r, c));
            for (dr, dc) in DELTAS {
                let new_r = (r as isize).wrapping_add(dr) as usize;
                let new_c = (c as isize).wrapping_add(dc) as usize;
                if new_r < height && new_c < width {
                    cascade.push((new_r, new_c));
                }
            }
        }

        if flashed.len() == height * width {
            return step;
        }

        for r in 0..height {
            for c in 0..width {
                if lines[r][c] > FLASH_THRESHOLD {
                    lines[r][c] = 0;
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn example1() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem1(lines), 1656);
    }

    #[test]
    fn example2() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem2(lines), 195);
    }
}
