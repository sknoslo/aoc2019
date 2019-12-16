fn main() {
    let sequence = parse_input(include_str!("../../input/16.txt").trim());

    let p1 = part1(&sequence);

    println!("part 1: {}", p1);

    let p2 = part2(&sequence);

    println!("part 2: {}", p2);
}

fn part1(seq: &Vec<isize>) -> String {
    get_seq_after_phases(seq, 100)
        .iter()
        .map(|digit| std::char::from_digit(*digit as u32, 10).unwrap())
        .take(8) // get just the first 8 digits
        .collect::<String>()
}

fn part2(seq: &Vec<isize>) -> String {
    // Observations:
    // * digit k is a function of digits k-n, so any calculations before k are irrelevant
    // * if you visualize the phases as a matrix, there is a pattern at the end,
    //       [-, -, -, a, -]
    //       [-, -, -, b, c]
    //       [-, -, -, -, -]
    //       [-, -, -, -, -] -> b = (a + c) % 10
    //
    //       and the last digit is always the same as the initial value. Using this, should be able
    //       to work backwards to create the sequence?

    let offset = seq[0..7].iter().fold(0, |acc, digit| acc * 10 + digit);

    let total_size = seq.len() * 10_000;
    let end_length = total_size - offset as usize;

    let seq = seq.iter().cloned().rev().cycle().take(end_length);

    let final_seq: Vec<_> = (0..100).fold(seq.to_owned(), |prev_seq| {
        prev_seq
            .scan(0, |prev, curr| {
                *prev = (curr + *prev) % 10;
                Some(*prev)
            })
            .collect()
    });

    final_seq
        .iter()
        .rev()
        .map(|d| std::char::from_digit(*d as u32, 10).unwrap())
        .take(8)
        .collect()
}

fn get_seq_after_phases(seq: &Vec<isize>, phases: usize) -> Vec<isize> {
    (0..phases).fold(seq.to_owned(), |last_seq, _| {
        (1..=last_seq.len())
            .map(|digit| {
                last_seq
                    .iter()
                    .zip(Pattern::new(digit))
                    .map(|(s, p)| s * p)
                    .sum::<isize>()
                    .abs()
                    % 10
            })
            .collect()
    })
}

struct Pattern {
    curr: usize,
    period: usize,
}

impl Pattern {
    fn new(period: usize) -> Self {
        Self { curr: 0, period }
    }
}

impl Iterator for Pattern {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        static BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

        self.curr += 1;

        Some(BASE_PATTERN[(self.curr / self.period) % 4])
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

#[cfg(test)]
mod pattern_tests {
    use super::*;

    #[test]
    fn period2() {
        let p = Pattern::new(2);

        assert_eq!(
            p.take(8).collect::<Vec<_>>(),
            vec![0, 1, 1, 0, 0, -1, -1, 0]
        );
    }

    #[test]
    fn period3() {
        let p = Pattern::new(3);

        assert_eq!(p.take(8).collect::<Vec<_>>(), vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_test() {
        let seq = parse_input("12345678");

        assert_eq!(get_seq_after_phases(&seq, 4), vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn part1_test1() {
        let seq = parse_input("80871224585914546619083218645595");

        assert_eq!(part1(&seq), "24176176");
    }

    #[test]
    fn part2_test1() {
        let seq = parse_input("03036732577212944063491565474664");
        assert_eq!(part2(&seq), "84462026");
    }
}
