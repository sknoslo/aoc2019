use std::io;
use aoc2019::get_puzzle_input;

fn main() -> io::Result<()> {
    let input = get_puzzle_input()?;

    let input = parse_input(&input.trim());

    let p1 = part1(&input);

    println!("part 1: {}", p1);

    Ok(())
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|v| {
        println!("{}", v);
        v.parse().unwrap()
    }).collect()
}

fn part1(intcodes: &Vec<usize>) -> usize {
    let mut pc = 0;

    loop {
        // do stuff
        //
        break;
    }

    intcodes[0]
}

