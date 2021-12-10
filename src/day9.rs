use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day9").expect("Unable to read input");
    let lines = parse_lines(&content);

    println!("Day 9");
    println!("  Problem 1: {}", problem1(&lines));
    println!("  Problem 2: {}", problem2(&lines));
}

fn parse_lines(raw: &str) -> Vec<Vec<u8>> {
    let mut parsed = Vec::new();
    for line in raw.lines() {
        parsed.push(line.chars().map(|c| c as u8 - b'0').collect());
    }

    parsed
}

fn problem1(rows: &[Vec<u8>]) -> u32 {
    let mut minima = Vec::new();
    let height = rows.len();
    let width = rows[0].len();
    for r in 0..height {
        for c in 0..width {
            let v = rows[r][c];
            if c > 0 && v >= rows[r][c - 1] {
                continue;
            }
            if c < width - 1 && v >= rows[r][c + 1] {
                continue;
            }
            if r > 0 && v >= rows[r - 1][c] {
                continue;
            }
            if r < height - 1 && v >= rows[r + 1][c] {
                continue;
            }
            minima.push(v);
        }
    }

    minima.into_iter().map(|m| m as u32 + 1).sum()
}

fn problem2(lines: &[Vec<u8>]) -> u32 {
    const BARRIER: u8 = 9;
    let mut locations: Vec<Vec<Location>> = Vec::new();
    for line in lines {
        let mapped = line
            .iter()
            .map(|v| {
                if v == &BARRIER {
                    Location::Barrier
                } else {
                    Location::Unvisited
                }
            })
            .collect();
        locations.push(mapped);
    }

    let height = locations.len();
    let width = locations[0].len();

    let mut current_basin = 0;
    for r in 0..height {
        for c in 0..width {
            if flood_fill(&mut locations, r, c, width, height, current_basin) {
                current_basin += 1;
            }
        }
    }

    let mut counts = vec![0u32; current_basin];

    for row in locations.into_iter() {
        for col in row.into_iter() {
            if let Location::Basin(n) = col {
                counts[n] += 1;
            }
        }
    }

    counts.sort_unstable();
    counts.iter().rev().take(3).product()
}

#[derive(PartialEq, Eq)]
enum Location {
    Unvisited,
    Barrier,
    Basin(usize),
}

fn flood_fill(
    locations: &mut Vec<Vec<Location>>,
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    basin: usize,
) -> bool {
    let mut to_check = vec![(row, col)];
    let mut filled = false;

    while let Some((row, col)) = to_check.pop() {
        if locations[row][col] == Location::Unvisited {
            locations[row][col] = Location::Basin(basin);
            filled = true;

            if row > 0 {
                to_check.push((row - 1, col));
            }
            if row < height - 1 {
                to_check.push((row + 1, col));
            }

            if col > 0 {
                to_check.push((row, col - 1));
            }
            if col < width - 1 {
                to_check.push((row, col + 1));
            }
        }
    }

    filled
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn example1() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem1(&lines), 15);
    }

    #[test]
    fn example2() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem2(&lines), 1134);
    }
}
