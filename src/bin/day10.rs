use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input/10.txt").trim();

    let input = parse_input(input);

    let (p1, location) = part1(&input);

    println!("part 1: {}", p1);

    let p2 = part2(&input, location);

    println!("part 2: {}", p2);
}

fn part1(map: &HashSet<(isize, isize)>) -> (usize, (isize, isize)) {
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

fn part2(map: &HashSet<(isize, isize)>, a: (isize, isize)) -> isize {
    let mut points = map
        .into_iter()
        .filter_map(|b| {
            let (dx, dy) = get_delta(&a, b);
            get_minimal_vector(dx, dy).map(|v| (v, *b))
        })
        .collect::<Vec<_>>();

    points.sort_by(|(_, a), (_, b)| a.cmp(b));

    points.sort_by(|((ax, ay), _), ((bx, by), _)| {
        (*ax as f64)
            .atan2(*ay as f64)
            .partial_cmp(&(*bx as f64).atan2(*by as f64))
            .unwrap()
    });

    let points: Vec<_> = points.iter().unique_by(|(v, _)| v).collect();

    let twohundredth = points[200].1;

    twohundredth.0 * 100 + twohundredth.1
}

fn get_delta(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    (b.0 - a.0, b.1 - a.1)
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

        assert_eq!(part2(&input, location), 802);
    }
}
