use itertools::Itertools;

fn parse_file(filename: &str) -> Vec<Vec<i32>> {
    std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .split(|s| s.is_empty())
        .map(|slice| {
            slice
                .iter()
                .map(|string| string.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

pub fn most_calories_from_file(filename: &str) -> i32 {
    parse_file(filename)
        .iter()
        .map(|v| v.iter().sum::<i32>())
        .max()
        .unwrap()
}

pub fn three_most_calories_from_file(filename: &str) -> i32 {
    parse_file(filename)
        .iter()
        .map(|v| v.iter().sum::<i32>())
        .sorted_by(|a, b| Ord::cmp(b, a)) // sort descending
        .collect_vec()
        .get(0..3)
        .unwrap()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{most_calories_from_file, three_most_calories_from_file};

    #[test]
    fn part1() {
        assert_eq!(24_000, most_calories_from_file("../testinput/day1.txt"));
        assert_eq!(69_912, most_calories_from_file("../input/day1.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(
            45_000,
            three_most_calories_from_file("../testinput/day1.txt")
        );
        assert_eq!(208_180, three_most_calories_from_file("../input/day1.txt"));
    }
}