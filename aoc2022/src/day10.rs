use itertools::Itertools;
use std::{convert::TryFrom, fs::read_to_string};

fn parse_file(filename: &str) -> Vec<i32> {
    let mut timeline = Vec::<i32>::new();
    let mut x = 1;
    for line in read_to_string(filename).unwrap().lines() {
        if line.starts_with("addx") {
            timeline.push(x);
            timeline.push(x);
            x += line.get(5..).unwrap().parse::<i32>().unwrap();
        } else {
            timeline.push(x);
        }
    }
    timeline
}

pub fn signal_strength(filename: &str) -> i32 {
    let timeline = parse_file(filename);
    (0..6)
        .map(|base: usize| {
            let cycle = 20 + base * 40;
            timeline[cycle - 1] * i32::try_from(cycle).unwrap()
        })
        .sum()
}

pub fn crt(filename: &str) -> String {
    parse_file(filename)
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if (i32::try_from(i).unwrap() % 40 - *x).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect_vec()
        .chunks(40)
        .map(|chunk| chunk.iter().collect::<String>())
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{crt, signal_strength};

    #[test]
    fn part1() {
        assert_eq!(13140, signal_strength("../testinput/day10.txt"));
        assert_eq!(12980, signal_strength("../input/day10.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##..\n\
             ###...###...###...###...###...###...###.\n\
             ####....####....####....####....####....\n\
             #####.....#####.....#####.....#####.....\n\
             ######......######......######......####\n\
             #######.......#######.......#######.....",
            crt("../testinput/day10.txt")
        );
        assert_eq!(
            "###..###....##.#....####.#..#.#....###..\n\
             #..#.#..#....#.#....#....#..#.#....#..#.\n\
             ###..#..#....#.#....###..#..#.#....#..#.\n\
             #..#.###.....#.#....#....#..#.#....###..\n\
             #..#.#.#..#..#.#....#....#..#.#....#....\n\
             ###..#..#..##..####.#.....##..####.#....",
            crt("../input/day10.txt")
        );
    }
}
