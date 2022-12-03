#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]

use lazy_st::{lazy, Lazy};
use std::env;
use std::fmt;
use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;

struct Day {
    no: i8,
    part1: Lazy<String>,
    part2: Lazy<String>,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Day {} ===", self.no)?;
        let now = Instant::now();
        writeln!(f, "Part 1: {}", *self.part1)?;
        writeln!(f, "  finished in {:.2?}", now.elapsed())?;
        let now = Instant::now();
        writeln!(f, "Part 2: {}", *self.part2)?;
        writeln!(f, "  finished in {:.2?}", now.elapsed())?;
        writeln!(f)
    }
}

fn main() {
    let days = vec![
        Day {
            no: 1,
            part1: lazy!(day1::most_calories_from_file("input/day1.txt").to_string()),
            part2: lazy!(day1::three_most_calories_from_file("input/day1.txt").to_string()),
        },
        Day {
            no: 2,
            part1: lazy!(day2::score_selected("input/day2.txt").to_string()),
            part2: lazy!(day2::score_end("input/day2.txt").to_string()),
        },
        Day {
            no: 3,
            part1: lazy!(day3::rucksack("input/day3.txt").to_string()),
            part2: lazy!(day3::badges("input/day3.txt").to_string()),
        },
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let idx = args[1].parse::<usize>().unwrap();
        if 0 < idx && idx <= days.len() {
            print!("{}", &days[idx - 1]);
        } else {
            println!("Day not implemented: {}", idx);
        }
    } else {
        let now = Instant::now();
        for day in &days {
            print!("{}", day);
        }
        println!("all finished in {:.2?}", now.elapsed());
    }
}
