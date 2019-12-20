use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

fn main() {
    let mut maze = parse_maze(include_str!("../../input/18.txt").trim());

    let p1 = part1(&mut maze);
    println!("part 1: {}", p1);

    let mut maze = parse_maze(include_str!("../../input/18.txt").trim());

    maze.split_quadrants();

    let p2 = part2(&maze);
    println!("part 2: {}", p2);
}

fn part1(maze: &mut Maze) -> usize {
    let start = maze.get_pos_of(Tile::Entry);
    let keys = maze
        .get_keys()
        .into_iter()
        .map(|key| match key {
            Tile::Key(k) => k,
            _ => panic!("what!"),
        })
        .collect();

    maze.steps_to_collect(start, keys)
}

fn part2(_maze: &Maze) -> usize {
    0
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
    cache: HashMap<(usize, Vec<char>), usize>,
    other_cache: HashMap<(usize, usize, Vec<char>), usize>,
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

    fn steps_to_collect(&mut self, a: usize, mut keys: Vec<char>) -> usize {
        if keys.len() == 0 {
            return 0;
        }

        keys.sort();

        if let Some(dist) = self.cache.get(&(a, keys.clone())) {
            return *dist;
        }

        let mut result = std::usize::MAX;

        for key in keys.iter() {
            let key_pos = self.get_pos_of(Tile::Key(*key));

            let cached_dist = self.other_cache.get(&(a, key_pos, keys.clone()));

            if let Some(d) = cached_dist {
                println!("cache hit");
                let dist = *d
                    + self.steps_to_collect(
                        key_pos,
                        keys.iter().cloned().filter(|k| k != key).collect(),
                    );

                result = std::cmp::min(result, dist);
            } else if let Some(mut dist) = self.steps_between(a, key_pos, &keys) {
                self.other_cache.insert((a, key_pos, keys.clone()), dist);

                dist += self
                    .steps_to_collect(key_pos, keys.iter().cloned().filter(|k| k != key).collect());

                result = std::cmp::min(result, dist);
            }
        }

        self.cache.insert((a, keys.clone()), result);

        result
    }

    fn steps_between(&self, a: usize, b: usize, keys: &Vec<char>) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();

        to_visit.push_front((a, 0));
        visited.insert(a);

        while let Some((i, steps)) = to_visit.pop_back() {
            if i == b {
                return Some(steps);
            }

            let steps = steps + 1;

            match self.get_tile(i) {
                Tile::Wall => continue,
                Tile::Door(c) if keys.contains(&c.to_ascii_lowercase()) => {
                    // println!("ran into door {} - {:?}", c, keys);
                    continue;
                }
                _ => {
                    vec![i - self.w, i + self.w, i - 1, i + 1]
                        .iter()
                        .for_each(|&pos| {
                            if visited.insert(pos) {
                                to_visit.push_front((pos, steps));
                            }
                        });
                }
            }
        }

        None
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
        cache: HashMap::new(),
        other_cache: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
