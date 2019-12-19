use std::collections::{HashSet, VecDeque};
use std::fmt;

fn main() {
    let maze = parse_maze(include_str!("../../input/18.txt").trim());

    let p1 = part1(&maze);
    println!("part 1: {}", p1);
    println!("part 2: {}", "incomplete");
}

fn part1(maze: &Maze) -> usize {
    println!("{}", maze);

    // IDEA:
    // * do a BFS, where each node on the queue is the postion + the set of keys that have been
    //   found + the number of steps taken to get here.
    // * when all keys have been found, you are at the optimal node.
    //
    // * Okay, this works but uses about 5-6 GB of RAM. Gotta be some optimization oportunity...
    //   * Djikstras?
    //   * Build a proper graph out of the maze?

    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::new();

    let total_keys = maze.get_keys().len();

    let start = maze.get_pos_of(Tile::Entry);

    to_visit.push_front((start, vec![], 0));

    while let Some((i, mut keys, steps)) = to_visit.pop_back() {
        let tile = maze.get_tile(i);

        if tile == Tile::Wall {
            continue;
        } else if let Tile::Door(c) = tile {
            if !keys.contains(&c.to_ascii_lowercase()) {
                // can't get passed this door yet
                continue;
            }
        }

        if let Tile::Key(c) = tile {
            // if we don't already have this key, put in on the keychain!
            if !keys.contains(&c) {
                keys.push(c);
                if keys.len() == total_keys {
                    return steps;
                }

                keys.sort(); // sort keys to better avoid duplicates
            }
        }

        let up = i - maze.w;
        let down = i + maze.w;
        let left = i - 1;
        let right = i + 1;
        let steps = steps + 1;

        if visited.insert((up, keys.clone())) {
            to_visit.push_front((up, keys.clone(), steps));
        }
        if visited.insert((down, keys.clone())) {
            to_visit.push_front((down, keys.clone(), steps));
        }
        if visited.insert((left, keys.clone())) {
            to_visit.push_front((left, keys.clone(), steps));
        }
        if visited.insert((right, keys.clone())) {
            to_visit.push_front((right, keys.clone(), steps));
        }
    }

    0
}

fn parse_maze(input: &str) -> Maze {
    let height = input.lines().count();

    let tiles: Vec<_> = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            '@' => Tile::Entry,
            'a'..='z' => Tile::Key(c),
            'A'..='Z' => Tile::Door(c),
            _ => unreachable!(),
        })
        .collect();

    let width = tiles.len() / height;

    Maze {
        tiles,
        w: width,
        h: height,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Entry,
    Key(char),
    Door(char),
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
}

impl Maze {
    fn get_tile(&self, i: usize) -> Tile {
        self.tiles[i]
    }

    fn get_pos_of(&self, target: Tile) -> usize {
        self.tiles.iter().position(|&tile| tile == target).unwrap()
    }

    fn get_keys(&self) -> Vec<Tile> {
        self.tiles
            .iter()
            .cloned()
            .filter(|t| if let Tile::Key(_) = t { true } else { false })
            .collect()
    }

    #[allow(dead_code)]
    fn get_doors(&self) -> Vec<Tile> {
        self.tiles
            .iter()
            .cloned()
            .filter(|t| if let Tile::Door(_) = t { true } else { false })
            .collect()
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, tile) in self.tiles.iter().enumerate() {
            if i % self.w == 0 {
                write!(f, "\n")?;
            }

            write!(
                f,
                "{}",
                match tile {
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                    Tile::Entry => '@',
                    Tile::Key(c) | Tile::Door(c) => *c,
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
        let maze = parse_maze(
            "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
        );

        assert_eq!(part1(&maze), 86);
    }

    #[test]
    fn part1_test2() {
        let maze = parse_maze(
            "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
        );

        assert_eq!(part1(&maze), 132);
    }

    #[test]
    fn part1_test3() {
        let maze = parse_maze(
            "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
        );

        assert_eq!(part1(&maze), 136);
    }

    #[test]
    fn part1_test4() {
        let maze = parse_maze(
            "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
        );

        assert_eq!(part1(&maze), 81);
    }
}
