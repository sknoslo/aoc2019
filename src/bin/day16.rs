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
    // TODO:
    // clearly brute force is going to take too long...
    // given that the input is repeated, the final sequence is probably repeated too. just get the
    // offset from part 1 and find the message in the repeated output.
    let seq = seq
        .iter()
        .cloned()
        .cycle()
        .take(seq.len() * 10_000)
        .collect();

    get_seq_after_phases(&seq, 100)
        .iter()
        .map(|digit| std::char::from_digit(*digit as u32, 10).unwrap())
        .take(8) // get just the first 8 digits
        .collect::<String>()
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
}
