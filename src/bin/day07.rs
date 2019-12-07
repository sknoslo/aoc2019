use aoc2019::computer::{parse_program, Computer};
use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/07.txt").trim();

    let program = parse_program(&input);

    let p1 = part1(&program);

    println!("part 1: {}", p1);
}

fn part1(program: &Vec<isize>) -> isize {
    let mut computer = Computer::new();
    let mut max = 0;

    for permutation in (0..=4).into_iter().permutations(5) {
        let mut next_input = 0;

        for amp_input in permutation {
            computer.load(&program);
            computer.send(next_input);
            computer.send(amp_input);
            computer.run();

            next_input = computer.read_output();
        }

        if next_input > max {
            max = next_input;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1_test() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";

        let program = parse_program(&input);

        assert_eq!(part1(&program), 43210);
    }

    #[test]
    fn part1_example2_test() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";

        let program = parse_program(&input);

        assert_eq!(part1(&program), 54321);
    }
}
