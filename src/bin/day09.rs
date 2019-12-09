use aoc2019::computer::{parse_program, Computer};

fn main() {
    let input = include_str!("../../input/09.txt").trim();

    let program = parse_program(&input);

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    let p2 = part2(&program);

    println!("part 2: {}", p2);
}

fn part1(program: &Vec<isize>) -> isize {
    let mut computer = Computer::new();

    computer.load(&program);
    computer.send(1); // initialize test mode
    computer.run();

    computer.read_output().expect("No output!")
}

fn part2(program: &Vec<isize>) -> isize {
    let mut computer = Computer::new();

    computer.load(&program);
    computer.send(2); // initialize BO0ST mode
    computer.run();

    computer.read_output().expect("No output!")
}
