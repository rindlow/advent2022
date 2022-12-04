use std::{collections::HashSet, fs::read_to_string};

fn helper<F>(filename: &str, f: F) -> u64
where
    F: Fn(HashSet<i32>, HashSet<i32>) -> bool,
{
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut pair = line.split(',').map(|range| {
                let mut sections = range.split('-').map(|i| i.parse::<i32>().unwrap());
                (sections.next().unwrap()..=sections.next().unwrap()).collect::<HashSet<i32>>()
            });
            f(pair.next().unwrap(), pair.next().unwrap())
        })
        .filter(|b| *b)
        .count() as u64
}

pub fn fully_contain(filename: &str) -> u64 {
    helper(filename, |a, b| a.is_subset(&b) || a.is_superset(&b))
}

pub fn overlap(filename: &str) -> u64 {
    helper(filename, |a, b| !a.is_disjoint(&b))
}

#[cfg(test)]
mod tests {
    use crate::day4::{fully_contain, overlap};

    #[test]
    pub fn part1() {
        assert_eq!(2, fully_contain("testinput/day4.txt"));
        assert_eq!(538, fully_contain("input/day4.txt"));
    }
    #[test]
    pub fn part2() {
        assert_eq!(4, overlap("testinput/day4.txt"));
        assert_eq!(792, overlap("input/day4.txt"));
    }
}
