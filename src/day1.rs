#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::stable_sort_primitive)]

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
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

pub fn most_calories_from_file(filename: &str) -> i32 {
    parse_file(filename)
        .iter()
        .map(|elf| elf.iter().sum::<i32>())
        .max()
        .unwrap()
}

pub fn three_most_calories_from_file(filename: &str) -> i32 {
    let mut cal = parse_file(filename)
        .iter()
        .map(|elf| elf.iter().sum::<i32>())
        .collect::<Vec<i32>>();
    cal.sort();
    cal.reverse();
    cal.get(0..3).unwrap().iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{most_calories_from_file, three_most_calories_from_file};

    #[test]
    fn part1() {
        assert_eq!(24000, most_calories_from_file("testinput/day1.txt"));
        assert_eq!(69912, most_calories_from_file("input/day1.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(45000, three_most_calories_from_file("testinput/day1.txt"));
        assert_eq!(208_180, three_most_calories_from_file("input/day1.txt"));
    }
}
