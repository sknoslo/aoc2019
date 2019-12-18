use aoc2019::computer::io::{IoDevice, QueuedIoDevice};
use aoc2019::computer::{parse_program, Computer};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

fn main() {
    let program = parse_program(include_str!("../../input/17.txt").trim());

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    let p2 = part2(&program);

    println!("part 2: {}", p2);
}

fn part1(program: &Vec<isize>) -> usize {
    let ascii = Rc::new(RefCell::new(AsciiDisplay::new()));
    let mut computer = Computer::new(Some(ascii.clone()), Some(ascii.clone()));
    computer.load(&program);

    computer.run();

    let ascii = ascii.borrow();

    let mut checksum = 0;

    for y in 1..(ascii.buffer.len() / ascii.width - 1) {
        for x in 1..ascii.width - 1 {
            let scaffold_count = [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .iter()
                .filter(|(x, y)| ascii.char_at(*x, *y) == '#')
                .count();

            if scaffold_count == 5 {
                checksum += x * y;
            }
        }
    }

    // println!("{}", ascii);

    checksum
}

fn part2(program: &Vec<isize>) -> isize {
    let ascii = Rc::new(RefCell::new(AsciiDisplay::new()));
    let mut computer = Computer::new(Some(ascii.clone()), Some(ascii.clone()));
    computer.load(&program);

    computer.run();

    let ascii = ascii.borrow();

    let _path = ascii.calculate_path();
    // println!("{:?}", path);
    //
    // result:
    // R12L8L4L4L8R6L6R12L8L4L4L8R6L6L8L4R12L6L4R12L8L4L4L8L4R12L6L4R12L8L4L4L8L4R12L6L4L8R6L6

    // TODO: solve with code? I came up with these just by looking at the output of the path.
    // some sort of compression algorithm could probably find these.
    let main_routine = "A,B,A,B,C,A,C,A,C,B\n";
    let func_a = "R,12,L,8,L,4,L,4\n";
    let func_b = "L,8,R,6,L,6\n";
    let func_c = "L,8,L,4,R,12,L,6,L,4\n";
    let vid_feed_res = "n\n";

    let mut program = program.clone();
    program[0] = 2; // override

    let input_queue = Rc::new(RefCell::new(QueuedIoDevice::new()));
    let output_queue = Rc::new(RefCell::new(QueuedIoDevice::new()));

    {
        let mut q = input_queue.borrow_mut();

        main_routine.chars().for_each(|c| q.write(c as u8 as isize));
        func_a.chars().for_each(|c| q.write(c as u8 as isize));
        func_b.chars().for_each(|c| q.write(c as u8 as isize));
        func_c.chars().for_each(|c| q.write(c as u8 as isize));
        vid_feed_res.chars().for_each(|c| q.write(c as u8 as isize));
    }

    let mut computer = Computer::new(Some(input_queue.clone()), Some(output_queue.clone()));

    computer.load(&program);

    computer.run();

    // skip over all the video feed junk
    let mut result = 0;
    while let Some(v) = computer.read_output() {
        result = v;
    }

    result
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

    fn char_at(&self, x: usize, y: usize) -> char {
        self.buffer[y * self.width + x]
    }

    fn calculate_path(&self) -> Vec<u8> {
        let w = self.width;
        let h = self.buffer.len() / w;

        let starting_point = self
            .buffer
            .iter()
            .position(|&c| c == '^' || c == '>' || c == 'v' || c == '<')
            .expect("robot position not found!");

        let mut cpos = (starting_point % w, starting_point / w);
        let mut cdir = self.buffer[starting_point];

        let mut path = Vec::new();

        let mut line_len = 0;

        loop {
            let switch_dir = match cdir {
                '^' => cpos.1 == 0 || self.char_at(cpos.0, cpos.1 - 1) != '#',
                '>' => cpos.0 == w - 1 || self.char_at(cpos.0 + 1, cpos.1) != '#',
                'v' => cpos.1 == h - 1 || self.char_at(cpos.0, cpos.1 + 1) != '#',
                '<' => cpos.0 == 0 || self.char_at(cpos.0 - 1, cpos.1) != '#',
                _ => unreachable!(),
            };

            if switch_dir {
                if line_len != 0 {
                    path.push(line_len);
                }

                line_len = 0;

                cdir = match cdir {
                    '^' => {
                        if cpos.0 != 0 && self.char_at(cpos.0 - 1, cpos.1) == '#' {
                            path.push('L' as u8);

                            '<'
                        } else if cpos.0 != w - 1 && self.char_at(cpos.0 + 1, cpos.1) == '#' {
                            path.push('R' as u8);

                            '>'
                        } else {
                            break;
                        }
                    }
                    '>' => {
                        if cpos.1 != 0 && self.char_at(cpos.0, cpos.1 - 1) == '#' {
                            path.push('L' as u8);

                            '^'
                        } else if cpos.1 != h - 1 && self.char_at(cpos.0, cpos.1 + 1) == '#' {
                            path.push('R' as u8);

                            'v'
                        } else {
                            break;
                        }
                    }
                    'v' => {
                        if cpos.0 != 0 && self.char_at(cpos.0 - 1, cpos.1) == '#' {
                            path.push('R' as u8);

                            '<'
                        } else if cpos.0 != w - 1 && self.char_at(cpos.0 + 1, cpos.1) == '#' {
                            path.push('L' as u8);

                            '>'
                        } else {
                            break;
                        }
                    }
                    '<' => {
                        if cpos.1 != 0 && self.char_at(cpos.0, cpos.1 - 1) == '#' {
                            path.push('R' as u8);

                            '^'
                        } else if cpos.1 != h - 1 && self.char_at(cpos.0, cpos.1 + 1) == '#' {
                            path.push('L' as u8);

                            'v'
                        } else {
                            break;
                        }
                    }
                    _ => unreachable!(),
                }
            } else {
                line_len += 1;

                cpos = match (cdir, cpos) {
                    ('^', (x, y)) => (x, y - 1),
                    ('>', (x, y)) => (x + 1, y),
                    ('v', (x, y)) => (x, y + 1),
                    ('<', (x, y)) => (x - 1, y),
                    _ => unreachable!(),
                };
            }
        }

        path
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
