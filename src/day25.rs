use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day25").expect("Unable to read input");
    let seafloor = parse(&content);

    println!("Day 25");
    println!("  Problem 1: {}", problem1(seafloor));
}

fn parse(raw: &str) -> Vec<Vec<State>> {
    let mut parsed = Vec::new();
    for line in raw.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let state = match c {
                '.' => State::Empty,
                '>' => State::EastMover,
                'v' => State::SouthMover,
                _ => unreachable!(),
            };
            row.push(state);
        }
        parsed.push(row);
    }
    parsed
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Empty,
    EastMover,
    SouthMover,
}

fn step(seafloor: &[Vec<State>]) -> (Vec<Vec<State>>, bool) {
    let height = seafloor.len();
    let width = seafloor[0].len();

    fn replace_eastmover(prev: State, curr: State, next: State, movement: &mut bool) -> State {
        match (prev, curr, next) {
            (State::EastMover, State::Empty, _) => { *movement = true; State::EastMover },
            (_, State::EastMover, State::Empty) => { *movement = true; State::Empty },
            (_, curr, _) => curr,
        }
    }

    fn replace_southmover(prev: State, curr: State, next: State, movement: &mut bool) -> State {
        match (prev, curr, next) {
            (State::SouthMover, State::Empty, _) => { *movement = true; State::SouthMover },
            (_, State::SouthMover, State::Empty) => { *movement = true; State::Empty },
            (_, curr, _) => curr,
        }
    }

    let mut movement = false;
    let mut after_eastmovers = Vec::new();
    for row in seafloor.iter() {
        let mut shifted = Vec::new();
        shifted.push(replace_eastmover(row[width - 1], row[0], row[1], &mut movement));

        for w in row.windows(3) {
            shifted.push(replace_eastmover(w[0], w[1], w[2], &mut movement));
        }

        shifted.push(replace_eastmover(row[width - 2], row[width - 1], row[0], &mut movement));
        after_eastmovers.push(shifted);
    }

    let mut after_southmovers = Vec::new();
    for _ in 0..height {
        after_southmovers.push(vec![State::Empty; width]);
    }

    for col in 0..width {
        after_southmovers[0][col] = replace_southmover(after_eastmovers[height - 1][col], after_eastmovers[0][col], after_eastmovers[1][col], &mut movement);

        for row in 1..(height - 1) {
            after_southmovers[row][col] = replace_southmover(after_eastmovers[row - 1][col], after_eastmovers[row][col], after_eastmovers[row + 1][col], &mut movement);
        }

        after_southmovers[height - 1][col] = replace_southmover(after_eastmovers[height - 2][col], after_eastmovers[height - 1][col], after_eastmovers[0][col], &mut movement);
    }

    (after_southmovers, movement)
}

fn problem1(mut seafloor: Vec<Vec<State>>) -> usize {
    for count in 1.. {
        let (new_seafloor, movement) = step(&seafloor);
        if !movement {
            return count;
        }
        seafloor = new_seafloor;
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn example1() {
        let seafloor = parse(SAMPLE);
        assert_eq!(problem1(seafloor), 58);
    }
}
