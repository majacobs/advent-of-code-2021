use std::fs;
use std::str::FromStr;

pub fn run() {
    let content = fs::read_to_string("input/day1").expect("Unable to read input");
    let measurements = parse_lines(&content);

    println!("Day 1");
    println!("  Problem 1: {}", problem1(&measurements));
    println!("  Problem 2: {}", problem2(&measurements));
}

fn parse_lines(data: &str) -> Vec<i32> {
    data.split('\n')
        .filter_map(|l| i32::from_str(l).ok())
        .collect()
}

fn problem1(measurements: &[i32]) -> usize {
    measurements.windows(2).filter(|w| w[1] > w[0]).count()
}

fn problem2(measurements: &[i32]) -> usize {
    let sums: Vec<i32> = measurements.windows(3).map(|w| w.iter().sum()).collect();
    sums.windows(2).filter(|w| w[1] > w[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn example1() {
        assert_eq!(problem1(&DATA), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(problem2(&DATA), 5);
    }
}
