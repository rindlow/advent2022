use brunch::{benches, Bench};

benches!(
    Bench::new("day14::sand_before_abyss")
        .run(|| aoc2022::day14::sand_before_abyss("../input/day14.txt")),
    Bench::new("day14::sand_to_rest").run(|| aoc2022::day14::sand_to_rest("../input/day14.txt")),
);
