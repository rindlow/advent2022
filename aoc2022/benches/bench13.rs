use brunch::{benches, Bench};

benches!(
    Bench::new("day13::sum_indices").run(|| aoc2022::day13::sum_indices("../input/day13.txt")),
    Bench::new("day13::decoder_key").run(|| aoc2022::day13::decoder_key("../input/day13.txt")),
);
