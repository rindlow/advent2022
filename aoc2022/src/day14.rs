use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
    fs::read_to_string,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Block {
    Air,
    Rock,
    Sand,
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Air => write!(f, "."),
            Block::Rock => write!(f, "#"),
            Block::Sand => write!(f, "o"),
        }
    }
}

type Map = HashMap<Pos, Block>;

fn parse_file(filename: &str) -> Map {
    let mut map: Map = HashMap::new();
    read_to_string(filename).unwrap().lines().for_each(|line| {
        let splits = line.split(" -> ");
        let mut last: Option<Pos> = None;
        for split in splits {
            let mut coords = split.split(',');
            let pos = Pos {
                x: coords.next().unwrap().parse::<usize>().unwrap(),
                y: coords.next().unwrap().parse::<usize>().unwrap(),
            };
            if let Some(ref lastpos) = last {
                if pos.x == lastpos.x {
                    for y in min(pos.y, lastpos.y)..=max(pos.y, lastpos.y) {
                        map.insert(Pos { x: pos.x, y }, Block::Rock);
                    }
                }
                if pos.y == lastpos.y {
                    for x in min(pos.x, lastpos.x)..=max(pos.x, lastpos.x) {
                        map.insert(Pos { x, y: pos.y }, Block::Rock);
                    }
                }
            }
            last = Some(pos);
        }
    });
    map
}

fn block_at(pos: &Pos, map: &Map) -> Block {
    if let Some(block) = map.get(pos) {
        block.clone()
    } else {
        Block::Air
    }
}
fn block_at2(pos: &Pos, map: &Map, floor: usize) -> Block {
    if pos.y == floor {
        Block::Rock
    } else if let Some(block) = map.get(pos) {
        block.clone()
    } else {
        Block::Air
    }
}

fn sand_pos(map: &Map) -> Option<Pos> {
    let mut x: usize = 500;
    let max_y = map.keys().map(|p| p.y).max().unwrap();
    for y in 0..=max_y {
        if block_at(&Pos { x, y }, map) != Block::Air {
            if block_at(&Pos { x: x - 1, y }, map) == Block::Air {
                x -= 1;
            } else if block_at(&Pos { x: x + 1, y }, map) == Block::Air {
                x += 1;
            } else {
                return Some(Pos { x, y: y - 1 });
            }
        }
    }
    None
}

fn sand_pos2(map: &Map, floor: usize) -> Option<Pos> {
    let mut x: usize = 500;
    for y in 0..=floor {
        if block_at2(&Pos { x, y }, map, floor) != Block::Air {
            if block_at2(&Pos { x: x - 1, y }, map, floor) == Block::Air {
                x -= 1;
            } else if block_at2(&Pos { x: x + 1, y }, map, floor) == Block::Air {
                x += 1;
            } else {
                return Some(Pos { x, y: y - 1 });
            }
        }
    }
    None
}

pub fn sand_before_abyss(filename: &str) -> i32 {
    let mut map = parse_file(filename);
    let mut i = 0;
    loop {
        if let Some(pos) = sand_pos(&map) {
            map.insert(pos, Block::Sand);
            i += 1;
        } else {
            return i;
        }
    }
}

pub fn sand_to_rest(filename: &str) -> i32 {
    let mut map = parse_file(filename);
    let max_y = map.keys().map(|p| p.y).max().unwrap();
    let mut i = 0;
    loop {
        if let Some(pos) = sand_pos2(&map, max_y + 2) {
            if pos.x == 500 && pos.y == 0 {
                return i + 1;
            }
            map.insert(pos, Block::Sand);
            i += 1;
        } else {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sand_before_abyss, sand_to_rest};
    use test_case::test_case;

    #[test_case("../testinput/day14.txt", 24; "on test input")]
    #[test_case("../input/day14.txt", 638; "on real input")]
    fn part1(filename: &str, expected: i32) {
        assert_eq!(sand_before_abyss(filename), expected);
    }
    #[test_case("../testinput/day14.txt", 93; "on test input")]
    #[test_case("../input/day14.txt", 31722; "on real input")]
    fn part2(filename: &str, expected: i32) {
        assert_eq!(sand_to_rest(filename), expected);
    }
}
