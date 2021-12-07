use std::fs;
use std::mem;

pub fn run() {
    let content = fs::read_to_string("input/day6").expect("Unable to read input");
    let input = parse_input(&content);

    println!("Day 6");
    println!("  Problem 1: {}", simulate(&input, 80));
    println!("  Problem 2: {}", simulate(&input, 256));
}

fn parse_input(raw: &str) -> Vec<usize> {
    raw.trim_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn simulate(input: &[usize], days: usize) -> usize {
    const CYCLE_TIME: usize = 7;
    const BABY_CYCLE_TIME: usize = CYCLE_TIME + 2;
    let mut population = [0; BABY_CYCLE_TIME];

    for i in input.iter() {
        population[*i] += 1;
    }

    for _ in 0..days {
        let mut carryover = 0;
        for i in (0..BABY_CYCLE_TIME).rev() {
            if i > 0 {
                mem::swap(&mut carryover, &mut population[i]);
            } else {
                population[BABY_CYCLE_TIME - 1] = population[0];
                population[CYCLE_TIME - 1] += population[0];
                population[0] = carryover;
            }
        }
    }

    population.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "3,4,3,1,2";

    #[test]
    fn example1() {
        let input = parse_input(SAMPLE);
        assert_eq!(simulate(&input, 18), 26);
        assert_eq!(simulate(&input, 80), 5934);
    }

    #[test]
    fn example2() {
        let input = parse_input(SAMPLE);
        assert_eq!(simulate(&input, 256), 26984457539);
    }
}
