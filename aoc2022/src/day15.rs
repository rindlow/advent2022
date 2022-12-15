use fxhash::{FxHashMap as HashMap, FxHashSet as HashSet};
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Report {
    sensor: Pos,
    beacon: Pos,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    first: i64,
    last: i64,
}

type Map = HashMap<i64, Vec<Range>>;

fn get_num(s: &str) -> i64 {
    let mut sign: i64 = 1;
    s.chars().fold(0, |acc, c| {
        if c == '-' {
            sign = -1;
        }
        if c.is_numeric() {
            10 * acc + i64::try_from(c.to_digit(10).unwrap()).unwrap()
        } else {
            acc
        }
    }) * sign
}

fn parse_file(filename: &str) -> Vec<Report> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let split = line.split(' ').collect_vec();
            Report {
                sensor: Pos {
                    x: get_num(split[2]),
                    y: get_num(split[3]),
                },
                beacon: Pos {
                    x: get_num(split[8]),
                    y: get_num(split[9]),
                },
            }
        })
        .collect_vec()
}

fn make_map(reports: &Vec<Report>) -> HashMap<i64, Vec<Range>> {
    let mut map: Map = HashMap::default();
    for report in reports {
        let dist =
            (report.sensor.y - report.beacon.y).abs() + (report.sensor.x - report.beacon.x).abs();
        for y in report.sensor.y - dist..=report.sensor.y + dist {
            let x = dist - (report.sensor.y - y).abs();
            let range = Range {
                first: report.sensor.x - x,
                last: report.sensor.x + x,
            };
            map.entry(y)
                .and_modify(|row| row.push(range.clone()))
                .or_insert_with(|| vec![range]);
        }
    }
    map
}

fn overlap_or_adjacent(a: &Range, b: &Range) -> bool {
    a.first <= b.first && a.last >= b.last
        || a.first >= b.first && a.last <= b.last
        || a.first >= b.first && a.first <= b.last
        || a.last >= b.first && a.last <= b.last
        || a.last == b.first - 1
        || b.last == a.first - 1
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut merged = Vec::<Range>::new();
    for range in ranges.iter().sorted() {
        if let Some(last) = merged.last_mut() {
            // println!(
            //     "overlap({:?}, {:?}) => {}",
            //     range,
            //     last,
            //     overlap_or_adjacent(range, last)
            // );
            if overlap_or_adjacent(range, last) {
                if range.last > last.last {
                    last.last = range.last;
                }
            } else {
                merged.push(range.clone());
            }
        } else {
            merged.push(range.clone());
        }
    }
    // println!("merge({:?}) => {:?}", ranges, merged);
    merged
}

pub fn cannot_contain(filename: &str, row_no: i64) -> i64 {
    let reports = parse_file(filename);
    let map = make_map(&reports);

    let mut occupied: HashSet<i64> = HashSet::default();
    for report in &reports {
        if report.sensor.y == row_no {
            occupied.insert(report.sensor.x);
        }
        if report.beacon.y == row_no {
            occupied.insert(report.beacon.x);
        }
    }
    let mut x = 0;
    let ranges = merge_ranges(map.get(&row_no).unwrap());
    // println!("{:?} {:?}", ranges, occupied);
    for range in ranges {
        x += range.last - range.first + 1;
        for o in &occupied {
            if *o >= range.first && *o <= range.last {
                x -= 1;
            }
        }
    }
    x
}

pub fn tuning_frequency(filename: &str, area: i64) -> i64 {
    let reports = parse_file(filename);
    let map = make_map(&reports);
    for row in map.keys() {
        if *row >= 0 && *row <= area {
            let ranges = map.get(row).unwrap();
            // println!("{}: {:?}", *row, ranges);
            for range in merge_ranges(ranges) {
                // println!("{}: {:?}", *row, range);
                if range.first > 0 && range.first <= area {
                    return 4_000_000 * (range.first - 1) + *row;
                }
                if range.last >= 0 && range.last < area {
                    return 4_000_000 * (range.last + 1) + *row;
                }
            }
        }
    }
    panic!("no valid solution found");
}

#[cfg(test)]
mod tests {
    use super::{cannot_contain, tuning_frequency};
    use test_case::test_case;

    #[test_case("../testinput/day15.txt", 10, 26; "on test input")]
    #[test_case("../input/day15.txt", 2_000_000, 4_985_193; "on real input")]
    fn part1(filename: &str, row_no: i64, expected: i64) {
        assert_eq!(cannot_contain(filename, row_no), expected);
    }
    #[test_case("../testinput/day15.txt", 20, 56_000_011; "on test input")]
    #[test_case("../input/day15.txt", 4_000_000, 11_583_882_601_918; "on real input")]
    fn part2(filename: &str, area: i64, expected: i64) {
        assert_eq!(tuning_frequency(filename, area), expected);
    }
}
