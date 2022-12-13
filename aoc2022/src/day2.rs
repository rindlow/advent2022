use std::fs::read_to_string;

fn score_strategy(filename: &str, score: &[&str]) -> u64 {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| score.iter().position(|g| *g == s).unwrap() as u64 + 1)
        .sum()
}

pub fn score_selected(filename: &str) -> u64 {
    score_strategy(
        filename,
        &[
            "B X", "C Y", "A Z", "A X", "B Y", "C Z", "C X", "A Y", "B Z",
        ],
    )
}

pub fn score_end(filename: &str) -> u64 {
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
    use test_case::test_case;

    #[test_case("../testinput/day2.txt", 15; "on test input")]
    #[test_case("../input/day2.txt", 12586; "on real input")]
    fn part1(filename: &str, expected: u64) {
        assert_eq!(score_selected(filename), expected);
    }
    #[test_case("../testinput/day2.txt", 12; "on test input")]
    #[test_case("../input/day2.txt", 13193; "on real input")]
    fn part2(filename: &str, expected: u64) {
        assert_eq!(score_end(filename), expected);
    }
}
