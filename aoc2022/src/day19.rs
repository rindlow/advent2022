#![allow(clippy::too_many_lines)]

use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::VecDeque,
    fs::read_to_string,
    ops::{Add, Sub},
};

#[derive(Debug, Clone)]
struct Blueprint {
    id: u32,
    ore_cost: Materials,
    clay_cost: Materials,
    obsidian_cost: Materials,
    geode_cost: Materials,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Materials {
    ore: u32,
    clay: u32,
    obsidian: u32,
}
impl Ord for Materials {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ore < other.ore || self.clay < other.clay || self.obsidian < other.obsidian {
            Ordering::Less
        } else if self.ore > other.ore || self.clay > other.clay || self.obsidian > other.obsidian {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Materials {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a, 'b> Sub<&'b Materials> for &'a Materials {
    type Output = Materials;

    fn sub(self, other: &'b Materials) -> Self::Output {
        Materials {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
        }
    }
}
impl<'a, 'b> Add<&'b Materials> for &'a Materials {
    type Output = Materials;

    fn add(self, other: &'b Materials) -> Self::Output {
        Materials {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct QueueItem {
    id: u32,
    minute: u32,
    purse: Materials,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    geodes: u32,
}

// blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 3 ore and 16 obsidian.
fn parse_file(filename: &str) -> Vec<Blueprint> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let split = line.split(' ').collect_vec();
            Blueprint {
                id: split[1][0..split[1].len() - 1].parse::<u32>().unwrap(),
                ore_cost: Materials {
                    ore: split[6].parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                clay_cost: Materials {
                    ore: split[12].parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                obsidian_cost: Materials {
                    ore: split[18].parse().unwrap(),
                    clay: split[21].parse().unwrap(),
                    obsidian: 0,
                },
                geode_cost: Materials {
                    ore: split[27].parse().unwrap(),
                    clay: 0,
                    obsidian: split[30].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn largest_number_of_geodes(blueprint: &Blueprint) -> u32 {
    let mut queue = VecDeque::<QueueItem>::new();
    let mut next_id: u32 = 0;

    queue.push_back(QueueItem {
        id: next_id,
        minute: 1,
        purse: Materials {
            ore: 0,
            clay: 0,
            obsidian: 0,
        },
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
        geodes: 0,
    });

    let mut max_geodes = 0;
    let mut visited: HashSet<QueueItem> = HashSet::default();

    while !queue.is_empty() {
        let item = queue.pop_front().unwrap();
        if visited.contains(&item) {
            continue;
        }
        visited.insert(item.clone());
        //  println!("{}: minute {} - {:?}", item.id, item.minute, item);
        let geodes = item.geodes + item.geode_robots;
        //  println!("  geodes now {}", geodes);
        if geodes > max_geodes {
            //  println!("  new max_geodes = {}", geodes);
            max_geodes = geodes;
        }
        if item.minute >= 24 {
            continue;
        }
        // if item.purse.ore > 12 || item.purse.clay > 50 {
        //     continue;
        // }

        // if item.minute > 22 && geodes == 0 {
        //     continue;
        // }

        let income = Materials {
            ore: item.ore_robots,
            clay: item.clay_robots,
            obsidian: item.obsidian_robots,
        };
        //  println!("  income this minute = {:?}", income);
        let purse = &item.purse + &income;
        //  println!("  materials availible = {:?}", purse);

        let mut built = 0;

        if item.minute < 10 && item.purse >= blueprint.ore_cost && item.ore_robots < 6 {
            built += 1;
            next_id += 1;
            //  println!("  {} is building ore robot", next_id);
            queue.push_back(QueueItem {
                id: next_id,
                minute: item.minute + 1,
                purse: &purse - &blueprint.ore_cost,
                ore_robots: item.ore_robots + 1,
                geodes,
                ..item
            });
        }
        if item.minute < 18 && item.purse >= blueprint.clay_cost && item.clay_robots < 8 {
            built += 1;
            next_id += 1;
            //  println!("  {} is building clay robot", next_id);
            queue.push_back(QueueItem {
                id: next_id,
                minute: item.minute + 1,
                purse: &purse - &blueprint.clay_cost,
                clay_robots: item.clay_robots + 1,
                geodes,
                ..item
            });
        }
        if item.minute < 21 && item.purse >= blueprint.obsidian_cost && item.obsidian_robots < 9 {
            built += 1;
            next_id += 1;
            //  println!("  {} is building obsidian robot", next_id);
            queue.push_back(QueueItem {
                id: next_id,
                minute: item.minute + 1,
                purse: &purse - &blueprint.obsidian_cost,
                obsidian_robots: item.obsidian_robots + 1,
                geodes,
                ..item
            });
        }
        if item.purse >= blueprint.geode_cost {
            built += 1;
            next_id += 1;
            //  println!("  {} is building geode robot", next_id);
            queue.push_back(QueueItem {
                id: next_id,
                minute: item.minute + 1,
                purse: &purse - &blueprint.geode_cost,
                geode_robots: item.geode_robots + 1,
                geodes,
                ..item
            });
        }
        if built < 4 {
            // save materials for more expensive build
            next_id += 1;
            //  println!("  {} is stockpiling materials", next_id);
            queue.push_back(QueueItem {
                id: next_id,
                minute: item.minute + 1,
                purse,
                geodes,
                ..item
            });
        }
        // //  println!("queue.len now {}", queue.len());
    }
    println!("returning max_geodes = {}", max_geodes);
    max_geodes
}

pub fn impl1(filename: &str) -> u32 {
    parse_file(filename)
        .iter()
        .map(|blueprint| blueprint.id * largest_number_of_geodes(blueprint))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{impl1, largest_number_of_geodes, parse_file};
    use test_case::test_case;

    #[test]
    fn lnog0() {
        let bps = parse_file("../testinput/day19.txt");
        assert_eq!(9, largest_number_of_geodes(&bps[0]));
    }
    #[test]
    fn lnog1() {
        let bps = parse_file("../testinput/day19.txt");
        assert_eq!(12, largest_number_of_geodes(&bps[1]));
    }

    #[test_case("../testinput/day19.txt", 33; "on test input")]
    #[test_case("../input/day19.txt", 0; "on real input")]
    fn part1(filename: &str, expected: u32) {
        assert_eq!(impl1(filename), expected);
    }
}
