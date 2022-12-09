use itertools::Itertools;
use std::{
    char::from_digit,
    cmp::{max, min},
    fmt,
    fs::read_to_string,
};

#[derive(Copy, Clone, Debug)]
enum Direction {
    L,
    U,
    R,
    D,
}
struct Motion {
    dir: Direction,
    steps: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct State {
    knots: Vec<Pos>,
    visited: Vec<Pos>,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_x = min(
            0,
            self.knots
                .iter()
                .chain(self.visited.iter())
                .map(|p| p.x)
                .min()
                .unwrap(),
        );
        let min_y = min(
            0,
            self.knots
                .iter()
                .chain(self.visited.iter())
                .map(|p| p.y)
                .min()
                .unwrap(),
        );
        let max_x = max(
            6,
            self.knots
                .iter()
                .chain(self.visited.iter())
                .map(|p| p.x)
                .max()
                .unwrap(),
        );
        let max_y = max(
            5,
            self.knots
                .iter()
                .chain(self.visited.iter())
                .map(|p| p.y)
                .max()
                .unwrap(),
        );
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                write!(f, "{}", display_pos(self, x, y)).unwrap();
            }
            writeln!(f).unwrap();
        }
        writeln!(f)
    }
}

fn display_pos(state: &State, x: i32, y: i32) -> char {
    match state
        .knots
        .iter()
        .enumerate()
        .find(|(_, p)| p.x == x && p.y == y)
    {
        Some((i, _)) => from_digit(u32::try_from(i).unwrap(), 10).unwrap(),
        None => {
            if state.visited.iter().any(|p| p.x == x && p.y == y) {
                '#'
            } else if x == 0 && y == 0 {
                's'
            } else {
                '.'
            }
        }
    }
}

fn parse_file(filename: &str) -> Vec<Motion> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let dir = split.next();
            let steps = split.next().unwrap().parse::<u32>().unwrap();
            Motion {
                dir: match dir {
                    Some("L") => Direction::L,
                    Some("U") => Direction::U,
                    Some("R") => Direction::R,
                    Some("D") => Direction::D,
                    Some(_) => panic!("unknown direction"),
                    None => panic!("parse error"),
                },
                steps,
            }
        })
        .collect()
}

fn knot_pos(knots: &[Pos], idx: usize) -> Pos {
    let knot = knots[idx];
    let dx = knots[idx - 1].x - knot.x;
    let dy = knots[idx - 1].y - knot.y;
    if dx > 1 {
        Pos {
            x: knot.x + 1,
            y: knot.y + dy.signum(),
        }
    } else if dx < -1 {
        Pos {
            x: knot.x - 1,
            y: knot.y + dy.signum(),
        }
    } else if dy > 1 {
        Pos {
            x: knot.x + dx.signum(),
            y: knot.y + 1,
        }
    } else if dy < -1 {
        Pos {
            x: knot.x + dx.signum(),
            y: knot.y - 1,
        }
    } else {
        knot
    }
}

fn step(state: &State, dir: Direction) -> State {
    let mut knots = state.knots.clone();
    knots[0] = match dir {
        Direction::D => Pos {
            x: knots[0].x,
            y: knots[0].y - 1,
        },
        Direction::L => Pos {
            x: knots[0].x - 1,
            y: knots[0].y,
        },
        Direction::R => Pos {
            x: knots[0].x + 1,
            y: knots[0].y,
        },
        Direction::U => Pos {
            x: knots[0].x,
            y: knots[0].y + 1,
        },
    };
    for i in 1..knots.len() {
        knots[i] = knot_pos(&knots, i);
    }
    let mut visited = state.visited.clone();
    visited.push(knots[knots.len() - 1]);

    State { knots, visited }
}

fn motion(acc: State, elem: &Motion) -> State {
    (0..elem.steps).fold(acc, |s, _| step(&s, elem.dir))
}

pub fn visited_nodes(filename: &str, n: usize) -> u64 {
    let state = parse_file(filename).iter().fold(
        State {
            knots: vec![Pos { x: 0, y: 0 }; n],
            visited: Vec::<Pos>::new(),
        },
        motion,
    );
    state.visited.iter().sorted().dedup().count() as u64
}

#[cfg(test)]
mod tests {
    use super::visited_nodes;

    #[test]
    fn part1() {
        assert_eq!(13, visited_nodes("testinput/day9.txt", 2));
        assert_eq!(6498, visited_nodes("input/day9.txt", 2));
    }
    #[test]
    fn part2() {
        assert_eq!(1, visited_nodes("testinput/day9.txt", 10));
        assert_eq!(36, visited_nodes("testinput/day9b.txt", 10));
        assert_eq!(2531, visited_nodes("input/day9.txt", 10));
    }
}
