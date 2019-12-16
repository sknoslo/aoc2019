use aoc2019::computer::io::IoDevice;
use aoc2019::computer::{parse_program, Computer};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::rc::Rc;

fn main() {
    let program = parse_program(include_str!("../../input/15.txt").trim());

    let p1 = part1(&program);
    println!("part 1: {}", p1);

    let p2 = part2(&program);
    println!("part 2: {}", p2);
}

fn part1(program: &Vec<isize>) -> usize {
    let map = build_map(program);

    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();

    to_visit.push_front((0, 0, 0));

    while let Some((x, y, dist)) = to_visit.pop_back() {
        if !visited.insert((x, y)) {
            continue;
        }

        match map.get(&(x, y)) {
            Some(2) => return dist,
            Some(1) => {
                to_visit.push_front((x, y - 1, dist + 1));
                to_visit.push_front((x, y + 1, dist + 1));
                to_visit.push_front((x - 1, y, dist + 1));
                to_visit.push_front((x + 1, y, dist + 1));
            }
            _ => {}
        }
    }

    0
}

fn part2(program: &Vec<isize>) -> usize {
    let map = build_map(program);

    let starting_pos = map
        .iter()
        .find(|(_, v)| **v == 2)
        .map(|(k, _)| k)
        .expect("didn't find the oxygen source");

    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    let mut max_time = 0;

    to_visit.push_front((starting_pos.0, starting_pos.1, 0));

    while let Some((x, y, time)) = to_visit.pop_back() {
        if !visited.insert((x, y)) {
            continue;
        }

        match map.get(&(x, y)) {
            Some(1) | Some(2) => {
                if time > max_time {
                    max_time = time;
                }

                to_visit.push_front((x, y - 1, time + 1));
                to_visit.push_front((x, y + 1, time + 1));
                to_visit.push_front((x - 1, y, time + 1));
                to_visit.push_front((x + 1, y, time + 1));
            }
            _ => {}
        }
    }

    max_time
}

fn build_map(program: &Vec<isize>) -> HashMap<(isize, isize), isize> {
    let controller = Rc::new(RefCell::new(RepairDroidController::new()));
    let mut comp = Computer::new(Some(controller.clone()), Some(controller.clone()));

    comp.load(&program);
    comp.run();

    let controller = controller.borrow();

    controller.map.clone() // can this be avoided?
}

#[derive(Debug)]
struct RepairDroidController {
    pos: (isize, isize),
    map: HashMap<(isize, isize), isize>,
    next_moves: Vec<(isize, isize, isize)>, // (dir, x, y)
    breadcrumbs: Vec<isize>,
    backtracking: bool,
    last_move: isize,
}

impl RepairDroidController {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            map: [((0, 0), 1)].iter().cloned().collect(),
            next_moves: vec![(1, 0, 0), (2, 0, 0), (3, 0, 0), (4, 0, 0)],
            breadcrumbs: Vec::new(),
            backtracking: false,
            last_move: 0,
        }
    }

    fn get_pos_in_dir(&self, dir: isize) -> (isize, isize) {
        match (dir, self.pos.0, self.pos.1) {
            (1, x, y) => (x, y - 1), // go north
            (2, x, y) => (x, y + 1), // go south
            (3, x, y) => (x - 1, y), // go west
            (4, x, y) => (x + 1, y), // go east
            _ => panic!("cannot travel in that direction"),
        }
    }

    fn drop_breadcrumb(&mut self) {
        self.breadcrumbs.push(match self.last_move {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => panic!("cannot travel in that direction"),
        })
    }

    fn create_next_moves(&mut self) {
        for dir in 1..=4 {
            let next_pos = self.get_pos_in_dir(dir);

            if !self.map.contains_key(&next_pos) {
                self.next_moves.push((dir, self.pos.0, self.pos.1));
            }
        }
    }
}

impl IoDevice for RepairDroidController {
    fn read(&mut self) -> Option<isize> {
        if self.backtracking {
            if let Some(dir) = self.breadcrumbs.pop() {
                self.last_move = dir;
                Some(dir)
            } else {
                None
            }
        } else {
            if let Some((dir, _, _)) = self.next_moves.pop() {
                self.last_move = dir;
                Some(dir)
            } else {
                None
            }
        }
    }

    fn write(&mut self, value: isize) {
        if self.backtracking {
            self.pos = self.get_pos_in_dir(self.last_move);
            if let Some((_, x, y)) = self.next_moves.last() {
                if *x == self.pos.0 && *y == self.pos.1 {
                    self.backtracking = false;
                }
            }
            return;
        }

        match value {
            0 => {
                let pos = self.get_pos_in_dir(self.last_move);
                self.map.insert(pos, 0);
                if let Some((_, x, y)) = self.next_moves.last() {
                    if *x != self.pos.0 || *y != self.pos.1 {
                        self.backtracking = true;
                    }
                }
                return;
            }
            1 | 2 => {
                let pos = self.get_pos_in_dir(self.last_move);
                self.map.insert(pos, value);
                self.drop_breadcrumb();
                self.pos = pos;
                self.create_next_moves();
            }
            _ => panic!("not valid output!"),
        };
    }
}

impl Display for RepairDroidController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut minx = 0;
        let mut miny = 0;
        let mut maxx = 0;
        let mut maxy = 0;

        for (x, y) in self.map.keys() {
            if *x < minx {
                minx = *x;
            } else if *x > maxx {
                maxx = *x;
            }

            if *y < miny {
                miny = *y;
            } else if *y > maxy {
                maxy = *y;
            }
        }

        for y in miny..=maxy {
            for x in minx..=maxx {
                if self.pos == (x, y) {
                    write!(f, "@")?;
                } else {
                    let c = self.map.get(&(x, y)).map_or('.', |c| match c {
                        0 => '#',
                        1 => '.',
                        2 => '$',
                        _ => panic!("cannot display that!"),
                    });
                    write!(f, "{}", c)?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
