use aoc2019::computer::io::{IoDevice, QueuedIoDevice};
use aoc2019::computer::{parse_program, Computer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let program = parse_program(include_str!("../../input/13.txt").trim());

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    let p2 = part2(&program);

    println!("part 2: {}", p2);
}

fn part1(program: &Vec<isize>) -> usize {
    let cabinet = Rc::new(RefCell::new(Cabinet::new()));
    let input = Rc::new(RefCell::new(QueuedIoDevice::new()));

    let mut comp = Computer::new(Some(input.clone()), Some(cabinet.clone()));
    comp.load(&program);
    comp.run();

    {
        let cabinet = cabinet.borrow();

        cabinet.display.values().filter(|&&t| t == 2).count()
    }
}

fn part2(program: &Vec<isize>) -> isize {
    let cabinet = Rc::new(RefCell::new(Cabinet::new()));

    let mut comp = Computer::new(Some(cabinet.clone()), Some(cabinet.clone()));
    let mut program = program.clone();
    program[0] = 2; // insert coins

    comp.load(&program);
    comp.run();

    {
        let cabinet = cabinet.borrow();

        cabinet.score
    }
}

#[derive(Debug)]
struct Cabinet {
    bp: usize,
    buffer: Vec<isize>,
    display: HashMap<(isize, isize), isize>,
    score: isize,
    // For the bot player to spy on the ball and paddle
    ballx: isize,
    paddlex: isize,
}

impl Cabinet {
    fn new() -> Self {
        Self {
            bp: 0,
            buffer: vec![0; 3],
            display: HashMap::new(),
            score: 0,
            ballx: 0,
            paddlex: 0,
        }
    }
}

impl IoDevice for Cabinet {
    fn write(&mut self, value: isize) {
        self.buffer[self.bp] = value;

        self.bp += 1;

        if self.bp == 3 {
            self.bp = 0;

            // flush buffer
            let x = self.buffer[0];
            let y = self.buffer[1];
            let tileid = self.buffer[2];
            if x == -1 && y == 0 {
                self.score = tileid; // a "tileid" at (-1, 0) is actually the score.
            } else {
                self.display.insert((x, y), tileid);
            }

            if tileid == 3 {
                self.paddlex = x;
            } else if tileid == 4 {
                self.ballx = x;
            }
        }
    }

    fn read(&mut self) -> Option<isize> {
        let dx = self.ballx - self.paddlex;

        Some(if dx == 0 { 0 } else { dx / dx.abs() })
    }
}
