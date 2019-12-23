use nom::character::complete::{digit1, newline};
use nom::{alt, do_parse, map_res, named, opt, recognize, separated_list, tag, tuple};

fn main() {
    let shuffles = match parse_input(include_str!("../../input/22.txt").trim()) {
        Ok(("", res)) => res,
        _ => panic!("failed to fully parse input"),
    };

    let p1 = part1(&shuffles);

    println!("part 1: {}", p1);
    println!("part 2: {}", "incomplete");
}

fn part1(shuffles: &[Shuffle]) -> usize {
    let mut stack: Vec<isize> = (0..=10006).collect();

    for shuffle in shuffles.iter() {
        match shuffle {
            Shuffle::Stack => deal_to_stack(&mut stack),
            Shuffle::Cut(pivot) => cut(&mut stack, *pivot),
            Shuffle::Increment(by) => deal_increment(&mut stack, *by),
        }
    }

    stack.iter().position(|&v| v == 2019).unwrap()
}

fn deal_to_stack(stack: &mut Vec<isize>) {
    stack.reverse();
}

fn cut(stack: &mut Vec<isize>, mut at: isize) {
    if at < 0 {
        at = stack.len() as isize + at;
    }

    stack[0..at as usize].reverse();
    stack[at as usize..].reverse();
    stack.reverse();
}

fn deal_increment(stack: &mut Vec<isize>, incr: usize) {
    let len = stack.len();
    for (i, v) in stack.clone().iter().enumerate() {
        stack[i * incr % len] = *v;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shuffle {
    Stack,
    Cut(isize),
    Increment(usize),
}

named!(
    parse_usize<&str, usize>,
    map_res!(
        digit1,
        std::str::FromStr::from_str
    )
);

named!(
    parse_isize<&str, isize>,
    map_res!(
        recognize!(tuple!(opt!(tag!("-")), digit1)),
        std::str::FromStr::from_str
    )
);

named!(
    parse_stack<&str, Shuffle>,
    do_parse!(tag!("deal into new stack") >> (Shuffle::Stack))
);

named!(
    parse_cut<&str, Shuffle>,
    do_parse!(tag!("cut ") >> val: parse_isize >> (Shuffle::Cut(val)))
);

named!(
    parse_increment<&str, Shuffle>,
    do_parse!(tag!("deal with increment ") >> val: parse_usize >> (Shuffle::Increment(val)))
);

named!(
    parse_shuffle<&str, Shuffle>,
    alt!(parse_stack | parse_cut | parse_increment)
);

named!(
    parse_input<&str, Vec<Shuffle>>,
    separated_list!(newline, parse_shuffle)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_to_stack_test() {
        let mut stack = vec![1, 2, 3, 4];

        deal_to_stack(&mut stack);

        assert_eq!(stack, vec![4, 3, 2, 1]);
    }

    #[test]
    fn cut_test() {
        let mut stack = vec![1, 2, 3, 4, 5, 6, 7];

        cut(&mut stack, 3);

        assert_eq!(stack, vec![4, 5, 6, 7, 1, 2, 3]);
    }

    #[test]
    fn cut_negative_test() {
        let mut stack = vec![1, 2, 3, 4, 5, 6, 7];

        cut(&mut stack, -3);

        assert_eq!(stack, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn increment_test() {
        let mut stack = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        deal_increment(&mut stack, 3);

        assert_eq!(stack, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
