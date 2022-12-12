use brunch::{benches, Bench};

benches!(
    Bench::new("day11::monkey_business_20")
        .run(|| aoc2022::day11::monkey_business("../input/day11.txt", true)),
    Bench::new("day11::monkey_business_10k")
        .run(|| aoc2022::day11::monkey_business("../input/day11.txt", false)),
);
