use std::collections::HashMap;
use std::ops::AddAssign;

pub fn run() {
    let content = std::fs::read_to_string("input/day21").expect("Unable to read input");
    let (p1_position, p2_position) = parse(&content);

    println!("Day 21");
    println!("  Problem 1: {}", problem1(p1_position as u32, p2_position as u32));
    println!("  Problem 2: {}", problem2(p1_position, p2_position));
}

fn parse(raw: &str) -> (u8, u8) {
    let mut lines = raw.lines();

    let player1_position = lines.next().unwrap().strip_prefix("Player 1 starting position: ").unwrap().parse().unwrap();
    let player2_position = lines.next().unwrap().strip_prefix("Player 2 starting position: ").unwrap().parse().unwrap();

    (player1_position, player2_position)
}


fn problem1(mut player1_position: u32, mut player2_position: u32) -> u32 {
    const LENGTH: u32 = 10;
    const SCORE_LIMIT: u32 = 1000;

    player1_position -= 1;
    player2_position -= 1;

    let mut player1_score = 0;
    let mut player2_score = 0;

    let mut die_rolls = (1..=100).into_iter().cycle();
    let mut roll_count = 0;

    loop {
        let mut steps = 0u32;
        steps += die_rolls.next().unwrap();
        steps += die_rolls.next().unwrap();
        steps += die_rolls.next().unwrap();
        roll_count += 3;

        player1_position += steps;
        player1_position %= LENGTH;
        player1_score += (player1_position + 1) as u32;
        if player1_score >= SCORE_LIMIT {
            break;
        }

        let mut steps = 0u32;
        steps += die_rolls.next().unwrap();
        steps += die_rolls.next().unwrap();
        steps += die_rolls.next().unwrap();
        roll_count += 3;

        player2_position += steps;
        player2_position %= LENGTH;
        player2_score += (player2_position + 1) as u32;
        if player2_score >= SCORE_LIMIT {
            break;
        }
    }

    std::cmp::min(player1_score, player2_score) * roll_count
}

const DIRAC_ROLLS: [u8; 27] = [3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9];

fn problem2(player1_position: u8, player2_position: u8) -> u64 {
    let start_state = State {
        player1_position: player1_position - 1,
        player1_score: 0,
        player2_position: player2_position - 1,
        player2_score: 0,
        is_player1s_turn: true,
    };

    let mut memoized = HashMap::default();

    let counts = get_win_counts(&start_state, &mut memoized);
    std::cmp::max(counts.player1, counts.player2)
}

fn get_win_counts(
    state: &State,
    memoized: &mut HashMap<State, WinCounts>,
) -> WinCounts {
    const LENGTH: u8 = 10;
    const SCORE_LIMIT: u8 = 21;

    if state.player1_score >= SCORE_LIMIT || state.player2_score >= SCORE_LIMIT {
        return WinCounts {
            player1: (state.player1_score >= SCORE_LIMIT).into(),
            player2: (state.player2_score >= SCORE_LIMIT).into(),
        };
    }

    if let Some(counts) = memoized.get(state) {
        return *counts;
    }

    let mut new = WinCounts::default();
    for roll in DIRAC_ROLLS.iter() {
        let mut new_state = state.clone();
        if state.is_player1s_turn {
            new_state.player1_position += roll;
            new_state.player1_position %= LENGTH;
            new_state.player1_score += new_state.player1_position + 1;
        } else {
            new_state.player2_position += roll;
            new_state.player2_position %= LENGTH;
            new_state.player2_score += new_state.player2_position + 1;
        }

        new_state.is_player1s_turn ^= true;
        new += get_win_counts(&new_state, memoized);
    }

    memoized.insert(*state, new);
    new
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    player1_position: u8,
    player1_score: u8,
    player2_position: u8,
    player2_score: u8,
    is_player1s_turn: bool,
}

#[derive(Clone, Copy, Default)]
struct WinCounts {
    player1: u64,
    player2: u64,
}

impl AddAssign for WinCounts {
    fn add_assign(&mut self, rhs: Self) {
        self.player1 += rhs.player1;
        self.player2 += rhs.player2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn example1() {
        let (p1_position, p2_position) = parse(SAMPLE);
        assert_eq!(problem1(p1_position as u32, p2_position as u32), 739785);
    }

    #[test]
    fn example2() {
        let (p1_position, p2_position) = parse(SAMPLE);
        assert_eq!(problem2(p1_position, p2_position), 444356092776315);
    }
}

