use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

fn main() {
    let maze = parse_maze(include_str!("../../input/20.txt"));

    println!("{}", maze);
    println!("{:?}", maze.portals);

    let p1 = part1(&maze);
    println!("part 1: {}", p1);

    println!("part 2: {}", "incomplete");
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
                let jump = maze
                    .portals
                    .get(&i)
                    .expect("couldn't find the matching portal!");

                if visited.insert(*jump) {
                    to_visit.push_front((*jump, steps + 1));
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
}

fn parse_maze(input: &str) -> Maze {
    let height = input.lines().count();

    let chars: Vec<_> = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();

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
    fn test_parser() {
        let maze = parse_maze(
            "\
 A \n\
 B \n\
#.#
#.#
#.#
 A \n\
 B \n\
",
        );

        assert_eq!(maze.portals, HashMap::new());
    }
}
