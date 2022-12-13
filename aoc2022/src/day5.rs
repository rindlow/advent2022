use itertools::Itertools;
use std::fs::read_to_string;

fn crates<F>(filename: &str, mover: F) -> String
where
    F: Fn(Vec<Vec<char>>, i8, usize, usize) -> Vec<Vec<char>>,
{
    let mut stacks = Vec::<Vec<char>>::new();
    for line in read_to_string(filename).unwrap().lines() {
        if line.contains('[') {
            for e in line.chars().enumerate() {
                match e {
                    (i, c) if ('A'..='Z').contains(&c) => {
                        while stacks.len() <= i / 4 {
                            stacks.push(Vec::<char>::new());
                        }
                        stacks[i / 4].insert(0, c);
                    }
                    _ => (),
                }
            }
        } else if line.starts_with("move") {
            let tokens = line.split(' ').collect_vec();
            stacks = mover(
                stacks,
                tokens[1].parse::<i8>().unwrap(),
                tokens[3].parse::<usize>().unwrap(),
                tokens[5].parse::<usize>().unwrap(),
            );
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}

pub fn crates_single(filename: &str) -> String {
    crates(filename, |mut stacks, n, src, dst| -> Vec<Vec<char>> {
        for _ in 0..n {
            let c = stacks[src - 1].pop().unwrap();
            stacks[dst - 1].push(c);
        }
        stacks
    })
}

pub fn crates_multiple(filename: &str) -> String {
    crates(filename, |mut stacks, n, src, dst| -> Vec<Vec<char>> {
        let mut tmp = Vec::<char>::new();
        for _ in 0..n {
            tmp.insert(0, stacks[src - 1].pop().unwrap());
        }
        stacks[dst - 1].extend(tmp);
        stacks
    })
}

#[cfg(test)]
mod tests {
    use crate::day5::{crates_multiple, crates_single};
    use test_case::test_case;

    #[test_case("../testinput/day5.txt", "CMZ"; "on test input")]
    #[test_case("../input/day5.txt", "VRWBSFZWM"; "on real input")]
    fn part1(filename: &str, expected: &str) {
        assert_eq!(crates_single(filename), expected);
    }
    #[test_case("../testinput/day5.txt", "MCD"; "on test input")]
    #[test_case("../input/day5.txt", "RBTWJWMCF"; "on real input")]
    fn part2(filename: &str, expected: &str) {
        assert_eq!(crates_multiple(filename), expected);
    }
}
