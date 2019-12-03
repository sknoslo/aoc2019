use aoc2019::{computer::Computer, get_puzzle_input};
use std::io;

fn main() -> io::Result<()> {
    let input = get_puzzle_input()?;

    let input = parse_input(&input);

    let p1 = part1(&input);

    println!("part 1: {}", p1);

    let p2 = part2(&input);
    println!("part 2: {}", p2);

    Ok(())
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

fn part1(program: &Vec<usize>) -> usize {
    let mut computer = Computer::new();

    computer.load(program);

    computer.write(1, 12);
    computer.write(2, 2);

    computer.run();

    computer.read(0)
}

fn part2(program: &Vec<usize>) -> usize {
    let target = 19690720;

    let mut computer = Computer::new();

    let mut noun = 12;
    let mut verb = 2;

    // iterate noun until the output is large
    loop {
        computer.load(program);

        computer.write(1, noun);
        computer.write(2, verb);

        computer.run();

        if computer.read(0) < target {
            noun = noun + 1;
        } else {
            noun = noun - 1;
            break;
        }
    }

    // iterate verb until the output is just right
    loop {
        computer.load(program);

        computer.write(1, noun);
        computer.write(2, verb);

        computer.run();

        if computer.read(0) == target {
            break;
        } else {
            verb = verb + 1;
        }
    }

    100 * noun + verb
}