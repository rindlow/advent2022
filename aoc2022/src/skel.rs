use std::fs::read_to_string;

pub fn impl1() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::impl1;
    use test_case::test_case;

    #[test_case("../testinput/dayX.txt", 0; "on test input")]
    // #[test_case("../input/dayX.txt", 0; "on real input")]
    fn part1(filename: &str, expected: u64) {
        assert_eq!(impl1(filename), expected);
    }
}
