use aoc2019::computer::Computer;

fn main() {
    let input = include_str!("../../input/05.txt").trim();

    let program = parse_input(&input);

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    let p2 = part2(&program);

    println!("part 2: {}", p2);
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

fn part2(program: &Vec<isize>) -> isize {
    let mut computer = Computer::new();

    computer.load(program);

    computer.send(5);

    computer.run();

    *computer
        .receive()
        .last()
        .expect("no output received from the computer!")
}
