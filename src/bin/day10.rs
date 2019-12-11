use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/10.txt").trim();

    let input = parse_input(input);

    let (p1, location) = part1(&input);

    println!("part 1: {}", p1);

    let p2 = part2(&input, location);

    println!("part 2: {}", p2);
}

fn part1(map: &Vec<(isize, isize)>) -> (usize, (isize, isize)) {
    let mut max = 0;
    let mut max_loc = None;

    for a in map.into_iter() {
        let count = map
            .into_iter()
            .filter_map(|b| {
                let (dx, dy) = get_delta(a, b);
                get_minimal_vector(dx, dy)
            })
            .unique()
            .count();

        if count > max {
            max = count;
            max_loc = Some(a);
        }
    }

    (max, *max_loc.unwrap())
}

fn part2(map: &Vec<(isize, isize)>, start: (isize, isize)) -> isize {
    let mut map = map.clone();

    // algo:
    // 1. sort map by angle and radius relative to start.
    // 2. "remove" first item. can just leave them in, since there are more visible than 200, will
    //    never make a second loop around.
    // 3. skip items that were inline with the one just removed. (will have same step)
    map.sort_by(|a, b| {
        let (adx, ady) = get_delta(&start, a);
        let aangle = get_angle(adx, ady);
        let arad = get_radius_squared(adx, ady);
        let (bdx, bdy) = get_delta(&start, b);
        let bangle = get_angle(bdx, bdy);
        let brad = get_radius_squared(bdx, bdy);

        if bangle == aangle {
            arad.cmp(&brad)
        } else {
            aangle.cmp(&bangle)
        }
    });

    let mut removed = 0;
    let mut last_step = (0, 0);

    for &other in map.iter() {
        if other == start {
            continue;
        }

        let (dx, dy) = get_delta(&start, &other);
        let step = get_minimal_vector(dx, dy).unwrap();

        if step == last_step {
            continue;
        }

        removed += 1;

        if removed == 200 {
            return other.0 * 100 + other.1;
        }
        last_step = step;
    }

    0
}

fn get_delta(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    (b.0 - a.0, b.1 - a.1)
}

fn get_radius_squared(x: isize, y: isize) -> isize {
    x * x + y * y
}

// likely to only have enough precision for small vectors, should work for comparison purposes...
// gets the radians from [0, 2PI] * 10000
fn get_angle(x: isize, y: isize) -> isize {
    use std::f64::consts::PI;

    let angle = (y as f64).atan2(x as f64) + PI / 2.0;

    let angle = if angle < 0.0 { 2.0 * PI + angle } else { angle };

    (angle * 10000.0) as isize
}

fn get_minimal_vector(dx: isize, dy: isize) -> Option<(isize, isize)> {
    let f = gcd(dx, dy).abs();

    if f == 0 {
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

fn parse_input(input: &str) -> Vec<(isize, isize)> {
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
    fn get_min_vector_test() {
        assert_eq!(get_minimal_vector(6, 9), Some((2, 3)));
        assert_eq!(get_minimal_vector(0, 9), Some((0, 1)));
        assert_eq!(get_minimal_vector(0, -9), Some((0, -1)));
        assert_eq!(get_minimal_vector(9, 0), Some((1, 0)));
        assert_eq!(get_minimal_vector(-9, 0), Some((-1, 0)));
        assert_eq!(get_minimal_vector(-9, -6), Some((-3, -2)));
    }

    #[test]
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

        assert_eq!(part1(&input), (8, (3, 4)));
    }

    #[test]
    fn part1_test2() {
        let input = parse_input(
            "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
",
        );

        assert_eq!(part1(&input), (33, (5, 8)));
    }

    #[test]
    fn part1_test3() {
        let input = parse_input(
            "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
",
        );

        assert_eq!(part1(&input), (210, (11, 13)));
    }

    #[test]
    fn part2_test1() {
        let input = parse_input(
            "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
",
        );

        let (_, location) = part1(&input);

        assert_eq!(location, (11, 13));
        assert_eq!(part2(&input, location), 802);
    }
}
