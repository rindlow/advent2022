use fxhash::FxHashSet as HashSet;
use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

fn neighbours(cube: &Coord) -> Vec<Coord> {
    vec![
        Coord {
            x: cube.x - 1,
            ..*cube
        },
        Coord {
            x: cube.x + 1,
            ..*cube
        },
        Coord {
            y: cube.y - 1,
            ..*cube
        },
        Coord {
            y: cube.y + 1,
            ..*cube
        },
        Coord {
            z: cube.z - 1,
            ..*cube
        },
        Coord {
            z: cube.z + 1,
            ..*cube
        },
    ]
}

fn parse_file(filename: &str) -> HashSet<Coord> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let mut splits = line.split(',');
            Coord {
                x: splits.next().unwrap().parse().unwrap(),
                y: splits.next().unwrap().parse().unwrap(),
                z: splits.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

pub fn surface_area(filename: &str) -> usize {
    let droplet = parse_file(filename);
    droplet
        .iter()
        .map(|d| {
            neighbours(d)
                .iter()
                .filter(|c| !droplet.contains(*c))
                .count()
        })
        .sum()
}

pub fn external_surface_area(filename: &str) -> usize {
    let droplet = parse_file(filename);
    let x_min = droplet.iter().map(|c| c.x).min().unwrap();
    let x_max = droplet.iter().map(|c| c.x).max().unwrap();
    let y_min = droplet.iter().map(|c| c.y).min().unwrap();
    let y_max = droplet.iter().map(|c| c.y).max().unwrap();
    let z_min = droplet.iter().map(|c| c.z).min().unwrap();
    let z_max = droplet.iter().map(|c| c.z).max().unwrap();

    let mut queue = VecDeque::<Coord>::new();
    let mut steam: HashSet<Coord> = HashSet::default();

    queue.push_back(Coord {
        x: x_min,
        y: y_min,
        z: z_min,
    });

    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();

        for space in neighbours(&coord).iter().filter(|c| {
            c.x >= x_min - 1
                && c.x <= x_max + 1
                && c.y >= y_min - 1
                && c.y <= y_max + 1
                && c.z >= z_min - 1
                && c.z <= z_max + 1
                && !droplet.contains(c)
        }) {
            if !steam.contains(space) {
                queue.push_back(space.clone());
                steam.insert(space.clone());
            }
        }
    }

    droplet
        .iter()
        .map(|d| neighbours(d).iter().filter(|c| steam.contains(*c)).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{external_surface_area, surface_area};
    use test_case::test_case;

    #[test_case("../testinput/day18.txt", 64; "on test input")]
    #[test_case("../input/day18.txt", 4580; "on real input")]
    fn part1(filename: &str, expected: usize) {
        assert_eq!(surface_area(filename), expected);
    }
    #[test_case("../testinput/day18.txt", 58; "on test input")]
    #[test_case("../input/day18.txt", 2610; "on real input")]
    fn part2(filename: &str, expected: usize) {
        assert_eq!(external_surface_area(filename), expected);
    }
}
