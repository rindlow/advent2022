#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]

pub fn increasing_numbers_from_file(filename: &str) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

pub fn increasing_numbers_with_windows_from_file(filename: &str) -> usize {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(4)
        .filter(|w| w[0] < w[3])
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day1::{increasing_numbers_from_file, increasing_numbers_with_windows_from_file};

    #[test]
    fn part1() {
        assert_eq!(7, increasing_numbers_from_file("testinput/day1.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(
            5,
            increasing_numbers_with_windows_from_file("testinput/day1.txt")
        );
    }
}
