use itertools::Itertools;
use std::fs::read_to_string;

fn common_chars(s1: &str, s2: &str) -> String {
    s1.chars()
        .filter(|c| s2.contains(*c))
        .sorted()
        .dedup()
        .collect()
}

fn priority(s: &str) -> u32 {
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
            priority(&common_chars(&line[(..l)], &line[(l..)]))
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
            priority(&common_chars(
                &common_chars(chunk.first().unwrap(), chunk.get(1).unwrap()),
                chunk.last().unwrap(),
            ))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day3::{badges, rucksack};
    use test_case::test_case;

    #[test_case("../testinput/day3.txt", 157; "on test input")]
    #[test_case("../input/day3.txt", 7826; "on real input")]
    fn part1(filename: &str, expected: u32) {
        assert_eq!(rucksack(filename), expected);
    }
    #[test_case("../testinput/day3.txt", 70; "on test input")]
    #[test_case("../input/day3.txt", 2577; "on real input")]
    fn part2(filename: &str, expected: u32) {
        assert_eq!(badges(filename), expected);
    }
}
