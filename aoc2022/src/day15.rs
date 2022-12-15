use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use std::{
    cmp::{max, min},
    fs::read_to_string,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Report {
    sensor: Pos,
    beacon: Pos,
    range: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    first: i64,
    last: i64,
}

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
            let sensor = Pos {
                x: get_num(split[2]),
                y: get_num(split[3]),
            };
            let beacon = Pos {
                x: get_num(split[8]),
                y: get_num(split[9]),
            };
            let range = (sensor.y - beacon.y).abs() + (sensor.x - beacon.x).abs();
            Report {
                sensor,
                beacon,
                range,
            }
        })
        .collect_vec()
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
    merged
}

fn row_in_range(row_no: i64, report: &Report) -> bool {
    (report.sensor.y - row_no).abs()
        < (report.sensor.y - report.beacon.y).abs() + (report.sensor.x - report.beacon.x).abs()
}

pub fn cannot_contain(filename: &str, row_no: i64) -> i64 {
    let reports = parse_file(filename);
    let mut ranges = Vec::<Range>::new();
    let mut occupied: HashSet<i64> = HashSet::default();
    for report in &reports {
        if row_in_range(row_no, report) {
            let dx = report.range - (report.sensor.y - row_no).abs();
            ranges.push(Range {
                first: report.sensor.x - dx,
                last: report.sensor.x + dx,
            });
        }
        if report.sensor.y == row_no {
            occupied.insert(report.sensor.x);
        }
        if report.beacon.y == row_no {
            occupied.insert(report.beacon.x);
        }
    }

    let mut x = 0;
    for range in merge_ranges(&ranges) {
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
    let mut rows = Vec::<Range>::new();
    for report in &reports {
        rows.push(Range {
            first: max(0, report.sensor.y - report.range),
            last: min(area, report.sensor.y + report.range),
        });
    }
    for row_range in merge_ranges(&rows) {
        for row_no in row_range.first..=row_range.last {
            let mut ranges = Vec::<Range>::new();
            for report in &reports {
                if row_in_range(row_no, report) {
                    let dx = report.range - (report.sensor.y - row_no).abs();
                    ranges.push(Range {
                        first: report.sensor.x - dx,
                        last: report.sensor.x + dx,
                    });
                }
            }
            for range in merge_ranges(&ranges) {
                if range.first > 0 && range.first <= area {
                    return 4_000_000 * (range.first - 1) + row_no;
                }
                if range.last >= 0 && range.last < area {
                    return 4_000_000 * (range.last + 1) + row_no;
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
