use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn dirsizes(filename: &str) -> HashMap<String, i32> {
    let mut cwd = vec![""];
    let mut dirsizes = HashMap::<String, i32>::new();
    for line in read_to_string(filename).unwrap().lines() {
        if line.starts_with("$ cd ") {
            match line.get(5..) {
                Some("/") => cwd = vec![""],
                Some("..") => _ = cwd.pop(),
                Some(dir) => cwd.push(dir),
                None => panic!("impossible cd"),
            }
        } else if line.chars().next().unwrap().is_numeric() {
            let size = line.split(' ').next().unwrap().parse::<i32>().unwrap();
            for i in 0..cwd.len() {
                dirsizes
                    .entry(cwd.get(..=i).unwrap().join("/"))
                    .and_modify(|e| *e += size)
                    .or_insert(size);
            }
        }
    }
    dirsizes
}

pub fn sum_dirs_below(filename: &str) -> i32 {
    dirsizes(filename)
        .into_values()
        .filter(|i| *i <= 100_000)
        .sum()
}

pub fn dir_to_delete(filename: &str) -> i32 {
    let dirs = dirsizes(filename);
    let needed = 30_000_000 - 70_000_000 + dirs[""];
    dirs.into_values()
        .sorted()
        .find(|size| *size >= needed)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day7::{dir_to_delete, sum_dirs_below};
    use test_case::test_case;

    #[test_case("../testinput/day7.txt", 95_437; "on test input")]
    #[test_case("../input/day7.txt", 1_792_222; "on real input")]
    fn part1(filename: &str, expected: i32) {
        assert_eq!(sum_dirs_below(filename), expected);
    }
    #[test_case("../testinput/day7.txt", 24_933_642; "on test input")]
    #[test_case("../input/day7.txt", 1_112_963; "on real input")]
    fn part2(filename: &str, expected: i32) {
        assert_eq!(dir_to_delete(filename), expected);
    }
}
