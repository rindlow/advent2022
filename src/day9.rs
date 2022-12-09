use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
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

fn new_pos(knots: &[Pos], idx: usize) -> Pos {
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
        knots[i] = new_pos(&knots, i);
    }
    let mut visited = state.visited.clone();
    visited.push(knots[knots.len() - 1]);

    State { knots, visited }
}

pub fn visited_nodes(filename: &str, n: usize) -> u64 {
    let mut steps = 0;
    let state = parse_file(filename).iter().fold(
        State {
            knots: vec![Pos { x: 0, y: 0 }; n],
            visited: Vec::<Pos>::new(),
        },
        |acc, elem| {
            (0..elem.steps).fold(acc, |s, _| {
                steps += 1;
                step(&s, elem.dir)
            })
        },
    );
    println!("Total #steps = {}", steps);
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
