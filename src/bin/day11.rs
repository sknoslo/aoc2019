use aoc2019::computer::io::QueuedIoDevice;
use aoc2019::computer::{parse_program, Computer, ExecutionResult};
use std::collections::HashMap;

fn main() {
    let program = parse_program(include_str!("../../input/11.txt").trim());

    let p1 = part1(&program);

    println!("part 1: {}", p1);

    let p2 = part2(&program);

    println!("part 2:");
    for line in p2 {
        println!("{}", line);
    }
}

fn part1(program: &Vec<isize>) -> usize {
    let mut bot = PaintBot::new();
    bot.computer.load(&program);

    let mut paint_surface = HashMap::new();

    bot.run_paint_seq(&mut paint_surface);

    paint_surface.keys().len()
}

fn part2(program: &Vec<isize>) -> Vec<String> {
    let mut bot = PaintBot::new();
    bot.computer.load(&program);

    let mut paint_surface = HashMap::new();
    paint_surface.insert(bot.pos, Color::White);

    bot.run_paint_seq(&mut paint_surface);

    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;

    for (x, y) in paint_surface.keys().cloned() {
        if x > maxx {
            maxx = x
        }
        if x < minx {
            minx = x
        }
        if y > maxy {
            maxy = y
        }
        if y < miny {
            miny = y
        }
    }

    let mut lines = vec![];
    for y in miny..=maxy {
        let mut line = vec!['.'; (maxx - minx + 1) as usize];

        for (i, x) in (minx..=maxx).rev().enumerate() {
            if let Some(color) = paint_surface.get(&(x, y)) {
                line[i] = match color {
                    Color::Black => '.',
                    Color::White => '#',
                }
            }
        }

        lines.push(line.into_iter().collect());
    }

    lines
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    U,
    R,
    D,
    L,
}

impl Dir {
    fn rotate_right(self) -> Self {
        match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        }
    }

    fn rotate_left(self) -> Self {
        match self {
            Self::U => Self::L,
            Self::R => Self::U,
            Self::D => Self::R,
            Self::L => Self::D,
        }
    }
}

#[derive(Debug)]
struct PaintBot {
    facing: Dir,
    pos: (isize, isize),
    computer: Computer<QueuedIoDevice, QueuedIoDevice>,
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Black = 0,
    White,
}

impl PaintBot {
    fn new() -> Self {
        PaintBot {
            facing: Dir::U,
            pos: (0, 0),
            computer: Computer::with_queue_io(),
        }
    }

    fn run_paint_seq(&mut self, mut surface: &mut HashMap<(isize, isize), Color>) {
        loop {
            match self.computer.run() {
                ExecutionResult::Halted => break,
                ExecutionResult::Paused => {
                    // on pause, should execute all of the paint instructions and then provide an
                    // input of the current color.
                    self.paint_and_move(&mut surface);

                    let current_color = surface.get(&self.pos).unwrap_or(&Color::Black);
                    self.computer.send(*current_color as isize);
                }
            }
        }

        self.paint_and_move(&mut surface); // finish any remaining instructions?
    }

    // TODO: does this even need to return anything? Just making assumptions.
    fn paint_and_move(&mut self, surface: &mut HashMap<(isize, isize), Color>) {
        while let Some(color) = self.computer.read_output() {
            surface.insert(
                self.pos,
                match color {
                    0 => Color::Black,
                    1 => Color::White,
                    _ => panic!("can't paint that color!"),
                },
            );

            if let Some(dir) = self.computer.read_output() {
                match dir {
                    0 => self.facing = self.facing.rotate_left(),
                    1 => self.facing = self.facing.rotate_right(),
                    _ => panic!("can't turn that direction!"),
                }
            }

            self.pos = match (self.pos, self.facing) {
                ((x, y), Dir::U) => (x, y - 1),
                ((x, y), Dir::R) => (x - 1, y),
                ((x, y), Dir::D) => (x, y + 1),
                ((x, y), Dir::L) => (x + 1, y),
            }
        }
    }
}
