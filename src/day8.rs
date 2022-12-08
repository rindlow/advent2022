use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Grid {
    size: usize,
    rows: Vec<Vec<u32>>,
}

fn is_hidden(x: usize, y: usize, grid: &Grid) -> bool {
    if x == 0 || x == grid.size - 1 || y == 0 || y == grid.size - 1 {
        return false;
    }
    let height = grid.rows[y][x];
    let mut viz = true;
    for left in 0..x {
        if grid.rows[y][left] >= height {
            viz = false;
            break;
        }
    }
    if viz {
        return false;
    }
    viz = true;
    for up in 0..y {
        if grid.rows[up][x] >= height {
            viz = false;
            break;
        }
    }
    if viz {
        return false;
    }
    viz = true;
    for right in x + 1..grid.size {
        if grid.rows[y][right] >= height {
            viz = false;
            break;
        }
    }
    if viz {
        return false;
    }
    viz = true;
    for down in y + 1..grid.size {
        if grid.rows[down][x] >= height {
            viz = false;
            break;
        }
    }

    !viz
}

fn parse_file(filename: &str) -> Grid {
    let mut grid = Grid {
        size: 0,
        rows: vec![],
    };
    for line in read_to_string(filename).unwrap().lines() {
        let mut row: Vec<u32> = vec![];
        if grid.size == 0 {
            grid.size = line.len();
        }
        for tree in line.chars() {
            let height = tree.to_digit(10).unwrap();
            row.push(height);
        }
        grid.rows.push(row);
    }
    grid
}

pub fn visible(filename: &str) -> u64 {
    let grid = parse_file(filename);
    let n = grid.size as u64 * grid.size as u64;
    let viz: u64 = grid
        .rows
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, _)| is_hidden(*x, y, &grid))
                .count() as u64
        })
        .sum();
    n - viz
}

fn score(x: usize, y: usize, grid: &Grid) -> u64 {
    if x == 0 || x == grid.size - 1 || y == 0 || y == grid.size - 1 {
        return 0;
    }
    let height = grid.rows[y][x];
    let mut scr: u64 = 1;

    let mut n = x as u64;
    for xx in (0..x).rev() {
        if grid.rows[y][xx] >= height {
            n = x as u64 - xx as u64;
            break;
        }
    }
    scr *= n;

    n = y as u64;
    for yy in (0..y).rev() {
        if grid.rows[yy][x] >= height {
            n = y as u64 - yy as u64;
            break;
        }
    }
    scr *= n;

    n = grid.size as u64 - 1 - x as u64;
    for xx in x + 1..grid.size {
        if grid.rows[y][xx] >= height {
            n = xx as u64 - x as u64;
            break;
        }
    }
    scr *= n;

    n = grid.size as u64 - 1 - y as u64;
    for yy in y + 1..grid.size {
        if grid.rows[yy][x] >= height {
            n = yy as u64 - y as u64;
            break;
        }
    }
    scr *= n;

    scr
}

pub fn highest_score(filename: &str) -> u64 {
    let grid = parse_file(filename);
    let mut scores: Vec<u64> = vec![];
    for y in 0..grid.size {
        for x in 0..grid.size {
            scores.push(score(x, y, &grid));
        }
    }
    *scores.iter().max().unwrap()
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
