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
    // * use BFS (or A* if BFS is too slow?) to find the best path from each key to each other key
    //   * store distances in a map.
    // * starting at ENTRANCE choose the closest key.
    // * continue choosing the closest key, until they are all gone.

    println!("ENTRANCE: {:?}", maze.get_pos_of(Tile::Entry));

    let num_keys = maze.get_keys().len();
    println!("# KEYS:   {}", maze.get_keys().len());
    println!("# KEYS^2: {}", num_keys * num_keys);

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
            c => Tile::Key(c),
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
                    Tile::Key(c) => *c,
                }
            )?;
        }

        Ok(())
    }
}
