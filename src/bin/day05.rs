use aoc2019::{computer::Computer, get_puzzle_input};
use std::io;

fn main() -> io::Result<()> {
    let input = get_puzzle_input()?;

    let input = parse_input(&input);

    let p1 = part1(&input);

    println!("part 1: {}", p1);

    Ok(())
}

fn parse_input(input: &str) -> Vec<isize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn part1(program: &Vec<isize>) -> isize {
    let mut computer = Computer::new();

    computer.load(program);

    computer.send(1);

    computer.run();

    *computer
        .receive()
        .last()
        .expect("no output received from the computer!")
}
