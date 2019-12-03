use aoc2019::get_puzzle_input;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let input = get_puzzle_input()?;

    let input = parse_input(&input);

    let p1 = part1(&input[0], &input[1]);

    println!("part 1: {}", p1);

    Ok(())
}

fn part1(wire_a_inst: &Vec<Instruction>, wire_b_inst: &Vec<Instruction>) -> isize {
    let origin = Point::new(0, 0);
    let mut current = origin;

    let mut wire_a = HashSet::new();
    let mut wire_b = HashSet::new();

    for inst in wire_a_inst {
        current = current.go(inst);
        wire_a.insert(current);
    }

    current = origin;

    for inst in wire_b_inst {
        current = current.go(inst);
        wire_b.insert(current);
    }

    let _intersection: Vec<_> = wire_a.intersection(&wire_b).collect();

    // find smallest dist

    1
}

fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|v| v.parse::<Instruction>().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err(ParseDirectionError),
        }
    }
}

#[derive(Debug)]
struct Instruction(Direction, isize);

#[derive(Debug)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_at(1);

        Ok(Self(
            dir.parse().map_err(|_| ParseInstructionError)?,
            dist.parse().map_err(|_| ParseInstructionError)?,
        ))
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point(isize, isize);

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    // creates a new Point, `dist` units away from `self` in the `dir` direction
    fn go(self, Instruction(dir, dist): &Instruction) -> Self {
        let Point(x, y) = self;

        match dir {
            Direction::Up => Point::new(x, y + dist),
            Direction::Right => Point::new(x + dist, y),
            Direction::Down => Point::new(x, y - dist),
            Direction::Left => Point::new(x - dist, y),
        }
    }

    fn dist_to(self, other: Self) -> isize {
        1
    }
}
