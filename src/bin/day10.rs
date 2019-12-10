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
            if dy % dx != 0 || dx == 1 || dy == 1 {
                count += 1;
                continue;
            }

            let slope = dy / dx; // handle dx == 0 , step = (+/-1, 0)

            // find points on line between a and b and see if they exist in the map.
            let mut x = a.0 + step.0;
            let mut y = a.1 + step.1;
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
