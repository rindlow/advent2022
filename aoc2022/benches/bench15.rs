use brunch::{benches, Bench};
use std::time::Duration;

benches!(
    Bench::new("day15::cannot_contain")
        .run(|| aoc2022::day15::cannot_contain("../input/day15.txt", 2_000_000)),
    Bench::new("day15::tuning_frequency")
        .with_timeout(Duration::from_secs(60))
        .run(|| aoc2022::day15::tuning_frequency("../input/day15.txt", 4_000_000)),
);
