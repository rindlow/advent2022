use brunch::{benches, Bench};
use std::time::Duration;

benches!(
    Bench::new("day14::sand_before_abyss")
        .run(|| aoc2022::day14::sand_before_abyss("../input/day14.txt")),
    Bench::new("day14::sand_to_rest")
        .with_timeout(Duration::from_secs(20))
        .run(|| aoc2022::day14::sand_to_rest("../input/day14.txt")),
);
