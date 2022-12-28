use itertools::Itertools;
use std::fs::read_to_string;

type Object = Vec<u8>;

struct Rock {
    sprite: Object,
    width: usize,
}

fn make_shapes() -> Vec<Rock> {
    vec![
        Rock {
            sprite: vec![15],
            width: 4,
        },
        Rock {
            sprite: vec![2, 7, 2],
            width: 3,
        },
        Rock {
            sprite: vec![7, 1, 1],
            width: 3,
        },
        Rock {
            sprite: vec![1, 1, 1, 1],
            width: 1,
        },
        Rock {
            sprite: vec![3, 3],
            width: 2,
        },
    ]
}

fn can_move_left(rock: &Rock, x: usize, y: usize, chamber: &Object) -> bool {
    if x == 0 {
        return false;
    }
    for i in 0..rock.sprite.len() {
        if y + i < chamber.len() && chamber[y + i] & rock.sprite[i] << (7 - x - rock.width + 1) > 0
        {
            return false;
        }
    }
    true
}

fn can_move_right(rock: &Rock, x: usize, y: usize, chamber: &Object) -> bool {
    if x + rock.width > 6 {
        return false;
    }
    for i in 0..rock.sprite.len() {
        if y + i < chamber.len()
            && (chamber[y + i] & (rock.sprite[i] << (7 - x - rock.width - 1)) > 0)
        {
            // println!("false");
            return false;
        }
    }
    true
}

fn can_move_down(rock: &Rock, x: usize, y: usize, chamber: &Object) -> bool {
    if y == 0 {
        return false;
    }
    for i in 0..rock.sprite.len() {
        if chamber[y - 1] & rock.sprite[i] << (7 - x - rock.width) > 0 {
            return false;
        }
    }
    true
}

fn print_chamber(chamber: &Object, rock: &Rock, x: usize, y: usize) {
    for (rowno, row) in chamber.iter().enumerate().rev() {
        print!("{:4} ", rowno);
        for i in 0..7 {
            if rowno >= y
                && rowno < y + rock.sprite.len()
                && (rock.sprite[rowno - y] << (7 - x - rock.width) & (1 << (6 - i))) > 0
            {
                print!("@");
            } else if (*row & (1 << (6 - i))) > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn impl1(filename: &str) -> usize {
    let gusts = read_to_string(filename).unwrap().chars().collect_vec();
    let mut gustindex: usize = 0;
    let mut chamber: Object = Vec::new();
    let rocks = make_shapes();

    for i in 0..2022 {
        let top: i32 = if let Some(rowno) = chamber.iter().rposition(|row| *row > 0) {
            i32::try_from(rowno).unwrap()
        } else {
            -1
        };
        // println!("top = {}", top);
        chamber.resize(usize::try_from(top + 8).unwrap(), 0);
        println!("new size of chamber = {}", chamber.len());
        let mut x = 2;
        let mut y = usize::try_from(top + 4).unwrap();
        let rock = &rocks[i % 5];
        println!("Rock {} begins falling at {}, {}", i, x, y);
        print_chamber(&chamber, rock, x, y);
        loop {
            let gust = gusts[gustindex];
            println!("{}", gust);
            gustindex = (gustindex + 1) % gusts.len();
            if gust == '<' && can_move_left(rock, x, y, &chamber) {
                // println!("push left");
                x -= 1;
            } else if gust == '>' && can_move_right(rock, x, y, &chamber) {
                // println!("push right");
                x += 1;
            }
            if can_move_down(rock, x, y, &chamber) {
                // println!("falls down");
                y -= 1;
            } else {
                println!("hit rock at {}, {}", x, y);
                print_chamber(&chamber, rock, x, y);
                for i in 0..rock.sprite.len() {
                    chamber[y + i] |= rock.sprite[i] << (7 - x - rock.width);
                }
                // println!("chamber = {:?}", chamber);
                println!(
                    "after {} rocks, tower height = {}",
                    i + 1,
                    chamber.iter().rposition(|row| *row > 0).unwrap() + 1
                );
                break;
            }
        }
    }
    chamber.iter().rposition(|row| *row > 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::impl1;
    use test_case::test_case;

    #[test_case("../testinput/day17.txt", 3068; "on test input")]
    // #[test_case("../input/day17.txt", 0; "on real input")]
    fn part1(filename: &str, expected: usize) {
        assert_eq!(impl1(filename), expected);
    }
}
