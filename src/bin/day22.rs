use nom::character::complete::{digit1, newline};
use nom::{alt, do_parse, map_res, named, opt, recognize, separated_list, tag, tuple};

const P1_SIZE: isize = 10007;

fn main() {
    let shuffles = match parse_input(include_str!("../../input/22.txt").trim()) {
        Ok(("", res)) => res,
        _ => panic!("failed to fully parse input"),
    };

    let p1 = part1(&shuffles);

    println!("part 1: {}", p1);
    println!("part 2: {}", "incomplete");
}

fn part1(shuffles: &[Shuffle]) -> isize {
    shuffles.iter().fold(2019, |i, &shuffle| match shuffle {
        Shuffle::Stack => deal_to_stack(i),
        Shuffle::Cut(pivot) => cut(i, pivot),
        Shuffle::Increment(by) => deal_increment(i, by),
    })
}

fn deal_to_stack(i: isize) -> isize {
    P1_SIZE - 1 - i
}

fn cut(i: isize, pivot: isize) -> isize {
    (i - pivot) % P1_SIZE
}

fn deal_increment(i: isize, by: isize) -> isize {
    (i * by) % P1_SIZE
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shuffle {
    Stack,
    Cut(isize),
    Increment(isize),
}

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
    do_parse!(tag!("deal with increment ") >> val: parse_isize >> (Shuffle::Increment(val)))
);

named!(
    parse_shuffle<&str, Shuffle>,
    alt!(parse_stack | parse_cut | parse_increment)
);

named!(
    parse_input<&str, Vec<Shuffle>>,
    separated_list!(newline, parse_shuffle)
);
