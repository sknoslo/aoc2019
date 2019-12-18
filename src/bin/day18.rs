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
    // * ... i don't have one... previous idea was based on false understanding...

    println!("ENTRANCE: {:?}", maze.get_pos_of(Tile::Entry));

    let num_keys = maze.get_keys().len();
    let num_doors = maze.get_doors().len();
    println!("# KEYS:   {}", num_keys);
    println!("# DOORS:  {}", num_doors);

    let num_empty = maze.tiles.iter().filter(|&&t| t == Tile::Empty).count();
    println!("EXPLORE:  {}", num_empty);

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
    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * self.w + x]
    }

    fn get_pos_of(&self, target: Tile) -> (usize, usize) {
        let i = self.tiles.iter().position(|&tile| tile == target).unwrap();

        (i % self.w, i / self.w)
    }

    fn get_keys(&self) -> Vec<Tile> {
        self.tiles
            .iter()
            .cloned()
            .filter(|t| if let Tile::Key(_) = t { true } else { false })
            .collect()
    }

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
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
        );

        assert_eq!(part1(&maze), 86);
    }

    #[test]
    fn part1_test3() {
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
    fn part1_test4() {
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
}
