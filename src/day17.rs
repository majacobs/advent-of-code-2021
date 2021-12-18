use std::fs;
use std::ops::RangeInclusive;

pub fn run() {
    let content = fs::read_to_string("input/day17").expect("Unable to read input");
    let (x, y) = parse(content.trim_end());

    println!("Day 17");
    println!("  Problem 1: {}", problem1(x.clone(), y.clone()));
    println!("  Problem 2: {}", problem2(x, y));
}

fn parse(s: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let s = s.strip_prefix("target area: ").unwrap();
    let mut parts = s.split(", ");

    let mut x_parts = parts
        .next()
        .unwrap()
        .strip_prefix("x=")
        .unwrap()
        .split("..");
    let mut y_parts = parts
        .next()
        .unwrap()
        .strip_prefix("y=")
        .unwrap()
        .split("..");

    let min_x = x_parts.next().unwrap().parse().unwrap();
    let max_x = x_parts.next().unwrap().parse().unwrap();
    let min_y = y_parts.next().unwrap().parse().unwrap();
    let max_y = y_parts.next().unwrap().parse().unwrap();

    (min_x..=max_x, min_y..=max_y)
}

fn problem1(target_x: RangeInclusive<i32>, target_y: RangeInclusive<i32>) -> i32 {
    let mut min_dx = 1;
    let max_dx = *target_x.end();
    while !target_x.contains(&sum_to(min_dx)) {
        min_dx += 1;
    }

    let mut overall_max_y = i32::MIN;
    for original_dx in min_dx..=max_dx {
        for original_dy in 0..1000 {
            let mut dx = original_dx;
            let mut dy = original_dy;
            let mut x = 0;
            let mut y = 0;
            let mut max_y = 0;
            while &x <= target_x.end() && &y >= target_y.start() {
                x += dx;
                y += dy;

                if y > max_y {
                    max_y = y;
                }

                if target_x.contains(&x) && target_y.contains(&y) {
                    if max_y > overall_max_y {
                        overall_max_y = max_y;
                    }
                    break;
                }

                if dx > 0 {
                    dx -= 1;
                }
                dy -= 1;
            }
        }
    }

    overall_max_y
}

fn problem2(target_x: RangeInclusive<i32>, target_y: RangeInclusive<i32>) -> usize {
    let mut min_dx = 1;
    let max_dx = *target_x.end();
    while !target_x.contains(&sum_to(min_dx)) {
        min_dx += 1;
    }

    let mut successes = Vec::new();
    for original_dx in min_dx..=max_dx {
        for original_dy in -1000..1000 {
            let mut dx = original_dx;
            let mut dy = original_dy;
            let mut x = 0;
            let mut y = 0;
            let mut max_y = 0;
            while &x <= target_x.end() && &y >= target_y.start() {
                x += dx;
                y += dy;

                if y > max_y {
                    max_y = y;
                }

                if target_x.contains(&x) && target_y.contains(&y) {
                    successes.push((original_dx, original_dy));
                    break;
                }

                if dx > 0 {
                    dx -= 1;
                }
                dy -= 1;
            }
        }
    }

    successes.len()
}

fn sum_to(n: i32) -> i32 {
    n * (n + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn example1() {
        let (x, y) = parse(SAMPLE);
        assert_eq!(problem1(x, y), 45);
    }

    #[test]
    fn example2() {
        let (x, y) = parse(SAMPLE);
        assert_eq!(problem2(x, y), 112);
    }
}
