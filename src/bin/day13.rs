use aoc2019::computer::io::{IoDevice, QueuedIoDevice};
use aoc2019::computer::{parse_program, Computer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let program = parse_program(include_str!("../../input/13.txt").trim());

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    println!("part 2: {}", "incomplete");
}

fn part1(program: &Vec<isize>) -> usize {
    let arcade_screen = Rc::new(ArcadeScreen::new());
    let input = Rc::new(QueuedIoDevice::new());

    let mut comp = Computer::new(Some(input.clone()), Some(arcade_screen.clone()));
    comp.load(&program);
    comp.run();

    let display = arcade_screen.display.borrow();

    display.values().filter(|&&t| t == 2).count()
}

#[derive(Debug)]
struct ArcadeScreen {
    buffer_ptr: RefCell<usize>,
    buffer: RefCell<Vec<isize>>,
    display: RefCell<HashMap<(isize, isize), isize>>,
}

impl ArcadeScreen {
    fn new() -> Self {
        Self {
            buffer_ptr: RefCell::new(0),
            buffer: RefCell::new(vec![0; 3]),
            display: RefCell::new(HashMap::new()),
        }
    }
}

impl IoDevice for ArcadeScreen {
    fn write(&self, value: isize) {
        let mut bp = self.buffer_ptr.borrow_mut();
        self.buffer.borrow_mut()[*bp] = value;

        *bp += 1;

        if *bp == 3 {
            *bp = 0;

            let buffer = self.buffer.borrow_mut();
            let mut display = self.display.borrow_mut();

            // flush buffer
            let x = buffer[0];
            let y = buffer[1];
            let tileid = buffer[2];
            display.insert((x, y), tileid);
        }
    }

    fn read(&self) -> Option<isize> {
        None
    }
}
