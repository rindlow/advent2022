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
    use test_case::test_case;

    #[test_case("../testinput/day1.txt", 24000; "on test input")]
    #[test_case("../input/day1.txt", 69912; "on real input")]
    fn part1(filename: &str, expected: i32) {
        assert_eq!(most_calories_from_file(filename), expected);
    }
    #[test_case("../testinput/day1.txt", 45000; "on test input")]
    #[test_case("../input/day1.txt", 208_180; "on real input")]
    fn part2(filename: &str, expected: i32) {
        assert_eq!(three_most_calories_from_file(filename), expected);
    }
}
