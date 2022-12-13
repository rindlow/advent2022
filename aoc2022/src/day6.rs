use itertools::Itertools;
use std::fs::read_to_string;

fn start_of_sequence(filename: &str, size: usize) -> usize {
    read_to_string(filename)
        .unwrap()
        .chars()
        .collect_vec()
        .windows(size)
        .enumerate()
        .find(|(_, window)| window.iter().sorted().dedup().count() == size)
        .unwrap()
        .0
        + size
}

pub fn start_of_package(filename: &str) -> usize {
    start_of_sequence(filename, 4)
}
pub fn start_of_message(filename: &str) -> usize {
    start_of_sequence(filename, 14)
}

#[cfg(test)]
mod tests {
    use crate::day6::{start_of_message, start_of_package};
    use test_case::test_case;

    #[test_case("../testinput/day6.txt", 7; "on test input")]
    #[test_case("../input/day6.txt", 1804; "on real input")]
    fn part1(filename: &str, expected: usize) {
        assert_eq!(start_of_package(filename), expected);
    }
    #[test_case("../testinput/day6.txt", 19; "on test input")]
    #[test_case("../input/day6.txt", 2508; "on real input")]
    fn part2(filename: &str, expected: usize) {
        assert_eq!(start_of_message(filename), expected);
    }
}
