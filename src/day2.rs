#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

use std::fs::read_to_string;

pub fn score_strategy(filename: &str, score: &[&str]) -> i32 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| score.iter().position(|g| *g == s).unwrap() as i32 + 1)
        .sum()
}

pub fn score_selected(filename: &str) -> i32 {
    score_strategy(
        filename,
        &[
            "B X", "C Y", "A Z", "A X", "B Y", "C Z", "C X", "A Y", "B Z",
        ],
    )
}

pub fn score_end(filename: &str) -> i32 {
    score_strategy(
        filename,
        &[
            "B X", "C X", "A X", "A Y", "B Y", "C Y", "C Z", "A Z", "B Z",
        ],
    )
}

#[cfg(test)]
mod tests {
    use crate::day2::{score_end, score_selected};

    #[test]
    fn part1() {
        assert_eq!(15, score_selected("testinput/day2.txt"));
        assert_eq!(12586, score_selected("input/day2.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(12, score_end("testinput/day2.txt"));
        assert_eq!(13193, score_end("input/day2.txt"));
    }
}
