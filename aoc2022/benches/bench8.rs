use brunch::{benches, Bench};

benches!(
    Bench::new("day8::visible").run(|| aoc2022::day8::visible("../input/day8.txt")),
    Bench::new("day8::highest_score").run(|| aoc2022::day8::highest_score("../input/day8.txt")),
);
