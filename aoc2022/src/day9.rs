use std::{collections::HashSet, fs::read_to_string};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i32,
    y: i32,
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

// Imperative version is ~50 times faster than functional :-(
pub fn visited_nodes(filename: &str, n: usize) -> u64 {
    let mut knots = vec![Pos { x: 0, y: 0 }; n];
    let mut visited = HashSet::<Pos>::new();
    for elem in parse_file(filename) {
        for _ in 0..elem.steps {
            knots[0] = match elem.dir {
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
            visited.insert(knots[knots.len() - 1]);
        }
    }
    visited.len() as u64
}

#[cfg(test)]
mod tests {
    use super::visited_nodes;
    use test_case::test_case;

    #[test_case("../testinput/day9.txt", 13; "on test input")]
    #[test_case("../input/day9.txt", 6498; "on real input")]
    fn part1(filename: &str, expected: u64) {
        assert_eq!(visited_nodes(filename, 2), expected);
    }
    #[test_case("../testinput/day9.txt", 1; "on test input")]
    #[test_case("../testinput/day9b.txt", 36; "on test input B")]
    #[test_case("../input/day9.txt", 2531; "on real input")]
    fn part2(filename: &str, expected: u64) {
        assert_eq!(visited_nodes(filename, 10), expected);
    }
}
