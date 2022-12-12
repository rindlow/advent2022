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
    if p.x > 0 && map[p.y][p.x - 1] - map[p.y][p.x] < 2 {
        n.push(Pos { x: p.x - 1, y: p.y });
    }
    if p.x < map[0].len() - 1 && map[p.y][p.x + 1] - map[p.y][p.x] < 2 {
        n.push(Pos { x: p.x + 1, y: p.y });
    }
    if p.y > 0 && map[p.y - 1][p.x] - map[p.y][p.x] < 2 {
        n.push(Pos { x: p.x, y: p.y - 1 });
    }
    if p.y < map.len() - 1 && map[p.y + 1][p.x] - map[p.y][p.x] < 2 {
        n.push(Pos { x: p.x, y: p.y + 1 });
    }
    n
}

fn h(a: Pos, b: Pos) -> i32 {
    (i32::try_from(a.x).unwrap() - i32::try_from(b.x).unwrap()).abs()
        + (i32::try_from(a.y).unwrap() - i32::try_from(b.y).unwrap()).abs()
}

fn arrow(a: Pos, b: Pos) -> char {
    if a.x < b.x {
        '>'
    } else if a.x > b.x {
        '<'
    } else if a.y < b.y {
        'v'
    } else {
        '^'
    }
}

fn print_map(map: &Vec<Vec<i32>>, path: &[Pos]) {
    // println!("{:?}", path);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = Pos { x, y };
            if path[path.len() - 1] == pos {
                print!("E");
            } else if path.contains(&pos) {
                print!(
                    "{}",
                    arrow(pos, path[path.iter().position(|p| *p == pos).unwrap() + 1])
                );
            } else {
                print!(".");
                // print!("{}", u8::try_from(map[y][x]).unwrap() as char);
            }
        }
        println!();
    }
    println!();
}

fn print_f(f: &HashMap<Pos, i32>, map: &Vec<Vec<i32>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let pos = Pos { x, y };
            if f.contains_key(&pos) {
                print!("{:3}", f[&pos]);
            } else {
                print!(" ..");
            }
        }
        println!();
    }
    println!();
}

pub fn shortest_path(filename: &str) -> usize {
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
    a_star(start, end, &map)
}

pub fn shortest_path_from_any_a(filename: &str) -> usize {
    let mut starts = Vec::<Pos>::new();
    let mut end = Pos { x: 0, y: 0 };
    let map = read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' || (c == 'a' && x == 0) {
                        starts.push(Pos { x, y });
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
    starts
        .iter()
        .map(|start| a_star(*start, end, &map))
        .min()
        .unwrap()
}

fn a_star(start: Pos, end: Pos, map: &Vec<Vec<i32>>) -> usize {
    let mut open = HashSet::<Pos>::new();
    open.insert(start);

    let mut came_from = HashMap::<Pos, Pos>::new();

    let mut g_score = HashMap::<Pos, i32>::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::<Pos, i32>::new();
    f_score.insert(start, h(start, end));
    let mut current: &Pos;
    while !open.is_empty() {
        let oc = open.clone();
        current = oc
            .iter()
            .sorted_by(|a, b| f_score.get(a).unwrap().cmp(f_score.get(b).unwrap()))
            .next()
            .unwrap();
        if *current == end {
            let path = reconstruct_path(&came_from, current);
            // print_map(map, &path);
            return path.len() - 1;
        }
        open.remove(current);
        for neighbour in neighbours(*current, map) {
            let tentative = g_score[current] + 1;
            let g = match g_score.get(&neighbour) {
                Some(x) => *x,
                None => 1_000_000,
            };
            if tentative < g {
                came_from.insert(neighbour, *current);
                g_score.insert(neighbour, tentative);
                f_score.insert(neighbour, tentative + h(neighbour, end));
                open.insert(neighbour);
            }
        }

        // print_f(&f_score, map);
    }
    panic!("no path found");
}

#[cfg(test)]
mod tests {
    use super::{shortest_path, shortest_path_from_any_a};

    #[test]
    fn part1() {
        assert_eq!(31, shortest_path("../testinput/day12.txt"));
        assert_eq!(534, shortest_path("../input/day12.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(29, shortest_path_from_any_a("../testinput/day12.txt"));
        assert_eq!(525, shortest_path_from_any_a("../input/day12.txt"));
    }
}
