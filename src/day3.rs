#![allow(clippy::needless_pass_by_value)]

use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string};

fn common_chars(s1: &str, s2: &str) -> String {
    s1.chars()
        .collect::<HashSet<char>>()
        .intersection(&s2.chars().collect::<HashSet<char>>())
        .collect()
}

fn priority(s: String) -> u32 {
    assert!(s.len() == 1, "Not exactly one common char");
    match s.chars().next() {
        Some(c) if c >= 'a' => c as u32 - 96,
        Some(c) => c as u32 - 38,
        None => 0,
    }
}

pub fn rucksack(filename: &str) -> u32 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let l = line.len() / 2;
            priority(common_chars(&line[(..l)], &line[(l..)]))
        })
        .sum()
}

pub fn badges(filename: &str) -> u32 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .collect_vec()
        .chunks(3)
        .map(|chunk| {
            priority(common_chars(
                &common_chars(chunk.first().unwrap(), chunk.get(1).unwrap()),
                chunk.last().unwrap(),
            ))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day3::{badges, rucksack};

    #[test]
    fn part1() {
        assert_eq!(157, rucksack("testinput/day3.txt"));
        assert_eq!(7826, rucksack("input/day3.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(70, badges("testinput/day3.txt"));
        assert_eq!(2577, badges("input/day3.txt"));
    }
}
