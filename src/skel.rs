use std::fs::read_to_string;

pub fn impl1() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::impl1;

    #[test]
    fn part1() {
        assert_eq!(1, impl1());
    }
}
