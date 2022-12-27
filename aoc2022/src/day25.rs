use std::fs::read_to_string;

fn snafu_to_int(snafu: &str) -> i64 {
    let mut int = 0;
    for c in snafu.chars() {
        int *= 5;
        int += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("illegal char '{}'", c),
        }
    }
    int
}

fn int_to_snafu(i: i64) -> String {
    let mut int = i;
    let mut chars = Vec::<char>::new();
    while int > 0 {
        let m = int % 5;
        chars.push(match m {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                int += 5;
                '='
            }
            4 => {
                int += 5;
                '-'
            }
            _ => panic!("mod error"),
        });
        int /= 5;
    }
    chars.iter().rev().collect()
}

pub fn sum_snafu(filename: &str) -> String {
    int_to_snafu(
        read_to_string(filename)
            .unwrap()
            .lines()
            .map(snafu_to_int)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::sum_snafu;
    use test_case::test_case;

    #[test_case("../testinput/day25.txt", "2=-1=0"; "on test input")]
    #[test_case("../input/day25.txt", "2=20---01==222=0=0-2"; "on real input")]
    fn part1(filename: &str, expected: &str) {
        assert_eq!(sum_snafu(filename), expected);
    }
}
