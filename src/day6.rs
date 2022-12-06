use std::fs::read_to_string;

use itertools::Itertools;

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

    #[test]
    fn part1() {
        assert_eq!(7, start_of_package("testinput/day6.txt"));
        assert_eq!(1804, start_of_package("input/day6.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(19, start_of_message("testinput/day6.txt"));
        assert_eq!(2508, start_of_message("input/day6.txt"));
    }
}
