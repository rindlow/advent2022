use fxhash::FxHashSet as HashSet;
use itertools::Itertools;
use std::fs::read_to_string;

pub fn ximpl1(filename: &str) -> i32 {
    let numbers = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();
    let mut seq = numbers.clone();
    let nlen = i32::try_from(numbers.len()).unwrap();
    for n in &numbers {
        let i = seq.iter().position(|x| *x == *n).unwrap();
        println!("n = {}, i = {}", *n, i);
        seq.remove(i);
        let s = i32::try_from(i).unwrap();
        let q = (s + *n + nlen) % nlen;
        let u = if *n < 0 { (nlen + q - 1) % nlen } else { q };
        let index = usize::try_from(u).unwrap();
        println!("q = {}, index = {}", q, index);
        seq.insert(index, *n);
        println!(
            "{} moves between {} and {}:\n{:?}\n",
            *n % nlen,
            seq[(numbers.len() + index - 1) % numbers.len()],
            seq[(index + 1) % numbers.len()],
            seq
        );
    }
    let zero = seq.iter().position(|n| *n == 0).unwrap();
    (1..=3)
        .map(|t| seq[(t * 1000 + zero) % numbers.len()])
        .sum()
}

#[derive(Debug)]
struct Number {
    n: i32,
    i: usize,
}

fn check_indices(vec: &[Number]) {
    vec.iter()
        .sorted_by(|a, b| a.i.cmp(&b.i))
        .enumerate()
        .for_each(|(next, number)| assert_eq!(number.i, next));
}

pub fn impl1(filename: &str) -> i32 {
    let numbers = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();
    let mut mutnumbers = numbers
        .iter()
        .enumerate()
        .map(|(i, n)| Number { n: *n, i })
        .collect_vec();
    let nlen_usize = numbers.len();
    let nlen_i32 = i32::try_from(nlen_usize).unwrap();
    for i in 0..nlen_usize {
        let index_usize = mutnumbers[i].i;
        let index_i32 = i32::try_from(index_usize).unwrap();
        let n = mutnumbers[i].n;
        let new_index_i32 = if index_i32 + n > nlen_i32 {
            let mut x = index_i32 + n + 1;
            while x >= nlen_i32 {
                x -= nlen_i32;
            }
            x
        } else if index_i32 + n < 0 {
            let mut x = index_i32 + n - 1;
            while x < 0 {
                x += nlen_i32;
            }
            x
        } else {
            index_i32 + n
        };

        let new_index_usize = usize::try_from(new_index_i32).unwrap();
        // println!(
        //     "i: {}, n: {:?}, new_index: {}",
        //     i, mutnumbers[i], new_index_usize
        // );
        for number in &mut mutnumbers {
            if new_index_usize > index_usize
                && number.i > index_usize
                && number.i <= new_index_usize
            {
                number.i -= 1;
            } else if number.i < index_usize && number.i >= new_index_usize {
                number.i += 1;
            }
        }
        mutnumbers[i].i = new_index_usize;
        // println!("{:?}\n", mutnumbers);
        check_indices(&mutnumbers);
    }
    mutnumbers
        .iter()
        .sorted_by(|a, b| a.i.cmp(&b.i))
        .for_each(|n| println!("{:04}: {}", n.i, n.n));

    let zero = mutnumbers.iter().find(|n| n.n == 0).unwrap().i;
    let indices: HashSet<usize> = (1..=3).map(|t| (t * 1000 + zero) % numbers.len()).collect();
    mutnumbers
        .iter()
        .filter(|n| indices.contains(&n.i))
        .map(|n| n.n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::impl1;
    use test_case::test_case;

    #[test_case("../testinput/day20.txt", 3; "on test input")]
    #[test_case("../input/day20.txt", 0; "on real input")]
    fn part1(filename: &str, expected: i32) {
        assert_eq!(impl1(filename), expected);
    }
}
