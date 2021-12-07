use std::fs;
use std::mem;

pub fn run() {
    let content = fs::read_to_string("input/day7").expect("Unable to read input");
    let input = parse_input(&content);

    println!("Day 7");
    println!("  Problem 1: {}", problem1(&input));
    println!("  Problem 2: {}", problem2(&input));
}

fn parse_input(raw: &str) -> Vec<i32> {
    raw.trim_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn problem1(input: &[i32]) -> i32 {
    let mut input: Vec<_> = input.iter().collect();
    input.sort();

    let median = input[input.len() / 2];
    input.into_iter().map(|i| (median - i).abs()).sum()
}

fn problem2(input: &[i32]) -> i32 {
    fn calc_use(n: i32) -> i32 {
        n * (n + 1) / 2
    }

    let max = *input.iter().max().unwrap();
    let mut best_fuel_use = i32::MAX;
    for candidate in 0..=max {
        let mut fuel_use = 0;
        for value in input {
            fuel_use += calc_use((value - candidate).abs());
        }

        if fuel_use < best_fuel_use {
            best_fuel_use = fuel_use;
        }
    }

    best_fuel_use
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example1() {
        let input = parse_input(SAMPLE);
        assert_eq!(problem1(&input), 37);
    }

    #[test]
    fn example2() {
        let input = parse_input(SAMPLE);
        assert_eq!(problem2(&input), 168);
    }
}
