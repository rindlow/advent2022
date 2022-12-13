use std::{cmp::Ordering, fs::read_to_string, iter::zip};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Int(u32),
    List(Vec<Item>),
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Int(own_int), Item::Int(other_int)) => own_int.cmp(other_int),
            (Item::List(own_list), Item::List(other_list)) => {
                if let Some(order) = zip(own_list, other_list)
                    .map(|(a, b)| a.cmp(b))
                    .find(|o| *o != Ordering::Equal)
                {
                    order
                } else {
                    own_list.len().cmp(&other_list.len())
                }
            }
            (Item::Int(own_int), Item::List(other_list)) => {
                let list = Item::List(vec![Item::Int(*own_int)]);
                list.cmp(&Item::List(other_list.clone()))
            }
            (Item::List(own_list), Item::Int(other_int)) => {
                let list = Item::List(vec![Item::Int(*other_int)]);
                Item::List(own_list.clone()).cmp(&list)
            }
        }
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Pair {
    left: Item,
    right: Item,
}

#[derive(Debug)]
enum StackItem {
    ListStart,
    ListItem(Item),
}

fn parse_item(line: &str) -> Item {
    let mut stack = Vec::<StackItem>::new();
    let mut current_int: Option<u32> = None;

    for c in line.chars() {
        if c == '[' {
            stack.push(StackItem::ListStart);
        } else if c.is_numeric() {
            let d = c.to_digit(10).unwrap();
            current_int = match current_int {
                Some(i) => Some(i * 10 + d),
                None => Some(d),
            }
        } else if c == ',' {
            if let Some(i) = current_int {
                stack.push(StackItem::ListItem(Item::Int(i)));
                current_int = None;
            }
        } else if c == ']' {
            if let Some(i) = current_int {
                stack.push(StackItem::ListItem(Item::Int(i)));
                current_int = None;
            }

            let last_liststart = stack
                .iter()
                .rposition(|si| matches!(si, StackItem::ListStart))
                .unwrap();
            let list = stack
                .drain(last_liststart..)
                .skip(1) // skip ListStart
                .map(|si| {
                    if let StackItem::ListItem(item) = si {
                        item
                    } else {
                        panic!("ListStart found in wrong place");
                    }
                })
                .collect_vec();
            stack.push(StackItem::ListItem(Item::List(list)));
        }
    }
    match stack.first().unwrap() {
        StackItem::ListItem(item) => item.clone(),
        StackItem::ListStart => panic!("Parse error"),
    }
}

pub fn sum_indices(filename: &str) -> usize {
    read_to_string(filename)
        .unwrap()
        .lines()
        .collect_vec()
        .split(|s| s.is_empty())
        .map(|split| Pair {
            left: parse_item(split[0]),
            right: parse_item(split[1]),
        })
        .enumerate()
        .filter(|(_, p)| p.left.cmp(&p.right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn decoder_key(filename: &str) -> usize {
    let dividers = vec![parse_item("[[2]]"), parse_item("[[6]]")];
    let items = read_to_string(filename)
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(parse_item)
        .chain(dividers.iter().map(std::borrow::ToOwned::to_owned))
        .sorted()
        .collect_vec();

    dividers
        .iter()
        .map(|div| items.iter().position(|i| *i == *div).unwrap() + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::{decoder_key, sum_indices};

    #[test]
    fn part1test() {
        assert_eq!(13, sum_indices("../testinput/day13.txt"));
    }
    #[test]
    fn part1() {
        assert_eq!(5252, sum_indices("../input/day13.txt"));
    }
    #[test]
    fn part2test() {
        assert_eq!(140, decoder_key("../testinput/day13.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(20592, decoder_key("../input/day13.txt"));
    }
}
