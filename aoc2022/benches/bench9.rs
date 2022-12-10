use brunch::{benches, Bench};

benches!(
    Bench::new("day9::visited_nodes#2")
        .run(|| aoc2022::day9::visited_nodes("../input/day9.txt", 2)),
    Bench::new("day9::visited_nodes#10")
        .run(|| aoc2022::day9::visited_nodes("../input/day9.txt", 10))
);
