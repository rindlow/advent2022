#![warn(clippy::pedantic)]
#![allow(clippy::too_many_lines)]

use lazy_st::{lazy, Lazy};
use std::env;
use std::fmt;
use std::time::Instant;

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
            part1: lazy!(aoc2022::day1::most_calories_from_file("input/day1.txt").to_string()),
            part2: lazy!(
                aoc2022::day1::three_most_calories_from_file("input/day1.txt").to_string()
            ),
        },
        Day {
            no: 2,
            part1: lazy!(aoc2022::day2::score_selected("input/day2.txt").to_string()),
            part2: lazy!(aoc2022::day2::score_end("input/day2.txt").to_string()),
        },
        Day {
            no: 3,
            part1: lazy!(aoc2022::day3::rucksack("input/day3.txt").to_string()),
            part2: lazy!(aoc2022::day3::badges("input/day3.txt").to_string()),
        },
        Day {
            no: 4,
            part1: lazy!(aoc2022::day4::fully_contain("input/day4.txt").to_string()),
            part2: lazy!(aoc2022::day4::overlap("input/day4.txt").to_string()),
        },
        Day {
            no: 5,
            part1: lazy!(aoc2022::day5::crates_single("input/day5.txt")),
            part2: lazy!(aoc2022::day5::crates_multiple("input/day5.txt")),
        },
        Day {
            no: 6,
            part1: lazy!(aoc2022::day6::start_of_package("input/day6.txt").to_string()),
            part2: lazy!(aoc2022::day6::start_of_message("input/day6.txt").to_string()),
        },
        Day {
            no: 7,
            part1: lazy!(aoc2022::day7::sum_dirs_below("input/day7.txt").to_string()),
            part2: lazy!(aoc2022::day7::dir_to_delete("input/day7.txt").to_string()),
        },
        Day {
            no: 8,
            part1: lazy!(aoc2022::day8::visible("input/day8.txt").to_string()),
            part2: lazy!(aoc2022::day8::highest_score("input/day8.txt").to_string()),
        },
        Day {
            no: 9,
            part1: lazy!(aoc2022::day9::visited_nodes("input/day9.txt", 2).to_string()),
            part2: lazy!(aoc2022::day9::visited_nodes("input/day9.txt", 10).to_string()),
        },
        Day {
            no: 10,
            part1: lazy!(aoc2022::day10::signal_strength("input/day10.txt").to_string()),
            part2: lazy!(aoc2022::day10::crt("input/day10.txt")),
        },
        Day {
            no: 11,
            part1: lazy!(aoc2022::day11::monkey_business("input/day11.txt", true).to_string()),
            part2: lazy!(aoc2022::day11::monkey_business("input/day11.txt", false).to_string()),
        },
        Day {
            no: 12,
            part1: lazy!(aoc2022::day12::shortest_path("input/day12.txt").to_string()),
            part2: lazy!(aoc2022::day12::shortest_path_from_any_a("input/day12.txt").to_string()),
        },
        Day {
            no: 13,
            part1: lazy!(aoc2022::day13::sum_indices("input/day13.txt").to_string()),
            part2: lazy!(aoc2022::day13::decoder_key("input/day13.txt").to_string()),
        },
        Day {
            no: 14,
            part1: lazy!(aoc2022::day14::sand_before_abyss("input/day14.txt").to_string()),
            part2: lazy!(aoc2022::day14::sand_to_rest("input/day14.txt").to_string()),
        },
        Day {
            no: 15,
            part1: lazy!(aoc2022::day15::cannot_contain("input/day15.txt", 2_000_000).to_string()),
            part2: lazy!(
                aoc2022::day15::tuning_frequency("input/day15.txt", 4_000_000).to_string()
            ),
        },
        Day {
            no: 16,
            part1: lazy!(aoc2022::day16::impl1("input/day16.txt").to_string()),
            part2: lazy!(String::new()),
        },
        Day {
            no: 16,
            part1: lazy!(String::new()),
            part2: lazy!(String::new()),
        },
        Day {
            no: 18,
            part1: lazy!(aoc2022::day18::surface_area("input/day18.txt").to_string()),
            part2: lazy!(aoc2022::day18::external_surface_area("input/day18.txt").to_string()),
        },
        Day {
            no: 19,
            part1: lazy!(String::new()),
            part2: lazy!(String::new()),
        },
        Day {
            no: 20,
            part1: lazy!(String::new()),
            part2: lazy!(String::new()),
        },
        Day {
            no: 21,
            part1: lazy!(aoc2022::day21::monkey_yell("input/day21.txt").to_string()),
            part2: lazy!(aoc2022::day21::equality_test("input/day21.txt").to_string()),
        },
        Day {
            no: 22,
            part1: lazy!(String::new()),
            part2: lazy!(String::new()),
        },
        Day {
            no: 23,
            part1: lazy!(String::new()),
            part2: lazy!(String::new()),
        },
        Day {
            no: 24,
            part1: lazy!(String::new()),
            part2: lazy!(String::new()),
        },
        Day {
            no: 25,
            part1: lazy!(aoc2022::day25::sum_snafu("input/day25.txt")),
            part2: lazy!(String::new()),
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
