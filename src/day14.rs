use std::collections::HashMap;
use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day14").expect("Unable to read input");
    let (template, rules) = parse_lines(&content);

    println!("Day 14");
    println!("  Problem 1: {}", problem1(&template, &rules));
    println!("  Problem 2: {}", problem2(&template, &rules));
}

fn parse_lines(raw: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut lines = raw.lines();

    let template: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next(); // Skip

    let mut insertion_rules = HashMap::new();
    for line in lines {
        let mut letters = line.chars().filter(char::is_ascii_alphabetic);
        let left = letters.next().unwrap();
        let right = letters.next().unwrap();
        let inserted = letters.next().unwrap();

        insertion_rules.insert((left, right), inserted);
    }

    (template, insertion_rules)
}

fn problem1(template: &[char], rules: &HashMap<(char, char), char>) -> usize {
    polymerize(template, rules, 10)
}

fn problem2(template: &[char], rules: &HashMap<(char, char), char>) -> usize {
    polymerize(template, rules, 40)
}

fn polymerize(template: &[char], rules: &HashMap<(char, char), char>, height: usize) -> usize {
    let mut totals = HashMap::new();
    for c in template {
        let entry = totals.entry(*c).or_insert(0);
        *entry += 1;
    }

    let mut memoized = HashMap::new();
    for w in template.windows(2) {
        add_to_totals(&mut totals, w[0], w[1], height, &mut memoized, rules);
    }

    let mut counts: Vec<usize> = totals.into_values().collect();
    counts.sort_unstable();
    counts.last().unwrap() - counts.first().unwrap()
}

fn add_to_totals(
    totals: &mut HashMap<char, usize>,
    left: char,
    right: char,
    height: usize,
    memoized: &mut HashMap<(char, char, usize), HashMap<char, usize>>,
    rules: &HashMap<(char, char), char>,
) {
    if height == 0 {
        return;
    }

    let key = (left, right, height);
    if let Some(counts) = memoized.get(&key) {
        merge_into(counts, totals);
        return;
    }

    let inserted = *rules.get(&(left, right)).unwrap();

    let mut new = HashMap::new();
    new.insert(inserted, 1);
    add_to_totals(&mut new, left, inserted, height - 1, memoized, rules);
    add_to_totals(&mut new, inserted, right, height - 1, memoized, rules);
    merge_into(&new, totals);

    memoized.insert(key, new);
}

fn merge_into(source: &HashMap<char, usize>, dest: &mut HashMap<char, usize>) {
    for (k, v) in source.iter() {
        let entry = dest.entry(*k).or_insert(0);
        *entry += v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn example1() {
        let (template, rules) = parse_lines(SAMPLE);
        assert_eq!(problem1(&template, &rules), 1588);
    }

    #[test]
    fn example2() {
        let (template, rules) = parse_lines(SAMPLE);
        assert_eq!(problem2(&template, &rules), 2188189693529);
    }
}
