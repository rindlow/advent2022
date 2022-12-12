use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug)]
enum Operation {
    Mul(u64),
    Add(u64),
    Square,
}
#[derive(Debug)]
struct Monkey {
    index: usize,
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
}

fn make_monkey(lines: &[&str]) -> Monkey {
    let mut monkey = Monkey {
        index: 0,
        items: vec![],
        operation: Operation::Add(0),
        divisor: 1,
        true_monkey: 0,
        false_monkey: 0,
    };
    for line in lines {
        if line.starts_with("Monkey") {
            monkey.index = line.get(7..8).unwrap().parse().unwrap();
        }
        if line.starts_with("  S") {
            for item in line
                .get(18..)
                .unwrap()
                .split(", ")
                .map(|s| s.parse::<u64>().unwrap())
            {
                monkey.items.push(item);
            }
        } else if line.starts_with("  O") {
            let opstr = line.get(25..).unwrap();
            if opstr == "old" {
                monkey.operation = Operation::Square;
            } else {
                let operand = opstr.parse::<u64>().unwrap();
                monkey.operation = match line.get(23..24).unwrap() {
                    "*" => Operation::Mul(operand),
                    "+" => Operation::Add(operand),
                    _ => panic!("illegal operand {}", line.get(23..24).unwrap()),
                };
            }
        } else if line.starts_with("  T") {
            monkey.divisor = line.get(21..).unwrap().parse().unwrap();
        } else if line.starts_with("    If true") {
            monkey.true_monkey = line.get(29..).unwrap().parse().unwrap();
        } else if line.starts_with("    If false") {
            monkey.false_monkey = line.get(30..).unwrap().parse().unwrap();
        }
    }
    monkey
}

fn parse_file(filename: &str) -> Vec<Monkey> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .collect_vec()
        .split(|s| s.is_empty())
        .map(make_monkey)
        .collect_vec()
}

pub fn monkey_business(filename: &str, is_part1: bool) -> u64 {
    let monkeys = parse_file(filename);
    let mut items = monkeys.iter().map(|m| m.items.clone()).collect_vec();
    let mut inspections = vec![0; monkeys.len()];
    let modulus: u64 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 1..=if is_part1 { 20 } else { 10000 } {
        for monkey in &monkeys {
            for index in 0..items[monkey.index].len() {
                let item = items[monkey.index][index];
                let mut level = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Mul(x) => item * x,
                    Operation::Square => item * item,
                };
                if is_part1 {
                    level /= 3;
                } else {
                    level %= modulus;
                };
                if level % monkey.divisor == 0 {
                    items[monkey.true_monkey].push(level);
                } else {
                    items[monkey.false_monkey].push(level);
                }
                inspections[monkey.index] += 1;
            }
            items[monkey.index].clear();
        }
    }
    inspections.iter().sorted().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use super::monkey_business;

    #[test]
    fn part1() {
        assert_eq!(10605, monkey_business("../testinput/day11.txt", true));
        assert_eq!(50616, monkey_business("../input/day11.txt", true));
    }

    #[test]
    fn part2() {
        assert_eq!(
            2_713_310_158,
            monkey_business("../testinput/day11.txt", false)
        );
        assert_eq!(11_309_046_332, monkey_business("../input/day11.txt", false));
    }
}
