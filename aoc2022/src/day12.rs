use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Input {
    start: Pos,
    end: Pos,
    map: Vec<Vec<i32>>,
}

fn parse_file(filename: &str) -> Input {
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    let map = read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Pos { x, y };
                        'a' as i32
                    } else if c == 'E' {
                        end = Pos { x, y };
                        'z' as i32
                    } else {
                        c as i32
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    Input { start, end, map }
}

fn h(a: Pos, b: Option<Pos>) -> i32 {
    match b {
        Some(pos) => {
            (i32::try_from(a.x).unwrap() - i32::try_from(pos.x).unwrap()).abs()
                + (i32::try_from(a.y).unwrap() - i32::try_from(pos.y).unwrap()).abs()
        }
        None => i32::try_from(a.x).unwrap(),
    }
}

fn is_goal(c: Pos, end: Option<Pos>) -> bool {
    match end {
        Some(pos) => c == pos,
        None => c.x == 0,
    }
}

fn reconstruct_path(came_from: &HashMap<Pos, Pos>, start: &Pos) -> Vec<Pos> {
    let mut current = *start;
    let mut path = vec![current];
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap();
        // println!("{:?}", current);
        path.insert(0, current);
    }
    path
}

fn neighbours(p: Pos, map: &Vec<Vec<i32>>) -> Vec<Pos> {
    let mut n = Vec::<Pos>::new();
    if p.x > 0 && map[p.y][p.x] - map[p.y][p.x - 1] < 2 {
        n.push(Pos { x: p.x - 1, y: p.y });
    }
    if p.x < map[0].len() - 1 && map[p.y][p.x] - map[p.y][p.x + 1] < 2 {
        n.push(Pos { x: p.x + 1, y: p.y });
    }
    if p.y > 0 && map[p.y][p.x] - map[p.y - 1][p.x] < 2 {
        n.push(Pos { x: p.x, y: p.y - 1 });
    }
    if p.y < map.len() - 1 && map[p.y][p.x] - map[p.y + 1][p.x] < 2 {
        n.push(Pos { x: p.x, y: p.y + 1 });
    }
    n
}

fn a_star(start: Pos, end: Option<Pos>, map: &Vec<Vec<i32>>) -> usize {
    let mut open_set = HashSet::<Pos>::new();
    open_set.insert(start);

    let mut came_from = HashMap::<Pos, Pos>::new();

    let mut g_score = HashMap::<Pos, i32>::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::<Pos, i32>::new();
    f_score.insert(start, h(start, end));

    while !open_set.is_empty() {
        // let oc = open.clone();
        let current = *open_set
            .clone()
            .iter()
            .sorted_by(|a, b| f_score.get(a).unwrap().cmp(f_score.get(b).unwrap()))
            .next()
            .unwrap();
        if is_goal(current, end) {
            return reconstruct_path(&came_from, &current).len() - 1;
        }
        open_set.remove(&current);
        for neighbour in neighbours(current, map) {
            let tentative = g_score[&current] + 1;
            if match g_score.get(&neighbour) {
                Some(g) => tentative < *g,
                None => true,
            } {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative);
                f_score.insert(neighbour, tentative + h(neighbour, end));
                open_set.insert(neighbour);
            }
        }
    }
    panic!("no path found");
}

pub fn shortest_path(filename: &str) -> usize {
    let input = parse_file(filename);
    a_star(input.end, Some(input.start), &input.map)
}

pub fn shortest_path_from_any_a(filename: &str) -> usize {
    let input = parse_file(filename);
    a_star(input.end, None, &input.map)
}

#[cfg(test)]
mod tests {
    use super::{shortest_path, shortest_path_from_any_a};
    use test_case::test_case;

    #[test_case("../testinput/day12.txt", 31; "on test input")]
    #[test_case("../input/day12.txt", 534; "on real input")]
    fn part1(filename: &str, expected: usize) {
        assert_eq!(shortest_path(filename), expected);
    }
    #[test_case("../testinput/day12.txt", 29; "on test input")]
    #[test_case("../input/day12.txt", 525; "on real input")]
    fn part2(filename: &str, expected: usize) {
        assert_eq!(shortest_path_from_any_a(filename), expected);
    }
}
