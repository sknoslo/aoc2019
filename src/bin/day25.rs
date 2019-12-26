use aoc2019::computer::io::AsciiDevice;
use aoc2019::computer::{parse_program, Computer, ExecutionResult};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

fn main() {
    let program = parse_program(include_str!("../../input/25.txt").trim());

    let p1 = part1(&program);

    println!("part 1: {}", p1);
}

fn part1(program: &Vec<isize>) -> String {
    let input = Rc::new(RefCell::new(AsciiDevice::new()));
    let output = Rc::new(RefCell::new(AsciiDevice::new()));

    let mut comp = Computer::new(Some(input.clone()), Some(output.clone()));

    comp.load(&program);

    loop {
        match comp.run() {
            ExecutionResult::Halted => break,
            _ => {}
        }

        {
            let mut output = output.borrow_mut();
            println!("{}", output.get_ascii_image());
        }

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        {
            let mut input = input.borrow_mut();
            input.write_ascii(&buffer);
        }
    }

    let mut output = output.borrow_mut();
    output.get_ascii_image()
}
