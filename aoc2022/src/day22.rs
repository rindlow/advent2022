use fxhash::FxHashMap as HashMap;
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone)]
struct State {
    coord: Coord,
    facing: Facing,
}

#[derive(Debug, Clone)]
enum Move {
    Number(i32),
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

type Map = HashMap<Coord, Tile>;

fn wrap0(state: &State, map: &Map) -> Option<State> {
    let (coord, tile) = match state.facing {
        Facing::Right => map
            .iter()
            .filter(|(coord, _)| coord.row == state.coord.row)
            .min_by(|(a, _), (b, _)| a.col.cmp(&b.col))
            .unwrap(),
        Facing::Down => map
            .iter()
            .filter(|(coord, _)| coord.col == state.coord.col)
            .min_by(|(a, _), (b, _)| a.row.cmp(&b.row))
            .unwrap(),
        Facing::Left => map
            .iter()
            .filter(|(coord, _)| coord.row == state.coord.row)
            .max_by(|(a, _), (b, _)| a.col.cmp(&b.col))
            .unwrap(),
        Facing::Up => map
            .iter()
            .filter(|(coord, _)| coord.col == state.coord.col)
            .max_by(|(a, _), (b, _)| a.row.cmp(&b.row))
            .unwrap(),
    };
    if *tile == Tile::Open {
        Some(State {
            coord: coord.clone(),
            facing: state.facing.clone(),
        })
    } else {
        None
    }
}

fn wrap1(state: &State, map: &Map) -> Option<State> {
    let mut coord = state.coord.clone();
    let mut facing = state.facing.clone();
    if state.coord.row == 0 && state.facing == Facing::Up {
        coord = Coord {
            row: 4,
            col: 11 - state.coord.col,
        };
        facing = Facing::Down;
    } else if state.coord.col == 8 && state.coord.row < 4 && state.facing == Facing::Left {
        coord = Coord {
            row: 4,
            col: state.coord.col + 4,
        };
        facing = Facing::Down;
    } else if state.coord.col == 11 && state.coord.row < 4 && state.facing == Facing::Right {
        coord = Coord {
            row: 11 - state.coord.row,
            col: 15,
        };
        facing = Facing::Left;
    } else if state.coord.col == 0 && state.facing == Facing::Left {
        coord = Coord {
            row: state.coord.row,
            col: 11,
        };
    } else if state.coord.row == 4 && state.coord.col < 4 && state.facing == Facing::Up {
        coord = Coord {
            row: 0,
            col: 11 - state.coord.col,
        };
        facing = Facing::Down;
    } else if state.coord.row == 4 && state.facing == Facing::Up {
        coord = Coord {
            row: state.coord.col - 4,
            col: 8,
        };
        facing = Facing::Right;
    } else if state.coord.col == 11 && state.facing == Facing::Right {
        coord = Coord {
            row: 8,
            col: 19 - state.coord.row,
        };
        facing = Facing::Down;
    } else if state.coord.row == 7 && state.coord.col < 4 && state.facing == Facing::Down {
        coord = Coord {
            row: 11,
            col: 11 - state.coord.col,
        };
        facing = Facing::Up;
    } else if state.coord.row == 7 && state.facing == Facing::Down {
        coord = Coord {
            row: 15 - state.coord.col,
            col: 8,
        };
        facing = Facing::Right;
    } else if state.coord.col == 8 && state.facing == Facing::Left {
        coord = Coord {
            row: 7,
            col: 15 - state.coord.row,
        };
        facing = Facing::Up;
    } else if state.coord.row == 8 && state.facing == Facing::Up {
        coord = Coord {
            row: 7,
            col: 19 - state.coord.row,
        };
        facing = Facing::Left;
    } else if state.coord.col == 15 && state.facing == Facing::Right {
        coord = Coord {
            row: 11 - state.coord.row,
            col: 11,
        };
        facing = Facing::Left;
    } else if state.coord.row == 11 && state.coord.col < 12 && state.facing == Facing::Down {
        coord = Coord {
            row: 7,
            col: 11 - state.coord.col,
        };
        facing = Facing::Up;
    } else if state.coord.row == 11 && state.facing == Facing::Down {
        coord = Coord {
            row: 19 - state.coord.row,
            col: 0,
        };
        facing = Facing::Right;
    }
    assert!(map.contains_key(&coord), "cube fold error");

    if map[&coord] == Tile::Open {
        Some(State { coord, facing })
    } else {
        None
    }
}

fn wrap2(state: &State, map: &Map) -> Option<State> {
    let mut coord = state.coord.clone();
    let mut facing = state.facing.clone();
    let c = state.coord.col;
    let r = state.coord.row;
    if r == 0 && c < 100 && state.facing == Facing::Up {
        coord = Coord {
            row: 100 + c,
            col: 0,
        };
        facing = Facing::Right;
    } else if r == 0 && state.facing == Facing::Up {
        coord = Coord {
            row: 199,
            col: c - 100,
        };
        facing = Facing::Up;
    } else if c == 50 && r < 50 && state.facing == Facing::Left {
        coord = Coord {
            row: 149 - r,
            col: 0,
        };
        facing = Facing::Right;
    } else if c == 149 && state.facing == Facing::Right {
        coord = Coord {
            row: 149 - r,
            col: 99,
        };
        facing = Facing::Left;
    } else if r == 49 && state.facing == Facing::Down {
        coord = Coord {
            row: c - 50,
            col: 99,
        };
        facing = Facing::Left;
    } else if c == 50 && state.facing == Facing::Left {
        coord = Coord {
            row: 100,
            col: r - 50,
        };
        facing = Facing::Down;
    } else if c == 99 && r < 100 && state.facing == Facing::Right {
        coord = Coord {
            row: 49,
            col: r + 50,
        };
        facing = Facing::Up;
    } else if c == 0 && r < 150 && state.facing == Facing::Left {
        coord = Coord {
            row: 149 - r,
            col: 50,
        };
        facing = Facing::Right;
    } else if r == 100 && state.facing == Facing::Up {
        coord = Coord {
            row: c + 50,
            col: 50,
        };
        facing = Facing::Right;
    } else if c == 99 && state.facing == Facing::Right {
        coord = Coord {
            row: 149 - r,
            col: 149,
        };
        facing = Facing::Left;
    } else if r == 149 && state.facing == Facing::Down {
        coord = Coord {
            row: c + 100,
            col: 49,
        };
        facing = Facing::Left;
    } else if c == 0 && state.facing == Facing::Left {
        coord = Coord {
            row: 0,
            col: r - 100,
        };
        facing = Facing::Down;
    } else if c == 49 && state.facing == Facing::Right {
        coord = Coord {
            row: 149,
            col: r - 100,
        };
        facing = Facing::Up;
    } else if r == 199 && state.facing == Facing::Down {
        coord = Coord {
            row: 0,
            col: c + 100,
        };
    }
    assert!(map.contains_key(&coord), "cube fold error");

    if map[&coord] == Tile::Open {
        Some(State { coord, facing })
    } else {
        None
    }
}

fn try_move(state: &State, map: &Map, mode: i32) -> Option<State> {
    let new = match state.facing {
        Facing::Right => Coord {
            row: state.coord.row,
            col: state.coord.col + 1,
        },
        Facing::Down => Coord {
            row: state.coord.row + 1,
            col: state.coord.col,
        },
        Facing::Left => Coord {
            row: state.coord.row,
            col: state.coord.col - 1,
        },
        Facing::Up => Coord {
            row: state.coord.row - 1,
            col: state.coord.col,
        },
    };
    if map.contains_key(&new) {
        match map[&new] {
            Tile::Open => Some(State {
                coord: new.clone(),
                facing: state.facing.clone(),
            }),
            Tile::Wall => None,
        }
    } else if mode == 0 {
        wrap0(state, map)
    } else if mode == 1 {
        wrap1(state, map)
    } else {
        wrap2(state, map)
    }
}

pub fn final_password(filename: &str, mode: i32) -> i32 {
    let binding = read_to_string(filename).unwrap();
    let lines = binding.lines().collect_vec();
    let mut split = lines.split(|line| line.is_empty());
    let mut map: Map = HashMap::default();
    for (row_usize, line) in split.next().unwrap().iter().enumerate() {
        for (col_usize, c) in line.chars().enumerate() {
            let row = i32::try_from(row_usize).unwrap();
            let col = i32::try_from(col_usize).unwrap();
            match c {
                '.' => map.insert(Coord { row, col }, Tile::Open),
                '#' => map.insert(Coord { row, col }, Tile::Wall),
                _ => None,
            };
        }
    }
    let mut path = Vec::<Move>::new();
    let mut i: i32 = 0;
    for c in split.next().unwrap().iter().next().unwrap().chars() {
        match c {
            c if c.is_numeric() => {
                i *= 10;
                i += i32::try_from(c.to_digit(10).unwrap()).unwrap();
            }
            'L' => {
                path.push(Move::Number(i));
                i = 0;
                path.push(Move::Left);
            }
            'R' => {
                path.push(Move::Number(i));
                i = 0;
                path.push(Move::Right);
            }
            _ => panic!("parse error"),
        };
    }
    path.push(Move::Number(i));

    let mut state = State {
        coord: map
            .keys()
            .sorted_by(|a, b| {
                if a.row == b.row {
                    a.col.cmp(&b.col)
                } else {
                    a.row.cmp(&b.row)
                }
            })
            .next()
            .unwrap()
            .clone(),
        facing: Facing::Right,
    };

    for step in path {
        match step {
            Move::Right => {
                state.facing = match state.facing {
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                    Facing::Up => Facing::Right,
                }
            }
            Move::Left => {
                state.facing = match state.facing {
                    Facing::Right => Facing::Up,
                    Facing::Up => Facing::Left,
                    Facing::Left => Facing::Down,
                    Facing::Down => Facing::Right,
                }
            }
            Move::Number(n) => {
                for _ in 0..n {
                    if let Some(new) = try_move(&state, &map, mode) {
                        state = new.clone();
                    } else {
                        break;
                    }
                }
            }
        }
    }
    1000 * (state.coord.row + 1)
        + 4 * (state.coord.col + 1)
        + match state.facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
}

#[cfg(test)]
mod tests {
    use super::final_password;
    use test_case::test_case;

    #[test_case("../testinput/day22.txt", 6032; "on test input")]
    #[test_case("../input/day22.txt", 89224; "on real input")]
    fn part1(filename: &str, expected: i32) {
        assert_eq!(final_password(filename, 0), expected);
    }

    #[test_case("../testinput/day22.txt", 1, 5031; "on test input")]
    #[test_case("../input/day22.txt", 2, 136_182; "on real input")]
    fn part2(filename: &str, mode: i32, expected: i32) {
        assert_eq!(final_password(filename, mode), expected);
    }
}
