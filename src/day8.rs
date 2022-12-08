use itertools::Itertools;
use std::fs::read_to_string;

type Grid = Vec<Vec<u32>>;

fn parse_file(filename: &str) -> Grid {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|d| d.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

fn is_visible(x: usize, y: usize, grid: &Grid) -> bool {
    let size = grid.len();
    if x == 0 || x == size - 1 || y == 0 || y == size - 1 {
        return true;
    }
    let height = grid[y][x];

    // Ugly ||-chain to enable short circuit when first visible is found
    grid[y].get(0..x).unwrap().iter().all(|h| *h < height)
        || grid[y]
            .get(x + 1..size)
            .unwrap()
            .iter()
            .all(|h| *h < height)
        || grid
            .get(0..y)
            .unwrap()
            .iter()
            .map(|row| row[x])
            .all(|h| h < height)
        || grid
            .get(y + 1..size)
            .unwrap()
            .iter()
            .map(|row| row[x])
            .all(|h| h < height)
}

fn score(x: usize, y: usize, grid: &Grid) -> u64 {
    let size = grid.len();
    if x == 0 || x == size - 1 || y == 0 || y == size - 1 {
        return 0;
    }
    let height = grid[y][x];

    let left = match grid[y]
        .get(0..x)
        .unwrap()
        .iter()
        .rev()
        .enumerate()
        .find(|(_, h)| **h >= height)
    {
        Some((i, _)) => i + 1,
        None => x,
    } as u64;

    let right = match grid[y]
        .get(x + 1..size)
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, h)| **h >= height)
    {
        Some((i, _)) => i + 1,
        None => size - x - 1,
    } as u64;

    let up = match grid
        .get(0..y)
        .unwrap()
        .iter()
        .map(|row| row[x])
        .rev()
        .enumerate()
        .find(|(_, h)| *h >= height)
    {
        Some((i, _)) => i + 1,
        None => y,
    } as u64;

    let down = match grid
        .get(y + 1..size)
        .unwrap()
        .iter()
        .map(|row| row[x])
        .enumerate()
        .find(|(_, h)| *h >= height)
    {
        Some((i, _)) => i + 1,
        None => size - y - 1,
    } as u64;

    left * right * up * down
}

pub fn visible(filename: &str) -> u64 {
    let grid = parse_file(filename);
    let size = grid.len();
    (0..size)
        .map(|x| (0..size).filter(|y| is_visible(x, *y, &grid)).count() as u64)
        .sum()
}

pub fn highest_score(filename: &str) -> u64 {
    let grid = parse_file(filename);
    let size = grid.len();
    (0..size)
        .map(|x| (0..size).map(|y| score(x, y, &grid)).max().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day8::{highest_score, visible};

    #[test]
    fn part1() {
        assert_eq!(21, visible("testinput/day8.txt"));
        assert_eq!(1798, visible("input/day8.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(8, highest_score("testinput/day8.txt"));
        assert_eq!(259_308, highest_score("input/day8.txt"));
    }
}
