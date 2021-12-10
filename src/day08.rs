use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day8").expect("Unable to read input");
    let lines = parse_lines(&content);

    println!("Day 8");
    println!("  Problem 1: {}", problem1(&lines));
    println!("  Problem 2: {}", problem2(&lines));
}

fn parse_lines(raw: &str) -> Vec<Entry> {
    let mut parsed = Vec::new();
    for line in raw.lines() {
        let mut parts = line.split(' ');
        let patterns = parts
            .by_ref()
            .take_while(|s| *s != "|")
            .map(to_bits)
            .collect();
        let outputs = parts.map(to_bits).collect();
        parsed.push(Entry { patterns, outputs });
    }

    parsed
}

struct Entry {
    patterns: Vec<u8>,
    outputs: Vec<u8>,
}

fn problem1(lines: &[Entry]) -> usize {
    lines
        .iter()
        .map(|l| l.outputs.iter())
        .flatten()
        .filter(|s| matches!(s.count_ones(), 2 | 3 | 4 | 7))
        .count()
}

fn problem2(lines: &[Entry]) -> u32 {
    let mut sum = 0;
    for line in lines {
        let one = line.patterns.iter().find(|b| b.count_ones() == 2).unwrap();
        let four = line.patterns.iter().find(|b| b.count_ones() == 4).unwrap();

        let mut values = [0; 128];

        for b in line.patterns.iter() {
            values[*b as usize] = match b.count_ones() {
                2 => 1,
                3 => 7,
                4 => 4,
                5 if b & one == *one => 3,
                5 if (b & four).count_ones() == 3 => 5,
                5 => 2,
                6 if b & one != *one => 6,
                6 if b & four == *four => 9,
                6 => 0,
                7 => 8,
                _ => unreachable!(),
            };
        }

        let mut n = 0u32;
        for output in line.outputs.iter() {
            n = 10 * n + values[*output as usize];
        }
        sum += n;
    }
    sum
}

fn to_bits(s: &str) -> u8 {
    let mut bits = 0;
    for c in s.chars() {
        let shift = c as u8 - b'a';
        bits |= 1 << shift;
    }
    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn example1() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem1(&lines), 26);
    }

    #[test]
    fn example2() {
        let lines = parse_lines(SAMPLE);
        assert_eq!(problem2(&lines), 61229);
    }
}
