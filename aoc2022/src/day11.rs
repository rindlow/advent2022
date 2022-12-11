use itertools::Itertools;
use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug)]
enum Operation {
    Mul(u64),
    Add(u64),
    Square,
}
#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
    inspections: u64,
}

fn make_monkey(lines: &[&str]) -> Monkey {
    let mut monkey = Monkey {
        items: VecDeque::<u64>::new(),
        operation: Operation::Add(0),
        divisor: 1,
        true_monkey: 0,
        false_monkey: 0,
        inspections: 0,
    };
    for line in lines {
        if line.starts_with("  S") {
            for item in line
                .get(18..)
                .unwrap()
                .split(", ")
                .map(|s| s.parse::<u64>().unwrap())
            {
                monkey.items.push_back(item);
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

pub fn monkey_business_20(filename: &str) -> u64 {
    let mut monkeys = parse_file(filename);
    for _ in 1..=20 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            let tm = monkeys[i].true_monkey;
            let fm = monkeys[i].false_monkey;
            for item in items {
                let level = match monkeys[i].operation {
                    Operation::Add(x) => item + x,
                    Operation::Mul(x) => item * x,
                    Operation::Square => item * item,
                } / 3;
                if level % monkeys[i].divisor == 0 {
                    monkeys[tm].items.push_back(level);
                } else {
                    monkeys[fm].items.push_back(level);
                }
                monkeys[i].inspections += 1;
            }
            monkeys[i].items.clear();
        }
    }
    monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn monkey_business_10k(filename: &str) -> u64 {
    let mut monkeys = parse_file(filename);
    let modulus: u64 = monkeys.iter().map(|m| m.divisor).product();
    for _ in 1..=10000 {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.clone();
            let tm = monkeys[i].true_monkey;
            let fm = monkeys[i].false_monkey;
            for item in items {
                let level = match monkeys[i].operation {
                    Operation::Add(x) => item + x,
                    Operation::Mul(x) => item * x,
                    Operation::Square => item * item,
                } % modulus;
                if level % monkeys[i].divisor == 0 {
                    monkeys[tm].items.push_back(level);
                } else {
                    monkeys[fm].items.push_back(level);
                }
                monkeys[i].inspections += 1;
            }
            monkeys[i].items.clear();
        }
    }
    monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::{monkey_business_10k, monkey_business_20};

    #[test]
    fn part1() {
        assert_eq!(10605, monkey_business_20("../testinput/day11.txt"));
        assert_eq!(50616, monkey_business_20("../input/day11.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!(
            2_713_310_158,
            monkey_business_10k("../testinput/day11.txt",)
        );
        assert_eq!(11_309_046_332, monkey_business_10k("../input/day11.txt"));
    }
}
