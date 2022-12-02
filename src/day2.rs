#![allow(clippy::identity_op)]

use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug)]
struct Guide {
    first: u8,
    second: u8,
}

fn parse_file(filename: &str) -> Vec<Guide> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let b = s.as_bytes();
            Guide {
                first: b[0],
                second: b[2],
            }
        })
        .collect_vec()
}

pub fn score_strategy(filename: &str) -> i32 {
    parse_file(filename)
        .iter()
        .map(|guide| match guide {
            g if g.first == b'A' && g.second == b'Z' => 3 + 0,
            g if g.first == b'B' && g.second == b'X' => 1 + 0,
            g if g.first == b'C' && g.second == b'Y' => 2 + 0,
            g if g.first == b'A' && g.second == b'X' => 1 + 3,
            g if g.first == b'B' && g.second == b'Y' => 2 + 3,
            g if g.first == b'C' && g.second == b'Z' => 3 + 3,
            g if g.first == b'A' && g.second == b'Y' => 2 + 6,
            g if g.first == b'B' && g.second == b'Z' => 3 + 6,
            g if g.first == b'C' && g.second == b'X' => 1 + 6,
            _ => panic!("Illegal strategy"),
        })
        .sum()
}

pub fn score_strategy_2(filename: &str) -> i32 {
    parse_file(filename)
        .iter()
        .map(|guide| match guide {
            g if g.first == b'A' && g.second == b'X' => 3 + 0,
            g if g.first == b'B' && g.second == b'X' => 1 + 0,
            g if g.first == b'C' && g.second == b'X' => 2 + 0,
            g if g.first == b'A' && g.second == b'Y' => 1 + 3,
            g if g.first == b'B' && g.second == b'Y' => 2 + 3,
            g if g.first == b'C' && g.second == b'Y' => 3 + 3,
            g if g.first == b'A' && g.second == b'Z' => 2 + 6,
            g if g.first == b'B' && g.second == b'Z' => 3 + 6,
            g if g.first == b'C' && g.second == b'Z' => 1 + 6,
            _ => panic!("Illegal strategy"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::{score_strategy, score_strategy_2};

    #[test]
    fn part1() {
        assert_eq!(15, score_strategy("testinput/day2.txt"));
        assert_eq!(12586, score_strategy("input/day2.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(12, score_strategy_2("testinput/day2.txt"));
        assert_eq!(13193, score_strategy_2("input/day2.txt"));
    }
}
