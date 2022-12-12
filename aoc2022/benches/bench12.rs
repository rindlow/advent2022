use brunch::{benches, Bench};

benches!(
    Bench::new("day12::shortest_path").run(|| aoc2022::day12::shortest_path("../input/day12.txt")),
    Bench::new("day12::shortest_path_from_any_a")
        .run(|| aoc2022::day12::shortest_path_from_any_a("../input/day12.txt")),
);
