use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day3").expect("Unable to read input");
    let data = parse_lines(&content);

    println!("Day 3");
    println!("  Problem 1: {}", problem1::<12>(&data));
    println!("  Problem 2: {}", problem2::<12>(&data));
}

fn parse_lines(raw: &str) -> Vec<u32> {
    let mut data: Vec<u32> = raw
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect();
    data.sort();
    data
}

fn problem1<const N: usize>(data: &[u32]) -> u32 {
    let gamma = most_common_bits::<N>(data);
    let mask = (1 << N) - 1;
    let epsilon = gamma ^ mask;

    epsilon * gamma
}

fn most_common_bits<const N: usize>(data: &[u32]) -> u32 {
    // index 0 = least significant bit
    let mut bit_counts = vec![0; N];
    for n in data {
        for i in 0..N {
            bit_counts[i] += (n >> i) & 1;
        }
    }

    let half = (data.len() / 2) as u32;
    let mut most_common = 0;
    for i in 0..N {
        most_common |= ((bit_counts[i] >= half) as u32) << i;
    }
    most_common
}

fn problem2<const N: usize>(data: &[u32]) -> u32 {
    let pivot = 1 << (N - 1);
    let oxygen = binary_search(data, pivot, pivot >> 1, Keep::Common);
    let scrubber = binary_search(data, pivot, pivot >> 1, Keep::Uncommon);

    oxygen * scrubber
}

fn binary_search(data: &[u32], pivot: u32, delta: u32, keep: Keep) -> u32 {
    let index = data.partition_point(|n| n < &pivot);
    let (lesser, greater) = data.split_at(index);

    let more_common = if lesser.len() > greater.len() {
        Common::Lesser
    } else {
        Common::Greater
    };

    let (to_keep, next_pivot) = match (keep, more_common) {
        (Keep::Common, Common::Lesser) => (lesser, pivot - delta),
        (Keep::Uncommon, Common::Greater) => (lesser, pivot - delta),
        (Keep::Common, Common::Greater) => (greater, pivot + delta),
        (Keep::Uncommon, Common::Lesser) => (greater, pivot + delta),
    };

    if to_keep.len() == 1 {
        to_keep[0]
    } else {
        assert!(delta > 0);
        binary_search(to_keep, next_pivot, delta >> 1, keep)
    }
}

#[derive(Copy, Clone)]
enum Keep {
    Common,
    Uncommon,
}

#[derive(Copy, Clone)]
enum Common {
    Lesser,
    Greater,
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn example1() {
        let data = parse_lines(SAMPLE);
        assert_eq!(problem1::<5>(&data), 198);
    }

    #[test]
    fn example2() {
        let data = parse_lines(SAMPLE);
        assert_eq!(problem2::<5>(&data), 230);
    }
}
