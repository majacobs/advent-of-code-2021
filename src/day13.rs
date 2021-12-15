use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day13").expect("Unable to read input");
    let (dots, folds) = parse_lines(&content);

    println!("Day 13");
    println!("  Problem 1: {}", problem1(dots.clone(), &folds));
    println!("  Problem 2:\n{}", problem2(dots, &folds));
}

fn parse_lines(raw: &str) -> (Vec<Dot>, Vec<Fold>) {
    let mut lines = raw.lines();

    let mut dots = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(',');
        dots.push(Dot {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
        });
    }

    let mut folds = Vec::new();
    for line in lines {
        if let Some(x) = line.strip_prefix("fold along x=") {
            folds.push(Fold::X(x.parse().unwrap()));
        } else if let Some(y) = line.strip_prefix("fold along y=") {
            folds.push(Fold::Y(y.parse().unwrap()));
        }
    }

    (dots, folds)
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
struct Dot {
    x: usize,
    y: usize,
}

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn apply(&self, dot: &mut Dot) {
        match &self {
            Fold::X(x) => dot.x = x - (dot.x as isize - *x as isize).abs() as usize,
            Fold::Y(y) => dot.y = y - (dot.y as isize - *y as isize).abs() as usize,
        }
    }
}

fn problem1(mut dots: Vec<Dot>, folds: &[Fold]) -> usize {
    let fold = folds.first().unwrap();
    for dot in dots.iter_mut() {
        fold.apply(dot);
    }

    dots.sort_unstable();
    dots.dedup();
    dots.len()
}

fn problem2(mut dots: Vec<Dot>, folds: &[Fold]) -> String {
    for fold in folds {
        for dot in dots.iter_mut() {
            fold.apply(dot);
        }
    }

    let width = dots.iter().map(|d| d.x).max().unwrap_or(0) + 1;
    let height = dots.iter().map(|d| d.y).max().unwrap_or(0) + 1;
    let mut grid = vec![vec![' '; width]; height];
    for dot in dots.iter() {
        grid[dot.y][dot.x] = '#';
    }

    let mut output = String::with_capacity(height * (width + 1));
    for line in grid.into_iter() {
        for c in line.into_iter() {
            output.push(c);
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn example1() {
        let (dots, folds) = parse_lines(SAMPLE);
        assert_eq!(problem1(dots, &folds), 17);
    }

    #[test]
    fn example2() {
        let (dots, folds) = parse_lines(SAMPLE);
        assert_eq!(
            problem2(dots, &folds),
            "\
#####
#   #
#   #
#   #
#####
"
        );
    }
}
