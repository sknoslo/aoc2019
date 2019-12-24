use std::collections::{HashSet, VecDeque};
use std::fmt;

fn main() {
    let input = include_str!("../../input/24.txt").trim();

    let map = parse_input(&input);

    let p1 = part1(map);
    println!("part 1: {}", p1);

    let map = parse_input2(&input);

    let p2 = part2(map, 200);
    println!("part 2: {}", p2);
}

fn part1(mut map: Map) -> usize {
    let mut seen = HashSet::new();

    while {
        map.tick();

        seen.insert(map.bio_rating())
    } {}

    map.bio_rating()
}

fn part2(mut map: RecursiveMap, minutes: usize) -> usize {
    for _ in 0..minutes {
        map.tick();
    }

    map.layers.iter().fold(0, |count, layer| {
        count
            + layer
                .iter()
                .filter(|&cell| *cell == RecursiveCell::Infested)
                .count()
    })
}

static SIZE: isize = 5;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Infested,
    Empty,
}

#[derive(Debug, Clone)]
struct Map {
    cells: Vec<Cell>,
    next: Vec<Cell>,
}

impl Map {
    fn tick(&mut self) {
        for (i, cell) in self.cells.iter().enumerate() {
            self.next[i] = match (cell, self.count_adjacent_bugs(i)) {
                (Cell::Infested, 1) => Cell::Infested,
                (Cell::Infested, _) => Cell::Empty,
                (Cell::Empty, 1..=2) => Cell::Infested,
                (Cell::Empty, _) => Cell::Empty,
            };
        }

        std::mem::swap(&mut self.cells, &mut self.next);
    }

    fn bio_rating(&self) -> usize {
        self.cells
            .iter()
            .enumerate()
            .fold(0, |acc, (i, cell)| match cell {
                Cell::Infested => acc + 2_usize.pow(i as u32),
                Cell::Empty => acc,
            })
    }

    fn count_adjacent_bugs(&self, i: usize) -> usize {
        let y = i as isize / SIZE;
        let x = i as isize % SIZE;

        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .iter()
            .fold(0, |bugs, &(ax, ay)| {
                if ax < 0 || ax >= SIZE || ay < 0 || ay >= SIZE {
                    bugs
                } else {
                    let ii = (ay * SIZE + ax) as usize;
                    bugs + match self.cells[ii] {
                        Cell::Infested => 1,
                        Cell::Empty => 0,
                    }
                }
            })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RecursiveCell {
    Infested,
    Empty,
    Recursive,
}

#[derive(Debug)]
struct RecursiveMap {
    layers: VecDeque<Vec<RecursiveCell>>,
    next: VecDeque<Vec<RecursiveCell>>,
}

impl RecursiveMap {
    fn tick(&mut self) {
        let should_add_outer = self.layers[0]
            .iter()
            .filter(|&c| *c == RecursiveCell::Infested)
            .count()
            > 0;
        let should_add_inner = self.layers[self.layers.len() - 1]
            .iter()
            .filter(|&c| *c == RecursiveCell::Infested)
            .count()
            > 0;

        if should_add_outer || should_add_inner {
            let mut new_layer = vec![RecursiveCell::Empty; (SIZE * SIZE) as usize];
            new_layer[12] = RecursiveCell::Recursive;

            if should_add_outer {
                self.layers.push_front(new_layer.clone());
                self.next.push_front(new_layer.clone());
            }

            if should_add_inner {
                self.layers.push_back(new_layer.clone());
                self.next.push_back(new_layer.clone());
            }
        }

        for (depth, layer) in self.layers.iter().enumerate() {
            for (i, cell) in layer.iter().enumerate() {
                let adjacent_bugs = self.count_adjacent_bugs(depth, i);

                self.next[depth][i] = match (cell, adjacent_bugs) {
                    (RecursiveCell::Infested, 1) => RecursiveCell::Infested,
                    (RecursiveCell::Infested, _) => RecursiveCell::Empty,
                    (RecursiveCell::Empty, 1..=2) => RecursiveCell::Infested,
                    (RecursiveCell::Empty, _) => RecursiveCell::Empty,
                    (c, _) => *c,
                }
            }
        }

        std::mem::swap(&mut self.layers, &mut self.next);
    }

    fn count_adjacent_bugs(&self, depth: usize, i: usize) -> usize {
        let y = i as isize / SIZE;
        let x = i as isize % SIZE;

        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .iter()
            .fold(0, |bugs, &(ax, ay)| {
                if ax < 0 || ax >= SIZE || ay < 0 || ay >= SIZE {
                    if depth == 0 {
                        bugs
                    } else {
                        // 🤮
                        let outer_cells = match (ax, ay) {
                            (-1, -1) => vec![11, 7],
                            (-1, 5) => vec![11, 17],
                            (-1, _) => vec![11],
                            (5, -1) => vec![7, 13],
                            (5, 5) => vec![13, 17],
                            (5, _) => vec![13],
                            (_, 5) => vec![17],
                            (_, -1) => vec![7],
                            _ => unreachable!(),
                        };

                        bugs + outer_cells.iter().fold(0, |acc, outer_cell| {
                            match self.layers[depth - 1][*outer_cell] {
                                RecursiveCell::Infested => acc + 1,
                                _ => acc,
                            }
                        })
                    }
                } else {
                    let ii = (ay * SIZE + ax) as usize;

                    // I'm sure there is some beautiful way to express this, but hey, what are you
                    // gonna do?
                    bugs + match self.layers[depth][ii] {
                        RecursiveCell::Infested => 1,
                        RecursiveCell::Empty => 0,
                        RecursiveCell::Recursive => {
                            if depth + 1 == self.layers.len() {
                                0
                            } else {
                                let inner_cells = match i {
                                    7 => [0, 1, 2, 3, 4],
                                    11 => [0, 5, 10, 15, 20],
                                    13 => [4, 9, 14, 19, 24],
                                    17 => [20, 21, 22, 23, 24],
                                    _ => unreachable!(),
                                };

                                inner_cells.iter().fold(0, |acc, inner_cell| {
                                    match self.layers[depth + 1][*inner_cell] {
                                        RecursiveCell::Infested => acc + 1,
                                        _ => acc,
                                    }
                                })
                            }
                        }
                    }
                }
            })
    }
}

fn parse_input(input: &str) -> Map {
    let cells: Vec<_> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Cell::Infested,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let next = cells.clone();

    Map { cells, next }
}

fn parse_input2(input: &str) -> RecursiveMap {
    let mut layer0: Vec<_> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => RecursiveCell::Infested,
                    '.' => RecursiveCell::Empty,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    layer0[12] = RecursiveCell::Recursive;

    let mut layers = VecDeque::new();
    layers.push_front(layer0);

    let next = layers.clone();

    RecursiveMap { layers, next }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            if i > 0 && i as isize % SIZE == 0 {
                write!(f, "\n")?;
            }

            write!(
                f,
                "{}",
                match cell {
                    Cell::Infested => '#',
                    Cell::Empty => '.',
                }
            )?;
        }

        write!(f, "\n")
    }
}

impl fmt::Display for RecursiveMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (depth, layer) in self.layers.iter().enumerate() {
            writeln!(f, "depth: {}", depth)?;
            for (i, cell) in layer.iter().enumerate() {
                if i > 0 && i as isize % SIZE == 0 {
                    write!(f, "\n")?;
                }

                write!(
                    f,
                    "{}",
                    match cell {
                        RecursiveCell::Empty => '.',
                        RecursiveCell::Infested => '#',
                        RecursiveCell::Recursive => '?',
                    }
                )?;
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let map = parse_input(
            "\
....#
#..#.
#..##
..#..
#....",
        );

        assert_eq!(part1(map), 2129920);
    }

    #[test]
    fn part2_test1() {
        let map = parse_input2(
            "\
....#
#..#.
#..##
..#..
#....",
        );

        assert_eq!(part2(map, 10), 99);
    }
}
