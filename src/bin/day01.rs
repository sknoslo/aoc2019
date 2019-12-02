use std::io;
use aoc2019::get_puzzle_input;

fn main() -> io::Result<()> {
    let input = get_puzzle_input()?;

    let masses = parse_input(&input);

    let p1 = part1(&masses);
    let p2 = part2(&masses);

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);

    Ok(())
}

fn parse_input(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn fuel_required(mass: isize) -> isize {
    mass / 3 - 2
}

fn part1(masses: &Vec<isize>) -> isize {
    masses.iter().map(|&mass| fuel_required(mass)).sum()
}

fn part2(masses: &Vec<isize>) -> isize {
    masses.iter().map(|&mass| {
        let mut cost = fuel_required(mass);
        let mut total_cost = 0;
        
        while cost > 0 {
            total_cost += cost;
            cost = fuel_required(cost);
        }

        total_cost
    }).sum()
}

