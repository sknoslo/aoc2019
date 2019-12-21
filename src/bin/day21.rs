use aoc2019::computer::io::AsciiDevice;
use aoc2019::computer::{parse_program, Computer};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let program = parse_program(include_str!("../../input/21.txt").trim());

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    let p2 = part2(&program);

    println!("part 2: {}", p2);
}

fn part1(program: &Vec<isize>) -> isize {
    // Strategy:
    //   if there is a hole at A, B, or C and ground at D, jump
    let solution = "\
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";
    let input = Rc::new(RefCell::new(AsciiDevice::new()));
    let output = Rc::new(RefCell::new(AsciiDevice::new()));

    let mut comp = Computer::new(Some(input.clone()), Some(output.clone()));

    {
        let mut input = input.borrow_mut();

        input.write_ascii(&solution);
    }

    comp.load(&program);
    comp.run();

    {
        let mut output = output.borrow_mut();

        if let Some(v) = output.read_non_ascii() {
            return v;
        }

        println!("{}", output.get_ascii_image());
    }

    0
}

fn part2(program: &Vec<isize>) -> isize {
    // Strategy:
    //   if there is a hole at A, B, or C and ground at D and ground at E OR H, jump
    let solution = "\
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
AND E T
OR H T
AND T J
RUN
";
    let input = Rc::new(RefCell::new(AsciiDevice::new()));
    let output = Rc::new(RefCell::new(AsciiDevice::new()));

    let mut comp = Computer::new(Some(input.clone()), Some(output.clone()));

    {
        let mut input = input.borrow_mut();

        input.write_ascii(&solution);
    }

    comp.load(&program);
    comp.run();

    {
        let mut output = output.borrow_mut();

        if let Some(v) = output.read_non_ascii() {
            return v;
        }

        println!("{}", output.get_ascii_image());
    }

    0
}
