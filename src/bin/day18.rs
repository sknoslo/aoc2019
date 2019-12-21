use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

fn main() {
    let maze = parse_maze(include_str!("../../input/18.txt").trim());

    let p1 = part1(&maze);
    println!("part 1: {}", p1);

    let mut maze = parse_maze(include_str!("../../input/18.txt").trim());

    maze.split_quadrants();

    let p2 = part2(&maze);
    println!("part 2: {}", p2);
}

fn part1(maze: &Maze) -> usize {
    let paths = build_paths(&maze);
    let total_keys = maze.get_keys().len();

    let mut to_visit = Vec::new();
    let mut visited = HashMap::new();

    to_visit.push(('@', 0, Vec::new()));
    visited.insert(('@', Vec::new()), 0);

    let mut min = std::usize::MAX;

    while let Some((a, steps, collected)) = to_visit.pop() {
        if collected.len() == total_keys {
            if steps < min {
                min = steps;
            }
            continue;
        }

        if let Some(next_keys) = paths.get(&a) {
            next_keys.iter().cloned().for_each(|(key, dist, doors)| {
                if !collected.contains(&key) && has_required_keys(&collected, &doors) {
                    let mut next_collected = collected.clone();

                    next_collected.sort();
                    next_collected.push(key);

                    let entry = visited
                        .entry((key, next_collected.clone()))
                        .or_insert(std::usize::MAX);

                    if steps + dist < *entry {
                        *entry = steps + dist;
                        to_visit.push((key, steps + dist, next_collected));
                    }
                }
            });
        }
    }

    min
}

fn part2(_maze: &Maze) -> usize {
    0
}

fn has_required_keys(collected: &Vec<char>, doors: &Vec<char>) -> bool {
    doors.iter().fold(true, |acc, door| {
        acc && collected.contains(&door.to_ascii_lowercase())
    })
}

fn build_paths(maze: &Maze) -> HashMap<char, Vec<(char, usize, Vec<char>)>> {
    let mut paths = HashMap::new();

    for &source in maze.get_keys_and_entry().iter() {
        for &target in maze.get_keys().iter() {
            if source == target {
                continue;
            }

            let a = maze.get_pos_of(source);
            let b = maze.get_pos_of(target);

            let mut to_visit: VecDeque<(usize, usize, Vec<char>)> = VecDeque::new();
            let mut visited: HashSet<usize> = HashSet::new();

            to_visit.push_front((a, 0, vec![]));
            visited.insert(a);

            while let Some((i, steps, mut required_keys)) = to_visit.pop_back() {
                if i == b {
                    let entry = paths.entry(source.label()).or_insert(vec![]);

                    entry.push((target.label(), steps, required_keys));
                    break;
                }

                match maze.tiles[i] {
                    Tile::Wall => continue,
                    Tile::Door(c) => {
                        required_keys.push(c);
                        required_keys.sort();
                    }
                    _ => {}
                }

                [i - maze.w, i + maze.w, i - 1, i + 1]
                    .iter()
                    .cloned()
                    .for_each(|next| {
                        if visited.insert(next) {
                            to_visit.push_front((next, steps + 1, required_keys.clone()));
                        }
                    });
            }
        }
    }

    paths
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Entry,
    Key(char),
    Door(char),
}

impl Tile {
    fn label(&self) -> char {
        match self {
            Tile::Key(c) | Tile::Door(c) => *c,
            Tile::Entry => '@',
            _ => panic!("can only unwrap doors and keys"),
        }
    }

    fn is_key(&self) -> bool {
        match self {
            Tile::Key(_) => true,
            _ => false,
        }
    }

    fn is_entry(&self) -> bool {
        match self {
            Tile::Entry => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Tile>,
    w: usize,
    h: usize,
}

impl Maze {
    fn get_pos_of(&self, target: Tile) -> usize {
        self.tiles.iter().position(|&tile| tile == target).unwrap()
    }

    fn get_keys_and_entry(&self) -> Vec<Tile> {
        self.tiles
            .iter()
            .cloned()
            .filter(|t| t.is_entry() || t.is_key())
            .collect()
    }

    fn get_keys(&self) -> Vec<Tile> {
        self.tiles.iter().cloned().filter(|t| t.is_key()).collect()
    }

    fn get_quad_entry_positions(&self) -> (usize, usize, usize, usize) {
        let first = self.get_pos_of(Tile::Entry);

        (first, first + 2, first + 2 * self.w, first + 2 * self.w + 2)
    }

    fn split_quadrants(&mut self) {
        let i = self.get_pos_of(Tile::Entry);

        self.tiles[i] = Tile::Wall;
        self.tiles[i - self.w] = Tile::Wall;
        self.tiles[i + self.w] = Tile::Wall;
        self.tiles[i - 1] = Tile::Wall;
        self.tiles[i + 1] = Tile::Wall;
        self.tiles[i - self.w - 1] = Tile::Entry;
        self.tiles[i - self.w + 1] = Tile::Entry;
        self.tiles[i + self.w - 1] = Tile::Entry;
        self.tiles[i + self.w + 1] = Tile::Entry;
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

fn parse_maze(input: &str) -> Maze {
    let height = input.lines().count();

    let tiles: Vec<_> = input
        .chars()
        .filter(|&c| !c.is_whitespace())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_keys_test() {
        assert_eq!(has_required_keys(&vec!['a', 'b'], &vec!['A', 'B']), true);
        assert_eq!(
            has_required_keys(&vec!['a', 'c', 'b'], &vec!['A', 'B']),
            true
        );
        assert_eq!(
            has_required_keys(&vec!['a', 'b'], &vec!['A', 'C', 'B']),
            false
        );
    }

    #[test]
    fn part1_test1() {
        let mut maze = parse_maze(
            "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
        );

        assert_eq!(part1(&mut maze), 86);
    }

    #[test]
    fn part1_test2() {
        let mut maze = parse_maze(
            "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
        );

        assert_eq!(part1(&mut maze), 132);
    }

    #[test]
    fn part1_test3() {
        let mut maze = parse_maze(
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

        assert_eq!(part1(&mut maze), 136);
    }

    #[test]
    fn part1_test4() {
        let mut maze = parse_maze(
            "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
        );

        assert_eq!(part1(&mut maze), 81);
    }

    // #[test]
    fn part2_test4() {
        let maze = parse_maze(
            "\
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
        );

        assert_eq!(part2(&maze), 72);
    }
}
