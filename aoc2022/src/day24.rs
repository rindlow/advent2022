use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct QueueItem {
    pos: Coord,
    step: usize,
}

type Blizzards = HashMap<Coord, Vec<Direction>>;

fn parse_file(filename: &str) -> Blizzards {
    let mut blizzards: Blizzards = HashMap::default();
    read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| match c {
                '^' => {
                    blizzards.insert(Coord { row, col }, vec![Direction::Up]);
                }
                'v' => {
                    blizzards.insert(Coord { row, col }, vec![Direction::Down]);
                }
                '<' => {
                    blizzards.insert(Coord { row, col }, vec![Direction::Left]);
                }
                '>' => {
                    blizzards.insert(Coord { row, col }, vec![Direction::Right]);
                }
                _ => {}
            });
        });
    blizzards
}

fn move_blizzards(blizzards: &Blizzards, max: &Coord) -> Blizzards {
    let mut new: Blizzards = HashMap::default();
    for (coord, directions) in blizzards {
        for dir in directions {
            match dir {
                Direction::Up => {
                    if coord.row > 1 {
                        new.entry(Coord {
                            row: coord.row - 1,
                            col: coord.col,
                        })
                        .and_modify(|dirs| dirs.push(dir.clone()))
                        .or_insert_with(|| vec![dir.clone()]);
                    } else {
                        new.entry(Coord {
                            row: max.row,
                            col: coord.col,
                        })
                        .and_modify(|dirs| dirs.push(dir.clone()))
                        .or_insert_with(|| vec![dir.clone()]);
                    }
                }
                Direction::Down => {
                    if coord.row < max.row {
                        new.entry(Coord {
                            row: coord.row + 1,
                            col: coord.col,
                        })
                        .and_modify(|dirs| dirs.push(dir.clone()))
                        .or_insert_with(|| vec![dir.clone()]);
                    } else {
                        new.entry(Coord {
                            row: 1,
                            col: coord.col,
                        })
                        .and_modify(|dirs| dirs.push(dir.clone()))
                        .or_insert_with(|| vec![dir.clone()]);
                    }
                }
                Direction::Left => {
                    if coord.col > 1 {
                        new.entry(Coord {
                            row: coord.row,
                            col: coord.col - 1,
                        })
                        .and_modify(|dirs| dirs.push(dir.clone()))
                        .or_insert_with(|| vec![dir.clone()]);
                    } else {
                        new.entry(Coord {
                            row: coord.row,
                            col: max.col,
                        })
                        .and_modify(|dirs| dirs.push(dir.clone()))
                        .or_insert_with(|| vec![dir.clone()]);
                    }
                }
                Direction::Right => {
                    if coord.col < max.col {
                        new.entry(Coord {
                            row: coord.row,
                            col: coord.col + 1,
                        })
                        .and_modify(|dirs| dirs.push(dir.clone()))
                        .or_insert_with(|| vec![dir.clone()]);
                    } else {
                        new.entry(Coord {
                            row: coord.row,
                            col: 1,
                        })
                        .and_modify(|dirs| dirs.push(dir.clone()))
                        .or_insert_with(|| vec![dir.clone()]);
                    }
                }
            }
        }
    }
    new
}

fn enqueue(queue: &mut VecDeque<QueueItem>, visited: &mut HashSet<QueueItem>, item: QueueItem) {
    if !visited.contains(&item) {
        visited.insert(item.clone());
        queue.push_back(item);
    }
}

#[allow(clippy::too_many_lines)]
pub fn minutes(filename: &str, legs: usize) -> usize {
    let mut timeline: Vec<Blizzards> = vec![parse_file(filename)];
    let max = Coord {
        row: timeline[0].keys().map(|c| c.row).max().unwrap(),
        col: timeline[0].keys().map(|c| c.col).max().unwrap(),
    };
    let mut visited: HashSet<QueueItem> = HashSet::default();
    let mut queue = VecDeque::<QueueItem>::new();
    let mut leg = 1;
    enqueue(
        &mut queue,
        &mut visited,
        QueueItem {
            pos: Coord { row: 0, col: 1 },
            step: 0,
        },
    );

    while !queue.is_empty() {
        let item = queue.pop_front().unwrap();

        if timeline.len() <= item.step {
            timeline.push(move_blizzards(&timeline[item.step - 1], &max));
        }

        if timeline[item.step].contains_key(&item.pos) {
            continue;
        }

        if (leg % 2 == 1 && item.pos == max)
            || (leg % 2 == 0 && item.pos == Coord { row: 1, col: 1 })
        {
            if leg == legs {
                return item.step + 1;
            }
            leg += 1;
            queue.clear();
            visited.clear();

            if leg % 2 == 1 {
                enqueue(
                    &mut queue,
                    &mut visited,
                    QueueItem {
                        pos: Coord { row: 0, col: 1 },
                        step: item.step + 1,
                    },
                );
            } else {
                enqueue(
                    &mut queue,
                    &mut visited,
                    QueueItem {
                        pos: Coord {
                            row: max.row + 1,
                            col: max.col,
                        },
                        step: item.step + 1,
                    },
                );
            }
            continue;
        }

        enqueue(
            &mut queue,
            &mut visited,
            QueueItem {
                pos: item.pos.clone(),
                step: item.step + 1,
            },
        );

        if item.pos.row == 0 {
            enqueue(
                &mut queue,
                &mut visited,
                QueueItem {
                    pos: Coord {
                        row: item.pos.row + 1,
                        col: item.pos.col,
                    },
                    step: item.step + 1,
                },
            );
            continue;
        }
        if item.pos.row > max.row {
            enqueue(
                &mut queue,
                &mut visited,
                QueueItem {
                    pos: Coord {
                        row: item.pos.row - 1,
                        col: item.pos.col,
                    },
                    step: item.step + 1,
                },
            );
            continue;
        }

        if item.pos.row < max.row {
            enqueue(
                &mut queue,
                &mut visited,
                QueueItem {
                    pos: Coord {
                        row: item.pos.row + 1,
                        col: item.pos.col,
                    },
                    step: item.step + 1,
                },
            );
        }
        if item.pos.row > 1 {
            enqueue(
                &mut queue,
                &mut visited,
                QueueItem {
                    pos: Coord {
                        row: item.pos.row - 1,
                        col: item.pos.col,
                    },
                    step: item.step + 1,
                },
            );
        }
        if item.pos.col > 1 {
            enqueue(
                &mut queue,
                &mut visited,
                QueueItem {
                    pos: Coord {
                        row: item.pos.row,
                        col: item.pos.col - 1,
                    },
                    step: item.step + 1,
                },
            );
        }
        if item.pos.col < max.col {
            enqueue(
                &mut queue,
                &mut visited,
                QueueItem {
                    pos: Coord {
                        row: item.pos.row,
                        col: item.pos.col + 1,
                    },
                    step: item.step + 1,
                },
            );
        }
    }

    panic!("no valid moves");
}

#[cfg(test)]
mod tests {
    use super::minutes;
    use test_case::test_case;

    #[test_case("../testinput/day24.txt", 18; "on test input")]
    #[test_case("../input/day24.txt", 281; "on real input")]
    fn part1(filename: &str, expected: usize) {
        assert_eq!(minutes(filename, 1), expected);
    }
    #[test_case("../testinput/day24.txt", 54; "on test input")]
    #[test_case("../input/day24.txt", 807; "on real input")]
    fn part2(filename: &str, expected: usize) {
        assert_eq!(minutes(filename, 3), expected);
    }
}
