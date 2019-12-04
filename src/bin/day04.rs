use aoc2019::get_puzzle_input;
use std::io;

fn main() -> io::Result<()> {
    let input = get_puzzle_input()?;

    let (lower, upper) = parse_input(&input);

    let p1 = part1(&mut lower.clone(), &mut upper.clone());

    println!("part 1: {}", p1);

    let p2 = part2(&mut lower.clone(), &mut upper.clone());

    println!("part 2: {}", p2);

    Ok(())
}

fn part1(mut lower: &mut Vec<u8>, upper: &mut Vec<u8>) -> usize {
    groom_lower(lower);
    groom_upper(upper);

    let mut candidates = 0;

    while less_than_or_equal(&lower, &upper) {
        if is_candidate(&lower) {
            candidates += 1;
        }

        increment(&mut lower);
        groom_lower(&mut lower);
    }

    candidates
}

fn part2(mut lower: &mut Vec<u8>, upper: &mut Vec<u8>) -> usize {
    groom_lower(lower);
    groom_upper(upper);

    let mut candidates = 0;

    while less_than_or_equal(&lower, &upper) {
        if is_candidate2(&lower) {
            candidates += 1;
        }

        increment(&mut lower);
        groom_lower(&mut lower);
    }

    candidates
}

fn groom_lower(lower: &mut Vec<u8>) {
    for pos in 1..lower.len() {
        if lower[pos - 1] > lower[pos] {
            lower[pos] = lower[pos - 1]
        }
    }
}

fn groom_upper(upper: &mut Vec<u8>) {
    for pos in 1..upper.len() {
        if upper[pos - 1] > upper[pos] {
            upper[pos - 1] -= 1;

            for other in pos..upper.len() {
                upper[other] = 9;
            }
            break;
        }
    }
}

fn less_than_or_equal(lower: &Vec<u8>, upper: &Vec<u8>) -> bool {
    for pos in 0..lower.len() {
        if lower[pos] < upper[pos] {
            return true;
        } else if lower[pos] > upper[pos] {
            return false;
        }
    }

    true
}

fn increment(password: &mut Vec<u8>) {
    for pos in (0..password.len()).rev() {
        if password[pos] < 9 {
            password[pos] += 1;
            break;
        } else if pos == 0 {
            // this doesn't handle the case when the first digit needs to be incremented, but this
            // problem should never run into that case.
            panic!("whoopsies")
        }

        password[pos] = 0;
    }
}

/**
 * A possible password is one that contains only increasing (or same) digits from left to right,
 * with at least one group of matching digits.
 */
fn is_candidate(password: &Vec<u8>) -> bool {
    let mut last = 0;
    let mut has_double = false;

    for &digit in password {
        if digit < last {
            return false;
        }

        if digit == last {
            has_double = true;
        }

        last = digit;
    }

    has_double
}

/**
 * A possible password is one that contains only increasing (or same) digits from left to right,
 * with at least one group of exactly 2 matching digits.
 */
fn is_candidate2(password: &Vec<u8>) -> bool {
    let mut last = 0;
    let mut matches = 0;
    let mut has_double = false;

    for &digit in password {
        if digit < last {
            return false;
        }

        if digit != last {
            if matches == 1 {
                has_double = true;
            }

            matches = 0;
        } else {
            matches += 1;
        }

        last = digit;
    }

    has_double = has_double || matches == 1; // in case the last two were the double.

    has_double
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<u8>) {
    let parts: Vec<_> = input.split("-").collect();

    let lower = parts.get(0).expect("input is garbage");
    let upper = parts.get(1).expect("input is garbage");

    let lower = lower
        .chars()
        .map(|c| c.to_digit(10).expect("didn't find a digit") as u8)
        .collect();
    let upper = upper
        .chars()
        .map(|c| c.to_digit(10).expect("didn't find a digit") as u8)
        .collect();

    (lower, upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groom_lower_test() {
        let mut v = vec![5, 4, 6, 2, 8, 9, 1];
        groom_lower(&mut v);
        assert_eq!(v, vec![5, 5, 6, 6, 8, 9, 9]);
    }

    #[test]
    fn groom_upper_test() {
        let mut v = vec![5, 4, 6, 2, 8, 9, 1];
        groom_upper(&mut v);
        assert_eq!(v, vec![4, 9, 9, 9, 9, 9, 9]);
    }

    #[test]
    fn less_than_test() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![3, 1, 9];
        assert_eq!(less_than_or_equal(&v1, &v2), true);
    }

    #[test]
    fn not_less_than_test() {
        let v1 = vec![3, 2, 0];
        let v2 = vec![3, 1, 9];
        assert_eq!(less_than_or_equal(&v1, &v2), false);
    }

    #[test]
    fn equal_less_than_test() {
        let v1 = vec![3, 2, 0];
        let v2 = vec![3, 2, 0];
        assert_eq!(less_than_or_equal(&v1, &v2), true);
    }

    #[test]
    fn increment_test() {
        let mut v1 = vec![6, 9, 9];

        increment(&mut v1);

        assert_eq!(v1, vec![7, 0, 0]);
    }

    #[test]
    fn is_candidate2_simple_test() {
        let v = vec![1, 2, 3, 3, 5];

        assert_eq!(is_candidate2(&v), true);
    }

    #[test]
    fn is_candidate2_negative_test() {
        let v = vec![1, 2, 3, 4, 5];

        assert_eq!(is_candidate2(&v), false);
    }

    #[test]
    fn is_candidate2_complex_test() {
        let v = vec![1, 1, 1, 5, 5];

        assert_eq!(is_candidate2(&v), true);
    }
}
