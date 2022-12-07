use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn dirsizes(filename: &str) -> HashMap<String, u64> {
    let mut cwd = vec![""];
    let mut dirsizes = HashMap::<String, u64>::new();
    for line in read_to_string(filename).unwrap().lines() {
        if line.starts_with("$ cd ") {
            match line.get(5..) {
                Some("/") => cwd = vec![""],
                Some("..") => _ = cwd.pop(),
                Some(dir) => cwd.push(dir),
                None => panic!("impossible cd"),
            }
        } else if line.chars().next().unwrap().is_numeric() {
            let size = line.split(' ').next().unwrap().parse::<u64>().unwrap();
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

pub fn sum_dirs_below(filename: &str, threshold: u64) -> u64 {
    dirsizes(filename)
        .into_values()
        .filter(|i| *i <= threshold)
        .sum()
}

pub fn dir_to_delete(filename: &str, total_size: u64, required: u64) -> u64 {
    let dirs = dirsizes(filename);
    let unused = total_size - dirs[""];
    dirs.into_values()
        .sorted()
        .find(|size| *size >= required - unused)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day7::{dir_to_delete, sum_dirs_below};
    #[test]
    fn part1() {
        assert_eq!(95_437, sum_dirs_below("testinput/day7.txt", 100_000));
        assert_eq!(1_792_222, sum_dirs_below("input/day7.txt", 100_000));
    }
    #[test]
    fn part2() {
        assert_eq!(
            24_933_642,
            dir_to_delete("testinput/day7.txt", 70_000_000, 30_000_000)
        );
        assert_eq!(
            1_112_963,
            dir_to_delete("input/day7.txt", 70_000_000, 30_000_000)
        );
    }
}
