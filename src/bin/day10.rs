use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/10.txt").trim();

    let input = parse_input(input);

    let p1 = part1(&input);

    println!("part 1: {}", p1);
}

fn part1(map: &HashSet<(isize, isize)>) -> usize {
    let mut max = 0;

    for a in map.into_iter() {
        let mut count = 0;

        for b in map.into_iter() {
            if a == b {
                continue;
            }

            let (dx, dy) = get_delta(a, b);

            // nothing can be blocking these
            if dx == 1 || dy == 1 {
                count += 1;
                continue;
            }

            if let Some(step) = get_step(dx, dy) {
                // find points on line between a and b and see if they exist in the map.
                let mut x = a.0 + step.0;
                let mut y = a.1 + step.1;

                let mut collides = false;
                let mut loops = 0;
                while (a.0 < b.0 && x < b.0) || (a.0 > b.0 && x > b.0) {
                    loops += 1;

                    if loops > 10 {
                        // println!("artificially breaking looop");
                        break;
                    }
                    // println!("looping");
                    if map.contains(&(x, y)) {
                        collides = true;
                        break;
                    }

                    x += step.0;
                    y += step.1;
                }

                // println!("broke loop");

                if !collides {
                    count += 1;
                }
            } else {
                // no steps between, so no blocks
                count += 1;
            }
        }

        if count > max {
            max = count;
        }
    }

    max
}

fn get_delta(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    (b.0 - a.0, b.1 - a.1)
}

fn get_step(dx: isize, dy: isize) -> Option<(isize, isize)> {
    let f = gcd(dx, dy);

    println!("{}, {}, gcd: {} -> {}, {}", dx, dy, f, dx / f, dy / f);

    if f == 0 || f == 1 {
        None
    } else {
        Some((dx / f, dy / f))
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn parse_input(input: &str) -> HashSet<(isize, isize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(6, 9), 3);
        assert_eq!(gcd(9, 9), 9);
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(2, 9), 1);
    }

    #[test]
    fn get_step_test() {
        assert_eq!(get_step(6, 9), Some((2, 3)));
        assert_eq!(get_step(0, 9), Some((0, 1)));
        assert_eq!(get_step(0, -9), Some((0, -1)));
        assert_eq!(get_step(9, 0), Some((1, 0)));
        assert_eq!(get_step(-9, 0), Some((-1, 0)));
        assert_eq!(get_step(-9, -6), Some((-3, -2)));
    }

    // #[test]
    fn part1_test1() {
        let input = parse_input(
            "\
.#..#
.....
#####
....#
...##
",
        );

        assert_eq!(part1(&input), 8);
    }
}
