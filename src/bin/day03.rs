use aoc2019::get_puzzle_input;
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

fn main() -> io::Result<()> {
    let input = get_puzzle_input()?;

    let input = parse_input(&input);

    let p1 = part1(&input[0], &input[1]);
    println!("part 1: {}", p1);

    let p2 = part2(&input[0], &input[1]);
    println!("part 2: {}", p2);

    Ok(())
}

fn part1(wire_a_inst: &Vec<Instruction>, wire_b_inst: &Vec<Instruction>) -> isize {
    let origin = Point::new(0, 0);
    let mut current = origin;

    let mut wire_a = HashSet::new();
    let mut wire_b = HashSet::new();

    for inst in wire_a_inst {
        let points = current.go(inst);
        for point in points.iter() {
            wire_a.insert(*point);
        }
        current = *points.last().unwrap();
    }

    current = origin;

    for inst in wire_b_inst {
        let points = current.go(inst);
        for point in points.iter() {
            wire_b.insert(*point);
        }
        current = *points.last().unwrap();
    }

    let intersections: Vec<_> = wire_a.intersection(&wire_b).collect();

    let mut min = std::isize::MAX;

    for p in intersections {
        let dist = p.dist_to(origin);

        min = if dist < min { dist } else { min };
    }

    min
}

fn part2(wire_a_inst: &Vec<Instruction>, wire_b_inst: &Vec<Instruction>) -> isize {
    let origin = Point::new(0, 0);
    let mut current = origin;

    let mut map = HashMap::new();

    let mut steps = 0;

    for inst in wire_a_inst {
        let points = current.go(inst);
        for point in points.iter() {
            steps += 1;
            let cell = map.entry(*point).or_insert(MapCell::Empty);

            match cell {
                MapCell::Empty => *cell = MapCell::A(steps),
                MapCell::A(_) => {} // intersect self, ignore to keep lowest steps (the first one)
                _ => panic!("Ahhhhh"),
            }
        }
        current = *points.last().unwrap();
    }

    current = origin;

    let mut steps = 0;

    for inst in wire_b_inst {
        let points = current.go(inst);
        for point in points.iter() {
            steps += 1;
            let cell = map.entry(*point).or_insert(MapCell::Empty);

            match cell {
                MapCell::Empty => *cell = MapCell::B(steps),
                MapCell::A(a_steps) => *cell = MapCell::Intersection(steps + *a_steps),
                MapCell::B(_) => {} // intersect self, just ignore
                _ => panic!("Ahhhhh"),
            }
        }
        current = *points.last().unwrap();
    }

    map.values()
        .map(|v| match v {
            &MapCell::Intersection(steps) => steps,
            _ => std::isize::MAX,
        })
        .min()
        .expect("didn't find a min!")
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
enum MapCell {
    Empty,
    A(isize),            // Contains number of steps A took to get here
    B(isize),            // Contains number of steps B took to get here
    Intersection(isize), // Contains the combined number of steps to intersect
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

    // creates a list of Points, `dist` units away from `self` in the `dir` direction
    fn go(self, Instruction(dir, dist): &Instruction) -> Vec<Self> {
        let Point(x, y) = self;

        (1..*dist + 1)
            .map(|inc| match dir {
                Direction::Up => Point::new(x, y + inc),
                Direction::Right => Point::new(x + inc, y),
                Direction::Down => Point::new(x, y - inc),
                Direction::Left => Point::new(x - inc, y),
            })
            .collect()
    }

    fn dist_to(self, other: Self) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}
