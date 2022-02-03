use std::collections::HashMap;

pub fn run() {
    let content = std::fs::read_to_string("input/day20").expect("Unable to read input");
    let (algorithm, image) = parse(&content);

    println!("Day 20");
    println!("  Problem 1: {}", problem1(&algorithm, image.clone()));
    println!("  Problem 2: {}", problem2(&algorithm, image));
}

fn parse(raw: &str) -> ([bool; 512], HashMap<(isize, isize), bool>) {
    let mut lines = raw.lines();

    let mut algorithm = [false; 512];
    for (i, c) in lines.next().unwrap().chars().enumerate() {
        algorithm[i] = c == '#';
    }

    let mut image = HashMap::new();
    lines.next();
    for (row, line) in lines.enumerate() {
        for (col, space) in line.chars().enumerate() {
            if space == '#' {
                image.insert((row as isize, col as isize), true);
            }
        }
    }

    (algorithm, image)
}

fn problem1(algorithm: &[bool], mut image: HashMap<(isize, isize), bool>) -> usize {
    let mut background = false;
    for _ in 0..2 {
        let (next_image, next_background) = step(algorithm, image, background);
        image = next_image;
        background = next_background;
    }

    image.into_values().filter(|b| *b).count()
}

fn problem2(algorithm: &[bool], mut image: HashMap<(isize, isize), bool>) -> usize {
    let mut background = false;
    for _ in 0..50 {
        let (next_image, next_background) = step(algorithm, image, background);
        image = next_image;
        background = next_background;
    }

    image.into_values().filter(|b| *b).count()
}

fn step(algorithm: &[bool], image: HashMap<(isize, isize), bool>, background: bool) -> (HashMap<(isize, isize), bool>, bool) {
    let mut next_image = HashMap::new();
    let index = make_index([background; 9]);
    let next_background = algorithm[index];

    const NEIGHBORHOOD_DELTAS: [(isize, isize); 9] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1), ( 0, 0), ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut to_check = Vec::new();
    for k in image.keys() {
        to_check.push((*k, true));
    }

    while let Some(((row, col), from_image)) = to_check.pop() {
        let mut neighborhood = [false; 9];
        for (i, (dr, dc)) in NEIGHBORHOOD_DELTAS.iter().enumerate() {
            let key = (row + dr, col + dc);
            neighborhood[i] = *image.get(&key).unwrap_or(&background);

            if from_image && !image.contains_key(&key) {
                to_check.push((key, false));
            }
        }
        let index = make_index(neighborhood);
        next_image.insert((row, col), algorithm[index]);
    }

    (next_image, next_background)
}

fn make_index(neighborhood: [bool; 9]) -> usize {
    let mut index = 0;
    for n in neighborhood {
        index <<= 1;
        index |= n as usize;
    }
    index
}

fn display(image: &HashMap<(isize, isize), bool>, background: bool) {
    let mut min_row = isize::MAX;
    let mut max_row = isize::MIN;
    let mut min_col = isize::MAX;
    let mut max_col = isize::MIN;
    for (row, col) in image.keys() {
        min_row = std::cmp::min(min_row, *row);
        max_row = std::cmp::max(max_row, *row);
        min_col = std::cmp::min(min_col, *col);
        max_col = std::cmp::max(max_col, *col);
    }

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            let set = image.get(&(row, col)).unwrap_or(&background);
            if *set {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn example1() {
        let (algorithm, image) = parse(SAMPLE);
        assert_eq!(problem1(&algorithm, image), 35);
    }

    #[test]
    fn example2() {
        let (algorithm, image) = parse(SAMPLE);
        assert_eq!(problem2(&algorithm, image), 3351);
    }
}
