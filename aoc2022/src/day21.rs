use fxhash::FxHashMap as HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
enum Job {
    Number(i64),
    Operation(Operation),
}

#[derive(Debug, Clone)]
struct Node {
    node: XNode,
    is_flagged: bool,
}

#[derive(Debug, Clone)]
struct InnerNode {
    operator: Operator,
    left: Box<Node>,
    right: Box<Node>,
}

#[derive(Debug, Clone)]
enum XNode {
    Inner(Box<InnerNode>),
    Outer(i64),
}

fn parse_file(filename: &str) -> Node {
    let mut map = HashMap::default();
    for line in read_to_string(filename).unwrap().lines() {
        let mut split = line.split(": ");
        let name = split.next().unwrap();
        let jobstr = split.next().unwrap();
        if jobstr.chars().next().unwrap().is_numeric() {
            map.insert(name.to_string(), Job::Number(jobstr.parse().unwrap()));
        } else {
            let mut osplit = jobstr.split(' ');
            let a = osplit.next().unwrap();
            let op = match osplit.next().unwrap() {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                "/" => Operator::Div,
                _ => panic!("parse error '{}'", line),
            };
            let b = osplit.next().unwrap();

            map.insert(
                name.to_string(),
                Job::Operation(Operation {
                    operator: op,
                    left: a.to_string(),
                    right: b.to_string(),
                }),
            );
        }
    }
    tree("root", &map)
}

fn tree(monkey: &str, monkeys: &HashMap<String, Job>) -> Node {
    let o = monkeys.get(monkey).unwrap();
    if monkey == "humn" {
        Node {
            node: XNode::Outer(match o {
                Job::Number(x) => *x,
                Job::Operation(_) => panic!("humn not number"),
            }),
            is_flagged: true,
        }
    } else {
        match o {
            Job::Number(n) => Node {
                node: XNode::Outer(*n),
                is_flagged: false,
            },
            Job::Operation(op) => {
                let left = tree(&op.left, monkeys);
                let right = tree(&op.right, monkeys);
                let is_flagged = left.is_flagged | right.is_flagged;
                Node {
                    node: XNode::Inner(Box::new(InnerNode {
                        operator: op.operator.clone(),
                        left: Box::new(left),
                        right: Box::new(right),
                    })),
                    is_flagged,
                }
            }
        }
    }
}

fn sum_tree(root: &Node) -> i64 {
    match &root.node {
        XNode::Outer(n) => *n,
        XNode::Inner(boxed) => {
            let inner = &**boxed;
            let left = sum_tree(&inner.left);
            let right = sum_tree(&inner.right);
            match inner.operator {
                Operator::Add => left + right,
                Operator::Sub => left - right,
                Operator::Mul => left * right,
                Operator::Div => left / right,
            }
        }
    }
}

fn walk_tree(root: &Node, balance: i64) -> i64 {
    match &root.node {
        XNode::Outer(_) => {
            if root.is_flagged {
                return balance;
            }
            panic!("malformed tree");
        }
        XNode::Inner(boxed) => {
            let inner = &**boxed;
            let left = &*inner.left;
            let right = &*inner.right;
            if left.is_flagged {
                let right_sum = sum_tree(right);
                walk_tree(
                    left,
                    match inner.operator {
                        Operator::Add => balance - right_sum,
                        Operator::Sub => balance + right_sum,
                        Operator::Mul => balance / right_sum,
                        Operator::Div => balance * right_sum,
                    },
                )
            } else {
                let left_sum = sum_tree(left);
                walk_tree(
                    right,
                    match inner.operator {
                        Operator::Add => balance - left_sum,
                        Operator::Sub => left_sum - balance,
                        Operator::Mul => balance / left_sum,
                        Operator::Div => left_sum / balance,
                    },
                )
            }
        }
    }
}

pub fn monkey_yell(filename: &str) -> i64 {
    let root = parse_file(filename);
    sum_tree(&root)
}

pub fn equality_test(filename: &str) -> i64 {
    let root = parse_file(filename);
    if let XNode::Inner(boxed) = root.node {
        let inner = *boxed;
        let left = *inner.left;
        let right = *inner.right;
        if left.is_flagged {
            walk_tree(&left, sum_tree(&right))
        } else {
            walk_tree(&right, sum_tree(&left))
        }
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{equality_test, monkey_yell};
    use test_case::test_case;

    #[test_case("../testinput/day21.txt", 152; "on test input")]
    #[test_case("../input/day21.txt", 276_156_919_469_632; "on real input")]
    fn part1(filename: &str, expected: i64) {
        assert_eq!(monkey_yell(filename), expected);
    }
    #[test_case("../testinput/day21.txt", 301; "on test input")]
    #[test_case("../input/day21.txt", 3_441_198_826_073; "on real input")]
    fn part2(filename: &str, expected: i64) {
        assert_eq!(equality_test(filename), expected);
    }
}
