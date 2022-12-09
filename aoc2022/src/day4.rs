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

    #[test]
    pub fn part1() {
        assert_eq!(2, fully_contain("../testinput/day4.txt"));
        assert_eq!(538, fully_contain("../input/day4.txt"));
    }
    #[test]
    pub fn part2() {
        assert_eq!(4, overlap("../testinput/day4.txt"));
        assert_eq!(792, overlap("../input/day4.txt"));
    }
}
