use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day10").expect("Unable to read input");
    let lines = parse_lines(&content);

    println!("Day 10");
    println!("  Problem 1: {}", problem1(&lines));
    println!("  Problem 2: {}", problem2(&lines));
}

fn parse_lines(raw: &str) -> Vec<&str> {
    raw.lines().collect()
}

fn problem1(lines: &[&str]) -> u32 {
    let mut illegal = Vec::new();
    for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '[' | '{' | '(' | '<' => stack.push(c),
                ']' => {
                    if stack.pop() != Some('[') {
                        illegal.push(c);
                        break;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        illegal.push(c);
                        break;
                    }
                }
                ')' => {
                    if stack.pop() != Some('(') {
                        illegal.push(c);
                        break;
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        illegal.push(c);
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let mut sum = 0;
    for c in illegal.into_iter() {
        sum += match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    }

    sum
}

fn problem2(lines: &[&str]) -> u64 {
    let mut scores = Vec::new();
    'outer: for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '[' | '{' | '(' | '<' => stack.push(c),
                ']' => {
                    if stack.pop() != Some('[') {
                        continue 'outer;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        continue 'outer;
                    }
                }
                ')' => {
                    if stack.pop() != Some('(') {
                        continue 'outer;
                    }
                }
                '>' => {
                    if stack.pop() != Some('<') {
                        continue 'outer;
                    }
                }
                _ => unreachable!(),
            }
        }

        if stack.is_empty() {
            continue;
        }

        let mut score = 0;
        while let Some(c) = stack.pop() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };
        }

        scores.push(score);
    }

    scores.sort_unstable();
    let index = scores.len() / 2;
    scores[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn example1() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem1(&lines), 26397);
    }

    #[test]
    fn example2() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem2(&lines), 288957);
    }
}
