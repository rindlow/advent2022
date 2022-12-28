#![allow(clippy::too_many_lines)]

use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use itertools::Itertools;
use std::{cmp::min, collections::VecDeque, fs::read_to_string};

#[derive(Debug, Clone)]
struct Edge {
    cost: u32,
    dest: String,
}

#[derive(Debug, Clone)]
struct Valve {
    flow: u32,
    tunnels: Vec<Edge>,
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

#[derive(Debug)]
struct DualQueueItem {
    pressure: u32,
    increase: u32,
    me: Visitor,
    elephant: Visitor,
    valves: Map,
    visited: HashSet<String>,
    opened: HashSet<String>,
}
#[derive(Debug, Clone)]
struct Visitor {
    minutes: u32,
    valve: String,
    came_from: Vec<String>,
}

#[derive(Debug, Clone)]
struct Move {
    visitor: Visitor,
    increase: u32,
    opened: Option<String>,
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
    // println!("digraph G {{");
    for line in read_to_string(filename).unwrap().lines() {
        let split = line.split(' ').collect_vec();
        let name = split[1].to_string();
        let flow = get_num(split[4]);
        // println!("  {} [label=\"{}: {}\"]", name, name, flow);
        let tunnels = split
            .get(9..)
            .unwrap()
            .iter()
            .map(|t| {
                if t.contains(',') {
                    Edge {
                        cost: 1,
                        dest: t.get(..t.len() - 1).unwrap().to_string(),
                    }
                } else {
                    Edge {
                        cost: 1,
                        dest: (*t).to_string(),
                    }
                }
            })
            .collect_vec();
        // for t in &tunnels {
        //     println!("  {} -> {}", name, t);
        // }
        map.insert(
            name.to_string(),
            Valve {
                flow,
                tunnels,
                open: false,
            },
        );
    }
    // println!("}}");
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

    let max_increase: u32 = valves.values().map(|valve| valve.flow).sum();
    let mut max_pressure: u32 = 0;

    while !queue.is_empty() {
        // println!("queue.len() = {}", queue.len());
        let item = queue.pop_front().unwrap();
        if item.minutes > 30 {
            continue;
        }
        // println!(
        //     "minute {}: looking at QueueItem {} pressure: {}, increase: {}, {:?}",
        //     item.minutes, item.valve, item.pressure, item.increase, item.came_from
        // );

        if item.minutes >= 7 && item.increase < 20 || item.minutes >= 13 && item.increase < 50 {
            // println!("  increase too low");
            continue;
        }

        if item.increase == max_increase {
            let final_pressure = item.pressure + (30 - item.minutes) * item.increase;
            // println!(
            //     "final_pressure = {} (max_pressure = {})",
            //     final_pressure, max_pressure
            // );
            if final_pressure > max_pressure {
                max_pressure = final_pressure;
            }
            continue;
        }

        if item.pressure > max_pressure {
            max_pressure = item.pressure;
        }

        if item.minutes < 30 {
            let valve = item.valves.get(&item.valve).unwrap();

            let mut path = item.came_from.clone();
            path.push(item.valve.clone());

            for tunnel in &valve.tunnels {
                // if let Some(last) = item.came_from.iter().last() {
                //     println!("dest = {}, came_from.last = {}", dest, last);
                // } else {
                //     println!("no last");
                // }
                if item.came_from.is_empty() || tunnel.dest != *item.came_from.last().unwrap() {
                    queue.push_back(QueueItem {
                        valve: tunnel.dest.to_string(),
                        minutes: item.minutes + tunnel.cost,
                        pressure: item.pressure + tunnel.cost * item.increase,
                        increase: item.increase,
                        came_from: path.clone(),
                        valves: item.valves.clone(),
                    });
                }
            }
            if item.minutes < 29 && !valve.open && valve.flow > 0 {
                // println!("opening valve {}", item.valve);
                let mut mutvalves = item.valves.clone();
                mutvalves
                    .entry(item.valve.clone())
                    .and_modify(|v| v.open = true);
                for tunnel in &valve.tunnels {
                    // println!("opening valve {} ({}). item.pressure = {}, tunnel.cost = {}, next pressure = {} ",
                    //     item.valve, valve.flow, item.pressure, tunnel.cost, item.pressure + (tunnel.cost + 1) * item.increase + valve.flow);
                    queue.push_back(QueueItem {
                        valve: tunnel.dest.to_string(),
                        minutes: item.minutes + tunnel.cost + 1,
                        pressure: item.pressure
                            + item.increase
                            + tunnel.cost * (item.increase + valve.flow),
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

fn dual_path(valves: &Map) -> u32 {
    let mut queue = VecDeque::<DualQueueItem>::new();

    queue.push_back(DualQueueItem {
        pressure: 0,
        increase: 0,
        valves: valves.clone(),
        me: Visitor {
            minutes: 1,
            valve: "AA".to_string(),
            came_from: Vec::<String>::new(),
        },
        elephant: Visitor {
            minutes: 1,
            valve: "AA".to_string(),
            came_from: Vec::<String>::new(),
        },
        visited: HashSet::default(),
        opened: HashSet::default(),
    });

    let max_increase: u32 = valves.values().map(|valve| valve.flow).sum();
    let mut max_pressure: u32 = 0;

    while !queue.is_empty() {
        // println!("queue.len() = {}", queue.len());
        let item = queue.pop_front().unwrap();

        let minutes = min(item.elephant.minutes, item.me.minutes);

        if minutes > 26 {
            continue;
        }
        println!(
            "Minute {}: looking at DualQueueItem pressure: {}, increase: {}, me: {} {} {:?}, elephant: {} {} {:?}, visited {:?}, opened {:?}",
            minutes, item.pressure, item.increase, item.me.minutes, item.me.valve,
            item.me.came_from, item.elephant.minutes, item.elephant.valve, item.elephant.came_from, item.visited, item.opened);

        let open: HashSet<String> = item
            .valves
            .iter()
            .filter(|(_, valve)| valve.open)
            .map(|(name, _)| name.clone())
            .collect();

        if item.visited.contains(&item.me.valve)
            && item.visited.contains(&item.elephant.valve)
            && open.is_subset(&item.opened)
        {
            println!("already been here");
            continue;
        }
        // if ((item.me.minutes >= 7 || item.elephant.minutes >= 7) && item.increase < 20)
        //     || (item.me.minutes >= 13 || item.elephant.minutes >= 13) && item.increase < 50
        // {
        //     println!("  increase too low");
        //     continue;
        // }

        if item.increase == max_increase {
            let final_pressure = item.pressure + (26 - minutes) * item.increase;
            println!(
                "final_pressure = {} (max_pressure = {})",
                final_pressure, max_pressure
            );
            if final_pressure > max_pressure {
                println!(
                    "max_pressure updated from {} to {}",
                    max_pressure, final_pressure
                );
                max_pressure = final_pressure;
            }
            continue;
        }

        if item.pressure > max_pressure {
            println!(
                "max_pressure updated from {} to {}",
                max_pressure, item.pressure
            );
            max_pressure = item.pressure;
        }

        if minutes < 26 {
            let mut my_moves = Vec::<Move>::new();
            if item.me.minutes == minutes {
                let my_valve = item.valves.get(&item.me.valve).unwrap();
                let mut my_path = item.me.came_from.clone();
                my_path.push(item.me.valve.clone());
                for tunnel in &my_valve.tunnels {
                    if item.me.came_from.is_empty()
                        || tunnel.dest != *item.me.came_from.last().unwrap()
                    {
                        my_moves.push(Move {
                            visitor: Visitor {
                                minutes: minutes + tunnel.cost,
                                valve: tunnel.dest.to_string(),
                                came_from: my_path.clone(),
                            },
                            increase: 0,
                            opened: None,
                        });
                    }
                }
                if minutes < 25 && !my_valve.open && my_valve.flow > 0 {
                    // println!("opening valve {}", item.valve);
                    for tunnel in &my_valve.tunnels {
                        // println!("opening valve {} ({}). item.pressure = {}, tunnel.cost = {}, next pressure = {} ",
                        //     item.valve, valve.flow, item.pressure, tunnel.cost, item.pressure + (tunnel.cost + 1) * item.increase + valve.flow);
                        my_moves.push(Move {
                            visitor: Visitor {
                                minutes: minutes + 1 + tunnel.cost,
                                valve: tunnel.dest.to_string(),
                                came_from: my_path.clone(),
                            },
                            increase: my_valve.flow,
                            opened: Some(item.me.valve.clone()),
                        });
                    }
                }
            } else {
                my_moves.push(Move {
                    visitor: item.me.clone(),
                    increase: 0,
                    opened: None,
                });
            }
            // println!("my_moves = {:?}", my_moves);

            let mut elephant_moves = Vec::<Move>::new();
            if item.elephant.minutes == minutes {
                let elephant_valve = item.valves.get(&item.elephant.valve).unwrap();
                let mut elephant_path = item.elephant.came_from.clone();
                elephant_path.push(item.elephant.valve.clone());
                for tunnel in &elephant_valve.tunnels {
                    if item.elephant.came_from.is_empty()
                        || tunnel.dest != *item.elephant.came_from.last().unwrap()
                    {
                        elephant_moves.push(Move {
                            visitor: Visitor {
                                minutes: minutes + tunnel.cost,
                                valve: tunnel.dest.to_string(),
                                came_from: elephant_path.clone(),
                            },
                            increase: 0,
                            opened: None,
                        });
                    }
                }
                if minutes < 25 && !elephant_valve.open && elephant_valve.flow > 0 {
                    // println!("opening valve {}", item.valve);
                    for tunnel in &elephant_valve.tunnels {
                        // println!("opening valve {} ({}). item.pressure = {}, tunnel.cost = {}, next pressure = {} ",
                        //     item.valve, valve.flow, item.pressure, tunnel.cost, item.pressure + (tunnel.cost + 1) * item.increase + valve.flow);
                        elephant_moves.push(Move {
                            visitor: Visitor {
                                minutes: minutes + 1 + tunnel.cost,
                                valve: tunnel.dest.to_string(),
                                came_from: elephant_path.clone(),
                            },
                            increase: elephant_valve.flow,
                            opened: Some(item.elephant.valve.clone()),
                        });
                    }
                }
            } else {
                elephant_moves.push(Move {
                    visitor: item.elephant.clone(),
                    increase: 0,
                    opened: None,
                });
            }
            // println!("elephant_moves = {:?}", elephant_moves);

            for my_move in my_moves {
                for elephant_move in &elephant_moves {
                    if my_move.opened.is_some()
                        && elephant_move.opened.is_some()
                        && my_move.opened == elephant_move.opened
                    {
                        continue;
                    }
                    let mut opened = item.opened.clone();
                    let mut mutvalves = item.valves.clone();
                    if let Some(opened_valve) = my_move.opened.clone() {
                        opened.insert(opened_valve.clone());
                        mutvalves.entry(opened_valve).and_modify(|v| v.open = true);
                    }
                    if let Some(opened_valve) = elephant_move.opened.clone() {
                        opened.insert(opened_valve.clone());
                        mutvalves.entry(opened_valve).and_modify(|v| v.open = true);
                    }
                    let next_minute = min(my_move.visitor.minutes, elephant_move.visitor.minutes);
                    let time = next_minute - minutes;
                    let increase = item.increase + my_move.increase + elephant_move.increase;
                    let mut visited = item.visited.clone();
                    visited.insert(item.me.valve.clone());
                    visited.insert(item.elephant.valve.clone());
                    // println!(
                    //     "Pushing for minute {}: pressure {}, increase {}, me {:?}, elephant {:?}, opened ({:?}, {:?}), visited: {:?}",
                    //     next_minute,
                    //     item.pressure + time * increase,
                    //     increase,
                    //     my_move.visitor,
                    //     elephant_move.visitor,
                    //     my_move.opened,
                    //     elephant_move.opened,
                    //     visited,
                    // );
                    queue.push_back(DualQueueItem {
                        pressure: item.pressure + time * increase,
                        increase,
                        valves: mutvalves,
                        me: my_move.visitor.clone(),
                        elephant: elephant_move.visitor.clone(),
                        visited,
                        opened,
                    });
                }
            }
        }
    }

    //         let elephant_valve = item.valves.get(&item.elephant.valve).unwrap();
    //         let mut elephant_path = item.elephant.came_from.clone();
    //         elephant_path.push(item.elephant.valve.clone());

    //         for tunnel in &my_valve.tunnels {
    //             // if let Some(last) = item.came_from.iter().last() {
    //             //     println!("dest = {}, came_from.last = {}", dest, last);
    //             // } else {
    //             //     println!("no last");
    //             // }
    //             if item.came_from.is_empty() || tunnel.dest != *item.came_from.last().unwrap() {
    //                 queue.push_back(QueueItem {
    //                     valve: tunnel.dest.to_string(),
    //                     minutes: item.minutes + tunnel.cost,
    //                     pressure: item.pressure + tunnel.cost * item.increase,
    //                     increase: item.increase,
    //                     came_from: path.clone(),
    //                     valves: item.valves.clone(),
    //                 });
    //             }
    //         }

    //     }
    //     // println!("queue is now {:?}", queue);
    // }
    max_pressure
}

#[allow(dead_code)]
fn print_graph(graph: &Map) {
    println!("digraph G {{");
    for (node, valve) in graph {
        println!("  {} [label=\"{}: {}\"]", node, node, valve.flow);
        for tunnel in &valve.tunnels {
            println!("  {} -> {} [label=\"{}\"]", node, tunnel.dest, tunnel.cost);
        }
    }
    println!("}}");
}

fn simplify_graph(graph: &Map) -> Map {
    // print_graph(graph);
    let mut simple: Map = graph.clone();
    for node in graph.keys() {
        if let Some(valve) = simple.get(node) {
            let mut mutsimple = simple.clone();
            // println!("Looking at {} {:?}", node, valve);
            if node != "AA" && valve.flow == 0 {
                // println!("removing");
                for tunnel in &(valve.tunnels) {
                    // println!(" looking at tunnel to {} cost {}", tunnel.dest, tunnel.cost);
                    let node_other = valve
                        .tunnels
                        .iter()
                        .filter(|vt| *vt.dest != tunnel.dest)
                        .cloned()
                        .collect_vec();
                    // println!("  tunnels to other than {} = {:?}", tunnel.dest, node_other);

                    if simple.contains_key(&tunnel.dest) {
                        let mut dest_other = simple
                            .get(&tunnel.dest)
                            .unwrap()
                            .tunnels
                            .iter()
                            .filter(|e| *e.dest != *node)
                            .cloned()
                            .collect_vec();

                        dest_other.extend(node_other.iter().map(|vt| Edge {
                            dest: vt.dest.clone(),
                            cost: tunnel.cost + vt.cost,
                        }));
                        // println!("  tunnels from {} now {:?}", tunnel.dest, dest_other);
                        let valve = simple.get(&tunnel.dest).unwrap();
                        let new_valve = Valve {
                            tunnels: dest_other.clone(),
                            ..*valve
                        };
                        mutsimple.insert(tunnel.dest.clone(), new_valve);

                        // println!(
                        //     "   check: simple[{}] = {:?}",
                        //     tunnel.dest,
                        //     mutsimple.get(&tunnel.dest).unwrap()
                        // );
                    }
                }
                mutsimple.remove(node);
                simple = mutsimple;
            }
        } else {
            println!("node {} already gone", node);
        }
    }
    // print_graph(&simple);
    simple
}

pub fn max_pressure(filename: &str) -> u32 {
    let valves = parse_file(filename);
    let simple = simplify_graph(&valves);
    path(&simple)
}

pub fn max_pressure_with_elephant(filename: &str) -> u32 {
    let valves = parse_file(filename);
    let simple = simplify_graph(&valves);
    dual_path(&simple)
}

#[cfg(test)]
mod tests {
    use super::{max_pressure, max_pressure_with_elephant};
    use test_case::test_case;

    #[test_case("../testinput/day16.txt", 1651; "on test input")]
    #[test_case("../input/day16.txt", 1873; "on real input")]
    fn part1(filename: &str, expected: u32) {
        assert_eq!(max_pressure(filename), expected);
    }
    #[test_case("../testinput/day16.txt", 1707; "on test input")]
    #[test_case("../input/day16.txt", 1873; "on real input")]
    fn part2(filename: &str, expected: u32) {
        assert_eq!(max_pressure_with_elephant(filename), expected);
    }
}
