use nom::character::complete::{digit1, newline};
use nom::{alt, do_parse, map_res, named, opt, recognize, separated_list, tag, tuple};

fn main() {
    let shuffles = match parse_input(include_str!("../../input/22.txt").trim()) {
        Ok(("", res)) => res,
        _ => panic!("failed to fully parse input"),
    };

    let p1 = part1(&shuffles);

    println!("part 1: {}", p1);

    let p2 = part2(&shuffles);
    println!("part 2: {}", p2);
}

fn part1(shuffles: &[Shuffle]) -> i128 {
    let deck_size = 10007;

    shuffles.iter().fold(2019, |i, &shuffle| match shuffle {
        Shuffle::Stack => deal_to_stack(i, deck_size),
        Shuffle::Cut(pivot) => cut(i, pivot, deck_size),
        Shuffle::Increment(by) => deal_increment(i, by, deck_size),
    })
}

fn part2(shuffles: &[Shuffle]) -> i128 {
    let deck_size = 119315717514047;
    let iterations = 101741582076661;

    // Part 1 finds the position of a specific card after a single iteration.
    // Part 2 requires us to find the card at a specific position, so we need to inverse the
    // equations. And since it needs to be applied a kazillion times, we'll reduce the sequence
    // down to a single equation of the form ax + b
    let (a, b) = shuffles
        .iter()
        .rev()
        .fold((1, 0), |(a, b), &shuffle| match shuffle {
            Shuffle::Stack => inv_deal_to_stack(a, b),
            Shuffle::Cut(pivot) => inv_cut(a, b, pivot),
            Shuffle::Increment(by) => inv_deal_increment(a, b, by, deck_size),
        });

    // would have never come up with this equation on my own... Shout out to reddit, again.
    // to apply the equation above, n times, the final equation becomes:
    let a_n = mod_pow(a, iterations, deck_size);
    let temp1 = 2020 * a_n % deck_size;
    let temp2 = (a_n - 1) * mod_pow(a - 1, deck_size - 2, deck_size) % deck_size;
    let temp3 = b * temp2 % deck_size;

    (temp1 + temp3) % deck_size
}

fn deal_to_stack(i: i128, deck_size: i128) -> i128 {
    deck_size - (i + 1)
}

fn inv_deal_to_stack(a: i128, b: i128) -> (i128, i128) {
    (-a, -(b + 1))
}

fn cut(i: i128, pivot: i128, deck_size: i128) -> i128 {
    (i - pivot) % deck_size
}

fn inv_cut(a: i128, b: i128, pivot: i128) -> (i128, i128) {
    (a, (b + pivot))
}

fn deal_increment(i: i128, by: i128, deck_size: i128) -> i128 {
    (i * by) % deck_size
}

// would have never figured out how to inverse this one... Shout out to reddit.
fn inv_deal_increment(a: i128, b: i128, by: i128, deck_size: i128) -> (i128, i128) {
    // using modular exponentiation to get the inverse modulo only works when by and deck_size are
    // co-prime.
    let by = mod_pow(by, deck_size - 2, deck_size);

    (a * by % deck_size, b * by % deck_size)
}

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;

    base = base % modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }

        exp = exp >> 1;
        base = base * base % modulus;
    }

    result
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shuffle {
    Stack,
    Cut(i128),
    Increment(i128),
}

named!(
    parse_i128<&str, i128>,
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
    do_parse!(tag!("cut ") >> val: parse_i128 >> (Shuffle::Cut(val)))
);

named!(
    parse_increment<&str, Shuffle>,
    do_parse!(tag!("deal with increment ") >> val: parse_i128 >> (Shuffle::Increment(val)))
);

named!(
    parse_shuffle<&str, Shuffle>,
    alt!(parse_stack | parse_cut | parse_increment)
);

named!(
    parse_input<&str, Vec<Shuffle>>,
    separated_list!(newline, parse_shuffle)
);
