use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

fn main() {
    let maze = parse_maze(include_str!("../../input/20.txt"));

    let p1 = part1(&maze);
    println!("part 1: {}", p1);

    let p2 = part2(&maze);
    println!("part 2: {}", p2);
}

fn part1(maze: &Maze) -> usize {
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();

    let start = maze.get_start();
    let end = maze.get_end();

    to_visit.push_front((start, 0));
    visited.insert(start);

    while let Some((i, steps)) = to_visit.pop_back() {
        if i == end {
            return steps;
        }

        match maze.tiles[i] {
            Tile::Void | Tile::Wall => continue,
            Tile::Empty => maze.get_adjacent(i).iter().for_each(|next| {
                if visited.insert(*next) {
                    to_visit.push_front((*next, steps + 1));
                }
            }),
            Tile::Portal(_, _) => {
                if let Some(jump) = maze.portals.get(&i) {
                    if visited.insert(*jump) {
                        to_visit.push_front((*jump, steps + 1));
                    }
                }

                maze.get_adjacent(i).iter().for_each(|next| {
                    if visited.insert(*next) {
                        to_visit.push_front((*next, steps + 1));
                    }
                })
            }
        }
    }

    panic!("couldn't find a way out!");
}

fn part2(maze: &Maze) -> usize {
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();

    let start = maze.get_start();
    let end = maze.get_end();

    to_visit.push_front((start, 0, 0));
    visited.insert((start, 0));

    while let Some((i, lvl, steps)) = to_visit.pop_back() {
        if i == end && lvl == 0 {
            return steps;
        } else if lvl > 0 && (i == start || i == end) {
            // treat this like a wall
            continue;
        }

        match maze.tiles[i] {
            Tile::Void | Tile::Wall => continue,
            Tile::Empty => maze.get_adjacent(i).iter().for_each(|next| {
                if visited.insert((*next, lvl)) {
                    to_visit.push_front((*next, lvl, steps + 1));
                }
            }),
            Tile::Portal(_, _) => {
                if let Some(jump) = maze.portals.get(&i) {
                    let lvl = if maze.is_inner(i) {
                        if lvl == 0 {
                            // at level 0, treat inner portals like walls
                            continue;
                        }

                        lvl - 1
                    } else {
                        lvl + 1
                    };

                    if visited.insert((*jump, lvl)) {
                        to_visit.push_front((*jump, lvl, steps + 1));
                    }
                }

                maze.get_adjacent(i).iter().for_each(|next| {
                    if visited.insert((*next, lvl)) {
                        to_visit.push_front((*next, lvl, steps + 1));
                    }
                })
            }
        }
    }

    panic!("couldn't find a way out!");
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Void,
    Empty,
    Wall,
    Portal(char, char),
}

impl Tile {
    fn is_portal(&self) -> bool {
        match self {
            Tile::Portal(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Tile>,
    portals: HashMap<usize, usize>,
    width: usize,
    height: usize,
}

impl Maze {
    fn get_start(&self) -> usize {
        self.tiles
            .iter()
            .position(|&tile| tile == Tile::Portal('A', 'A'))
            .expect("Maze didn't have a starting point.")
    }

    fn get_end(&self) -> usize {
        self.tiles
            .iter()
            .position(|&tile| tile == Tile::Portal('Z', 'Z'))
            .expect("Maze didn't have an ending point.")
    }

    fn get_adjacent(&self, i: usize) -> Vec<usize> {
        vec![i + self.width, i - self.width, i + 1, i - 1]
            .into_iter()
            .filter(|&pos| match self.tiles[pos] {
                Tile::Empty | Tile::Portal(_, _) => true,
                _ => false,
            })
            .collect()
    }

    fn is_inner(&self, i: usize) -> bool {
        let x = i % self.width;
        let y = i / self.width;

        x == 2 || x == self.width - 3 || y == 2 || y == self.height - 3
    }
}

fn parse_maze(input: &str) -> Maze {
    let height = input.lines().count();

    let chars: Vec<_> = input.chars().filter(|&c| c != '\n').collect();

    let width = chars.len() / height;

    let mut tiles = Vec::with_capacity(chars.len());

    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;

            tiles.push(match chars[i] {
                '.' => {
                    match (
                        chars[i - width],
                        chars[i + width],
                        chars[i - 1],
                        chars[i + 1],
                    ) {
                        (c @ 'A'..='Z', _, _, _) => Tile::Portal(chars[i - width * 2], c),
                        (_, c @ 'A'..='Z', _, _) => Tile::Portal(c, chars[i + width * 2]),
                        (_, _, c @ 'A'..='Z', _) => Tile::Portal(chars[i - 2], c),
                        (_, _, _, c @ 'A'..='Z') => Tile::Portal(c, chars[i + 2]),
                        (_, _, _, _) => Tile::Empty,
                    }
                }
                '#' => Tile::Wall,
                _ => Tile::Void,
            });
        }
    }

    let mut portals = HashMap::new();

    for (a, tile) in tiles.iter().enumerate().filter(|&(_, t)| t.is_portal()) {
        if portals.contains_key(&a) {
            continue;
        }

        for b in a + 1..tiles.len() {
            if tiles[b] == *tile {
                portals.insert(a, b);
                portals.insert(b, a);

                break;
            }
        }
    }

    Maze {
        portals,
        tiles,
        width,
        height,
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, tile) in self.tiles.iter().enumerate() {
            if i % self.width == 0 {
                write!(f, "\n")?;
            }

            write!(
                f,
                "{}",
                match tile {
                    Tile::Void => ' ',
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                    Tile::Portal(a, _) => *a,
                }
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let maze = parse_maze(include_str!("../../test-input/20/test1.txt"));

        assert_eq!(part1(&maze), 23);
    }
}
