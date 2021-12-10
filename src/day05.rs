use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

pub fn run() {
    let content = fs::read_to_string("input/day5").expect("Unable to read input");
    let lines = parse_lines(&content);

    println!("Day 5");
    println!("  Problem 1: {}", problem1(&lines));
    println!("  Problem 2: {}", problem2(&lines));
}

fn parse_lines(raw: &str) -> Vec<Line> {
    raw.lines().filter_map(|l| l.parse().ok()).collect()
}

struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn is_orthogonal(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn points_between(&self) -> Vec<Point> {
        let diff_x = self.p2.x - self.p1.x;
        let diff_y = self.p2.y - self.p1.y;
        let dx = diff_x.signum();
        let dy = diff_y.signum();
        let steps = diff_x.abs().max(diff_y.abs());

        let mut vec = Vec::new();
        for i in 0..=steps {
            vec.push(Point {
                x: self.p1.x + i * dx,
                y: self.p1.y + i * dy,
            });
        }
        vec
    }
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut parts = s.split(" -> ");
        Ok(Self {
            p1: parts.next().ok_or(())?.parse().map_err(|_| ())?,
            p2: parts.next().ok_or(())?.parse().map_err(|_| ())?,
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut parts = s.split(',');
        Ok(Self {
            x: parts.next().ok_or(())?.parse().map_err(|_| ())?,
            y: parts.next().ok_or(())?.parse().map_err(|_| ())?,
        })
    }
}

fn problem1(lines: &[Line]) -> usize {
    let mut counts = HashMap::new();
    for line in lines {
        if !line.is_orthogonal() {
            continue;
        }

        for point in line.points_between().into_iter() {
            let entry = counts.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    counts.into_values().filter(|v| v >= &2).count()
}

fn problem2(lines: &[Line]) -> usize {
    let mut counts = HashMap::new();
    for line in lines {
        for point in line.points_between().into_iter() {
            let entry = counts.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    counts.into_values().filter(|v| v >= &2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn example1() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem1(&lines), 5);
    }

    #[test]
    fn example2() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem2(&lines), 12);
    }
}
