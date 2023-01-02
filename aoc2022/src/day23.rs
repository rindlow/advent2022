use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: i32,
    col: i32,
}

fn proposal(coord: &Coord, map: &HashSet<Coord>, dirindex: i32) -> Option<Coord> {
    if !(coord.row - 1..=coord.row + 1).any(|row| {
        (coord.col - 1..=coord.col + 1)
            .any(|col| (row != coord.row || col != coord.col) && map.contains(&Coord { row, col }))
    }) {
        return None;
    }
    for i in dirindex..dirindex + 4 {
        if i % 4 == 0
            && !(coord.col - 1..=coord.col + 1).any(|col| {
                map.contains(&Coord {
                    row: coord.row - 1,
                    col,
                })
            })
        {
            return Some(Coord {
                row: coord.row - 1,
                col: coord.col,
            });
        }
        if i % 4 == 1
            && !(coord.col - 1..=coord.col + 1).any(|col| {
                map.contains(&Coord {
                    row: coord.row + 1,
                    col,
                })
            })
        {
            return Some(Coord {
                row: coord.row + 1,
                col: coord.col,
            });
        }
        if i % 4 == 2
            && !(coord.row - 1..=coord.row + 1).any(|row| {
                map.contains(&Coord {
                    row,
                    col: coord.col - 1,
                })
            })
        {
            return Some(Coord {
                row: coord.row,
                col: coord.col - 1,
            });
        }
        if i % 4 == 3
            && !(coord.row - 1..=coord.row + 1).any(|row| {
                map.contains(&Coord {
                    row,
                    col: coord.col + 1,
                })
            })
        {
            return Some(Coord {
                row: coord.row,
                col: coord.col + 1,
            });
        }
    }
    None
}

#[allow(dead_code)]
fn print_map(map: &HashSet<Coord>) {
    let min_row = map.iter().map(|c| c.row).min().unwrap();
    let max_row = map.iter().map(|c| c.row).max().unwrap();
    let min_col = map.iter().map(|c| c.col).min().unwrap();
    let max_col = map.iter().map(|c| c.col).max().unwrap();

    (min_row..=max_row).for_each(|row| {
        (min_col..=max_col).for_each(|col| {
            if map.contains(&Coord { row, col }) {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });
    println!();
}

fn parse_file(filename: &str) -> HashSet<Coord> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| Coord {
                    row: i32::try_from(row).unwrap(),
                    col: i32::try_from(col).unwrap(),
                })
        })
        .collect()
}

fn move_elves(elves: &HashSet<Coord>, dirindex: i32) -> HashSet<Coord> {
    let mut proposals: HashMap<Coord, Coord> = HashMap::default();
    let mut destinations: HashMap<Coord, i32> = HashMap::default();
    for elf in elves {
        if let Some(prop) = proposal(elf, elves, dirindex) {
            proposals.insert(elf.clone(), prop.clone());
            destinations
                .entry(prop)
                .and_modify(|i| *i += 1)
                .or_insert(1);
        }
    }

    let mut new: HashSet<Coord> = HashSet::default();
    for elf in elves {
        if let Some(dst) = proposals.get(elf) {
            if destinations[dst] == 1 {
                new.insert(dst.clone());
            } else {
                new.insert(elf.clone());
            }
        } else {
            new.insert(elf.clone());
        }
    }
    new
}

pub fn empty_tiles(filename: &str) -> i32 {
    let mut elves = parse_file(filename);

    for i in 0..10 {
        elves = move_elves(&elves, i);
    }
    let min_row = elves.iter().map(|c| c.row).min().unwrap();
    let max_row = elves.iter().map(|c| c.row).max().unwrap();
    let min_col = elves.iter().map(|c| c.col).min().unwrap();
    let max_col = elves.iter().map(|c| c.col).max().unwrap();

    (max_row - min_row + 1) * (max_col - min_col + 1) - i32::try_from(elves.len()).unwrap()
}

pub fn no_move_round(filename: &str) -> i32 {
    let mut elves = parse_file(filename);
    let mut i = 0;
    loop {
        let new = move_elves(&elves, i);
        if new == elves {
            return i + 1;
        }
        elves = new;
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{empty_tiles, no_move_round};
    use test_case::test_case;

    #[test_case("../testinput/day23.txt", 110; "on test input")]
    #[test_case("../input/day23.txt", 3757; "on real input")]
    fn part1(filename: &str, expected: i32) {
        assert_eq!(empty_tiles(filename), expected);
    }
    #[test_case("../testinput/day23.txt", 20; "on test input")]
    #[test_case("../input/day23.txt", 918; "on real input")]
    fn part2(filename: &str, expected: i32) {
        assert_eq!(no_move_round(filename), expected);
    }
}
