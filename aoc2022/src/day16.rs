use fxhash::FxHashMap as HashMap;
use std::{collections::VecDeque, fs::read_to_string};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Valve {
    flow: u32,
    tunnels: Vec<String>,
    open: bool,
}

#[derive(Debug)]
struct QueueItem {
    valve: String,
    minutes: u32,
    pressure: u32,
    increase: u32,
    came_from: Vec<String>,
    valves: Map,
}

type Map = HashMap<String, Valve>;

fn get_num(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| -> u32 {
        if c.is_numeric() {
            10 * acc + c.to_digit(10).unwrap()
        } else {
            acc
        }
    })
}

fn parse_file(filename: &str) -> Map {
    let mut map: Map = HashMap::default();
    for line in read_to_string(filename).unwrap().lines() {
        let split = line.split(' ').collect_vec();
        let name = split[1].to_string();
        let flow = get_num(split[4]);
        let tunnels = split
            .get(9..)
            .unwrap()
            .iter()
            .map(|t| {
                if t.contains(',') {
                    t.get(..t.len() - 1).unwrap().to_string()
                } else {
                    (*t).to_string()
                }
            })
            .collect_vec();
        map.insert(
            name.to_string(),
            Valve {
                flow,
                tunnels,
                open: false,
            },
        );
    }
    map
}

fn path(valves: &Map) -> u32 {
    let mut queue = VecDeque::<QueueItem>::new();

    queue.push_back(QueueItem {
        valve: "AA".to_string(),
        minutes: 0,
        pressure: 0,
        increase: 0,
        came_from: Vec::<String>::new(),
        valves: valves.clone(),
    });

    let mut max_pressure: u32 = 0;

    while !queue.is_empty() {
        println!("queue.len() = {}", queue.len());
        let item = queue.pop_front().unwrap();
        println!(
            "minute {}: looking at QueueItem {} pressure: {}, increase: {}, {:?}",
            item.minutes, item.valve, item.pressure, item.increase, item.came_from
        );

        if item.pressure > max_pressure {
            max_pressure = item.pressure;
        }

        if item.minutes < 30 {
            let valve = item.valves.get(&item.valve).unwrap();
            let mut path = item.came_from.clone();
            path.push(item.valve.clone());

            for dest in &valve.tunnels {
                if let Some(last) = item.came_from.iter().last() {
                    println!("dest = {}, came_from.last = {}", dest, last);
                } else {
                    println!("no last");
                }
                if item.came_from.is_empty() || *dest != *item.came_from.iter().last().unwrap() {
                    queue.push_back(QueueItem {
                        valve: dest.to_string(),
                        minutes: item.minutes + 1,
                        pressure: item.pressure + item.increase,
                        increase: item.increase,
                        came_from: path.clone(),
                        valves: item.valves.clone(),
                    });
                }
            }
            if item.minutes < 29 && !valve.open && valve.flow > 0 {
                println!("opening valve {}", item.valve);
                path.push(format!("open({})", item.valve));
                let mut mutvalves = item.valves.clone();
                mutvalves
                    .entry(item.valve.clone())
                    .and_modify(|v| v.open = true);
                for dest in &valve.tunnels {
                    queue.push_back(QueueItem {
                        valve: dest.to_string(),
                        minutes: item.minutes + 2,
                        pressure: item.pressure + 2 * item.increase + valve.flow,
                        increase: item.increase + valve.flow,
                        came_from: path.clone(),
                        valves: mutvalves.clone(),
                    });
                }
            }
        }
        // println!("queue is now {:?}", queue);
    }
    max_pressure
}

//     println!(
//         "path({}, {}, {}, {}, ...)",
//         name, minute, pressure, came_from
//     );
//     if minute >= 30 {
//         println!("Time's up, returning {}", pressure);
//         return pressure;
//     }
//     let valve = valves.get(name).unwrap();
//     let increase: u32 = valves
//         .values()
//         .map(|v| if v.open { v.flow } else { 0 })
//         .sum();
//     let mut x = valve
//         .tunnels
//         .iter()
//         .filter(|dst| *dst != came_from)
//         .map(|dst| path(dst, minute + 1, pressure + increase, name, valves))
//         .collect_vec();
//     if !valve.open && valve.flow > 0 {
//         let mut mutvalves = (*valves).clone();
//         mutvalves
//             .entry(name.to_string())
//             .and_modify(|v| v.open = true);
//         let y = valve
//             .tunnels
//             .iter()
//             .map(|dst| path(dst, minute + 2, pressure + increase, name, &mutvalves));
//         x.extend(y);
//     }
//     if let Some(p) = x.iter().max() {
//         println!("returning max = {}", p);
//         return *p;
//     }
//     0
// }

pub fn impl1(filename: &str) -> u32 {
    let valves = parse_file(filename);
    path(&valves)
}

#[cfg(test)]
mod tests {
    use super::impl1;
    use test_case::test_case;

    #[test_case("../testinput/day16.txt", 1651; "on test input")]
    // #[test_case("../input/day16.txt", 0; "on real input")]
    fn part1(filename: &str, expected: u32) {
        assert_eq!(impl1(filename), expected);
    }
}
