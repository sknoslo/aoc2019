use nom::character::complete::{alpha1, digit1};
use nom::{do_parse, map, map_res, named, separated_list, separated_pair, tag};
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = parse_input(include_str!("../../input/14.txt").trim());

    let p1 = part1(&input);

    println!("part 1: {}", p1);

    let p2 = part2(&input);

    println!("part 2: {}", p2);
}

fn part1(reactions: &HashMap<String, Reaction>) -> usize {
    let mut ore_used = 0;
    let mut to_visit = VecDeque::new();
    let mut left_overs: HashMap<String, usize> = HashMap::new();

    to_visit.push_front(("FUEL".to_string(), 1));

    while let Some((chem, needed)) = to_visit.pop_back() {
        let left_over = left_overs.entry(chem.clone()).or_insert(0);

        if needed <= *left_over {
            *left_over -= needed;
        } else {
            let needed = needed - *left_over;
            *left_over = 0;

            if let Some(reaction) = reactions.get(&chem) {
                let reaction_quantity = reaction.prod.0;
                let mut multiplier = needed / reaction_quantity;
                if needed % reaction_quantity > 0 {
                    multiplier += 1;
                }

                *left_over = multiplier * reaction_quantity - needed;

                reaction.chem.iter().for_each(|c| {
                    if c.1 == "ORE".to_string() {
                        ore_used += c.0 * multiplier;
                    } else {
                        to_visit.push_front((c.1.clone(), multiplier * c.0));
                    }
                });
            }
        }
    }

    ore_used
}

fn part2(_reactions: &HashMap<String, Reaction>) -> usize {
    // find the percentage of ORE that is "unused" each time 1 FUEL is created.
    // if it required 3 ORE to make 5 A, and 6 A to make 1 FUEL, 4 out of 10 A go wasted.
    // so 40% of the 6 ORE are wasted as well. is that enough info to calculate the answer?
    0
}

#[derive(Debug, Eq, PartialEq)]
struct Reaction {
    prod: (usize, String),
    chem: Vec<(usize, String)>,
}

impl Reaction {
    fn new(prod: (usize, String), chem: Vec<(usize, String)>) -> Self {
        Self { prod, chem }
    }
}

fn parse_input(input: &str) -> HashMap<String, Reaction> {
    input
        .lines()
        .map(|line| {
            let (_, re) = parse_reaction(line.as_bytes()).unwrap();
            (re.prod.1.clone(), re)
        })
        .collect()
}

fn num_from_str(input: &str) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(input, 10)
}

named!(
    chemical<String>,
    map!(map_res!(alpha1, std::str::from_utf8), String::from)
);

named!(
    quantity<usize>,
    map_res!(map_res!(digit1, std::str::from_utf8), num_from_str)
);

named!(
    quantity_and_chem<(usize, String)>,
    separated_pair!(quantity, tag!(" "), chemical)
);

named!(
    chemicals<Vec<(usize, String)>>,
    separated_list!(tag!(", "), quantity_and_chem)
);

named!(
    parse_reaction<Reaction>,
    do_parse!(
        chem: chemicals >> tag!(" => ") >> prod: quantity_and_chem >> (Reaction::new(prod, chem))
    )
);

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn chemical_test() {
        assert_eq!(
            chemical("ABCDE".as_bytes()),
            Ok(("".as_bytes(), "ABCDE".to_string()))
        );
    }

    #[test]
    fn quantity_test() {
        assert_eq!(quantity("245".as_bytes()), Ok(("".as_bytes(), 245)));
    }

    #[test]
    fn full_parser_test() {
        assert_eq!(
            parse_reaction("8 AVCAD, 7 TSVQX => 6 LMVCD".as_bytes()),
            Ok((
                "".as_bytes(),
                Reaction::new(
                    (6, "LMVCD".to_string()),
                    vec![(8, "AVCAD".to_string()), (7, "TSVQX".to_string())]
                )
            )),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = parse_input(
            "\
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
        );

        assert_eq!(part1(&input), 165);
    }

    #[test]
    fn par1_test2() {
        let input = parse_input(
            "\
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );

        assert_eq!(part1(&input), 13312);
    }
}
