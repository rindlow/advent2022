#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

use std::fs::read_to_string;

enum Operator {
    Addx,
    Noop,
}
struct Instruction {
    operator: Operator,
    operand: i32,
}

fn parse_file(filename: &str) -> Vec<Instruction> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            if line.starts_with("addx") {
                Instruction {
                    operator: Operator::Addx,
                    operand: line.get(5..).unwrap().parse::<i32>().unwrap(),
                }
            } else {
                Instruction {
                    operator: Operator::Noop,
                    operand: 0,
                }
            }
        })
        .collect()
}

fn exec(instructions: Vec<Instruction>) -> Vec<i32> {
    let mut timeline = Vec::<i32>::new();
    let mut x = 1;
    for instruction in instructions {
        match instruction.operator {
            Operator::Addx => {
                timeline.push(x);
                timeline.push(x);
                x += instruction.operand;
            }
            Operator::Noop => timeline.push(x),
        }
    }
    timeline
}

pub fn signal_strength(filename: &str) -> i32 {
    let timeline = exec(parse_file(filename));
    (0..6)
        .map(|base: usize| {
            let cycle = 20 + base * 40;
            timeline[cycle - 1] * cycle as i32
        })
        .sum()
}

pub fn crt(filename: &str) -> String {
    let timeline = exec(parse_file(filename));
    let mut screen = String::new();
    screen.push('\n');
    for (i, x) in timeline.iter().enumerate() {
        if (i as i32 % 40 - *x).abs() <= 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }
        if i % 40 == 39 {
            screen.push('\n');
        }
    }
    screen
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
            "\n\
             ##..##..##..##..##..##..##..##..##..##..\n\
             ###...###...###...###...###...###...###.\n\
             ####....####....####....####....####....\n\
             #####.....#####.....#####.....#####.....\n\
             ######......######......######......####\n\
             #######.......#######.......#######.....\n",
            crt("../testinput/day10.txt")
        );
        assert_eq!(
            "\n\
             ###..###....##.#....####.#..#.#....###..\n\
             #..#.#..#....#.#....#....#..#.#....#..#.\n\
             ###..#..#....#.#....###..#..#.#....#..#.\n\
             #..#.###.....#.#....#....#..#.#....###..\n\
             #..#.#.#..#..#.#....#....#..#.#....#....\n\
             ###..#..#..##..####.#.....##..####.#....\n",
            crt("../input/day10.txt")
        );
    }
}
