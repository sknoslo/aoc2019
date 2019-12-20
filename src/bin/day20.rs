use std::collections::HashMap;

fn main() {
    println!("part 1: {}", "incomplete");
    println!("part 2: {}", "incomplete");
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Void,
    Empty,
    Wall,
    Portal(char, char),
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Tile>,
    portals: HashMap<(usize, usize), (usize, usize)>,
    w: usize,
}

fn parse_maze(input: &str) -> Maze {
    let height = input.lines().count();

    let chars: Vec<_> = input
        .lines()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let width = chars.len() / height;

    let mut portals = HashMap::new();
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
                        (c @ 'A'..='Z', _, _, _) => Tile::Portal(c, chars[i - width * 2]),
                        (_, c @ 'A'..='Z', _, _) => Tile::Portal(c, chars[i + width * 2]),
                        (_, _, c @ 'A'..='Z', _) => Tile::Portal(c, chars[i - 2]),
                        (_, _, _, c @ 'A'..='Z') => Tile::Portal(c, chars[i + 2]),
                        _ => Tile::Empty,
                    }
                }
                '#' => Tile::Wall,
                _ => Tile::Void,
            });
        }
    }

    Maze {
        portals,
        tiles,
        w: width,
    }
}
