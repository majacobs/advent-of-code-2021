use std::fs;
use std::str::FromStr;

pub fn run() {
    let content = fs::read_to_string("input/day2").expect("Unable to read input");
    let commands = parse_lines(&content);

    println!("Day 2");
    println!("  Problem 1: {}", problem1(&commands));
    println!("  Problem 2: {}", problem2(&commands));
}

fn parse_lines(data: &str) -> Vec<Command> {
    data.lines().filter_map(|l| l.parse().ok()).collect()
}

fn problem1(commands: &[Command]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        match command {
            Command::Forward(n) => horizontal += n,
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n,
        }
    }

    horizontal * depth
}

fn problem2(commands: &[Command]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        match command {
            Command::Forward(n) => {
                horizontal += n;
                depth += aim * n;
            }
            Command::Up(n) => aim -= n,
            Command::Down(n) => aim += n,
        }
    }

    horizontal * depth
}

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let direction = parts.next();
        let amount = parts.next().and_then(|a| a.parse().ok());
        match direction.zip(amount) {
            Some(("forward", n)) => Ok(Self::Forward(n)),
            Some(("up", n)) => Ok(Self::Up(n)),
            Some(("down", n)) => Ok(Self::Down(n)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn example1() {
        let commands = parse_lines(SAMPLE);
        assert_eq!(problem1(&commands), 150);
    }

    #[test]
    fn example2() {
        let commands = parse_lines(SAMPLE);
        assert_eq!(problem2(&commands), 900);
    }
}
