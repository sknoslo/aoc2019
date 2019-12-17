use aoc2019::computer::io::IoDevice;
use aoc2019::computer::{parse_program, Computer};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

fn main() {
    let program = parse_program(include_str!("../../input/17.txt").trim());

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    println!("part 2: {}", "incomplete");
}

fn part1(program: &Vec<isize>) -> usize {
    let ascii = Rc::new(RefCell::new(AsciiDisplay::new()));
    let mut computer = Computer::new(Some(ascii.clone()), Some(ascii.clone())); // TODO: make the computer not need an input...
    computer.load(&program);

    computer.run();

    let ascii = ascii.borrow();

    let mut checksum = 0;

    for y in 1..(ascii.buffer.len() / ascii.width - 1) {
        for x in 1..ascii.width - 1 {
            let scaffold_count = [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .filter(|(x, y)| ascii.get_char_at(*x, *y) == '#')
                .count();

            if scaffold_count == 5 {
                checksum += x * y;
            }
        }
    }

    println!("{}", ascii);

    checksum
}

#[derive(Debug)]
struct AsciiDisplay {
    width: usize,
    buffer: Vec<char>,
}

impl AsciiDisplay {
    fn new() -> Self {
        Self {
            width: 0,
            buffer: Vec::new(),
        }
    }

    fn get_char_at(&self, x: usize, y: usize) -> char {
        self.buffer[y * self.width + x]
    }
}

impl IoDevice for AsciiDisplay {
    fn read(&mut self) -> Option<isize> {
        None
    }

    fn write(&mut self, value: isize) {
        let value = (value as u8) as char;

        if value == '\n' {
            // set the width after reaching the end of the very first line.
            if self.width == 0 {
                self.width = self.buffer.len();
            }
        } else {
            self.buffer.push(value);
        }
    }
}

impl fmt::Display for AsciiDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        let mut y = 0;
        while i < self.buffer.len() - 1 {
            for x in 0..self.width {
                i = y * self.width + x;
                write!(f, "{}", self.buffer[i])?;
            }
            writeln!(f, "")?;
            y += 1;
        }

        Ok(())
    }
}
