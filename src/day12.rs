use std::fs;

const START_NAME: &str = "start";
const END_NAME: &str = "end";

pub fn run() {
    let content = fs::read_to_string("input/day12").expect("Unable to read input");
    let tunnels = parse_lines(&content);

    println!("Day 12");
    println!("  Problem 1: {}", problem1(&tunnels));
    println!("  Problem 2: {}", problem2(&tunnels));
}

fn parse_lines(raw: &str) -> Vec<Tunnel> {
    let mut tunnels = Vec::new();
    for line in raw.lines() {
        let mut parts = line.split('-');
        let cave1 = Cave::new(parts.next().unwrap());
        let cave2 = Cave::new(parts.next().unwrap());

        tunnels.push(Tunnel {
            source: cave1.clone(),
            dest: cave2.clone(),
        });

        tunnels.push(Tunnel {
            source: cave2,
            dest: cave1,
        });
    }

    tunnels
}

struct Tunnel {
    source: Cave,
    dest: Cave,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Cave {
    name: String,
    is_big: bool,
}

impl Cave {
    fn new(s: &str) -> Self {
        Self {
            name: s.to_string(),
            is_big: s.chars().next().unwrap().is_ascii_uppercase(),
        }
    }

    fn is_start(&self) -> bool {
        self.name == START_NAME
    }

    fn is_end(&self) -> bool {
        self.name == END_NAME
    }
}

fn count_paths<'a>(
    current_cave: &'a Cave,
    tunnels: &'a [Tunnel],
    visited: &mut Vec<&'a Cave>,
    can_revisit_small: bool,
) -> usize {
    if current_cave.is_end() {
        return 1;
    }

    let mut count = 0;
    visited.push(current_cave);

    for tunnel in tunnels.iter().filter(|t| t.source == *current_cave) {
        if tunnel.dest.is_start() {
            continue;
        }

        let mut next_can_revisit = can_revisit_small;
        if !tunnel.dest.is_big && visited.iter().any(|&v| v == &tunnel.dest) {
            if can_revisit_small {
                next_can_revisit = false;
            } else {
                continue;
            }
        }

        count += count_paths(&tunnel.dest, tunnels, visited, next_can_revisit);
    }

    visited.pop();
    count
}

fn problem1(tunnels: &[Tunnel]) -> usize {
    let start_cave = Cave::new(START_NAME);
    let mut visited = Vec::new();
    count_paths(&start_cave, tunnels, &mut visited, false)
}

fn problem2(tunnels: &[Tunnel]) -> usize {
    let start_cave = Cave::new(START_NAME);
    let mut visited = Vec::new();
    count_paths(&start_cave, tunnels, &mut visited, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &'static str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const SAMPLE2: &'static str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const SAMPLE3: &'static str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn example1_sample1() {
        let tunnels = parse_lines(SAMPLE1);
        assert_eq!(problem1(&tunnels), 10);
    }

    #[test]
    fn example1_sample2() {
        let tunnels = parse_lines(SAMPLE2);
        assert_eq!(problem1(&tunnels), 19);
    }

    #[test]
    fn example1_sample3() {
        let tunnels = parse_lines(SAMPLE3);
        assert_eq!(problem1(&tunnels), 226);
    }

    #[test]
    fn example2_sample1() {
        let tunnels = parse_lines(SAMPLE1);
        assert_eq!(problem2(&tunnels), 36);
    }

    #[test]
    fn example2_sample2() {
        let tunnels = parse_lines(SAMPLE2);
        assert_eq!(problem2(&tunnels), 103);
    }

    #[test]
    fn example2_sample3() {
        let tunnels = parse_lines(SAMPLE3);
        assert_eq!(problem2(&tunnels), 3509);
    }
}
