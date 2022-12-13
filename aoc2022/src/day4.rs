use std::fs::read_to_string;

struct Range {
    first: i32,
    last: i32,
}

fn helper<F>(filename: &str, f: F) -> u64
where
    F: Fn(Range, Range) -> bool,
{
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut pair = line.split(',').map(|range| {
                let mut sections = range.split('-').map(|i| i.parse::<i32>().unwrap());
                Range {
                    first: sections.next().unwrap(),
                    last: sections.next().unwrap(),
                }
            });
            f(pair.next().unwrap(), pair.next().unwrap())
        })
        .filter(|b| *b)
        .count() as u64
}

pub fn fully_contain(filename: &str) -> u64 {
    helper(filename, |a, b| {
        (a.first <= b.first && a.last >= b.last) || (a.first >= b.first && a.last <= b.last)
    })
}

pub fn overlap(filename: &str) -> u64 {
    helper(filename, |a, b| {
        (a.first >= b.first && a.first <= b.last)
            || (a.last >= b.first && a.last <= b.last)
            || (a.first <= b.first && a.last >= b.last)
            || (a.first >= b.first && a.last <= b.last)
    })
}

#[cfg(test)]
mod tests {
    use crate::day4::{fully_contain, overlap};
    use test_case::test_case;

    #[test_case("../testinput/day4.txt", 2; "on test input")]
    #[test_case("../input/day4.txt", 538; "on real input")]
    fn part1(filename: &str, expected: u64) {
        assert_eq!(fully_contain(filename), expected);
    }
    #[test_case("../testinput/day4.txt", 4; "on test input")]
    #[test_case("../input/day4.txt", 792; "on real input")]
    fn part2(filename: &str, expected: u64) {
        assert_eq!(overlap(filename), expected);
    }
}
