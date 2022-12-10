use brunch::{benches, Bench};

benches!(
    Bench::new("day10::signal_strength")
        .run(|| aoc2022::day10::signal_strength("../input/day10.txt")),
    Bench::new("day10::crt").run(|| aoc2022::day10::crt("../input/day10.txt")),
);
