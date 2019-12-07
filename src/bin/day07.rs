use aoc2019::computer::{parse_program, Computer, ExecutionResult};
use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/07.txt").trim();

    let program = parse_program(&input);

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    let p2 = part2(&program);

    println!("part 2: {}", p2);
}

fn part1(program: &Vec<isize>) -> isize {
    let mut computer = Computer::new();
    let mut max = 0;

    for permutation in (0..=4).into_iter().permutations(5) {
        let mut next_input = 0;

        for amp_input in permutation {
            computer.load(&program);
            computer.send(amp_input);
            computer.send(next_input);
            computer.run();

            next_input = computer.read_output().unwrap(); // TODO: do better
        }

        if next_input > max {
            max = next_input;
        }
    }

    max
}

fn part2(program: &Vec<isize>) -> isize {
    // TODO :do I need to pause on every output?
    // day 5 and 2 will need to be altered.

    let mut amps = vec![
        Computer::new(),
        Computer::new(),
        Computer::new(),
        Computer::new(),
        Computer::new(),
    ];

    let mut max = 0;

    for permutation in (5..=9).into_iter().permutations(5) {
        let mut next_input = 0;

        for (i, amp) in amps.iter_mut().enumerate() {
            amp.load(&program);
            amp.send(permutation[i]);
        }

        'cycle: loop {
            for (i, amp) in amps.iter_mut().enumerate() {
                amp.send(next_input);

                match amp.run() {
                    ExecutionResult::Halted => {
                        // this is weird? Assumes each amp halts in order, I think?
                        next_input = amp.read_output().expect("Uh, oh. Halted with no output!");

                        if i == 4 {
                            if next_input > max {
                                max = next_input;
                            }

                            break 'cycle;
                        }
                    }
                    ExecutionResult::Paused => {
                        next_input = amp.read_output().expect("Uh, oh. Paused with no output!");
                    }
                }
            }
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

    #[test]
    fn part2_example1_test() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";

        let program = parse_program(&input);

        assert_eq!(part2(&program), 139629729);
    }

    #[test]
    fn part2_example2_test() {
        let input =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";

        let program = parse_program(&input);

        assert_eq!(part2(&program), 18216);
    }
}
