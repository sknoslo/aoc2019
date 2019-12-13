use aoc2019::vec3::Vec3;
use num::Integer;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let planets = parse_input(include_str!("../../input/12.txt").trim());

    let p1 = part1(&mut planets.clone(), 1000);

    println!("part 1: {}", p1);

    let p2 = part2(&mut planets.clone());

    println!("part 2: {}", p2);
}

fn part1(mut planets: &mut Vec<Planet>, steps: usize) -> isize {
    for _ in 1..=steps {
        simulate(&mut planets);
    }

    planets
        .iter()
        .map(|p| energy(&p.pos) * energy(&p.vel))
        .sum()
}

fn part2(mut planets: &mut Vec<Planet>) -> isize {
    // observations:
    // * the sum of the axis values of each planet is stable.
    // * the system is always fixed around a single point.
    // * if the whole system will repeat, each axis must repeat, probably more frequently than the
    //   whole system? ding, ding! the least common multiple would be the answer?
    let mut steps = 0;

    let start_x = get_xs(&planets);
    let start_y = get_ys(&planets);
    let start_z = get_zs(&planets);

    // track the "period" between repeating axis values
    let mut per_x = 0;
    let mut per_y = 0;
    let mut per_z = 0;

    while per_x == 0 || per_y == 0 || per_z == 0 {
        simulate(&mut planets);
        steps += 1;

        if per_x == 0 && start_x == get_xs(&planets) {
            per_x = steps;
        }

        if per_y == 0 && start_y == get_ys(&planets) {
            per_y = steps;
        }

        if per_z == 0 && start_z == get_zs(&planets) {
            per_z = steps;
        }
    }

    per_x.lcm(&per_y).lcm(&per_z)
}

fn energy(v: &Vec3) -> isize {
    v.x.abs() + v.y.abs() + v.z.abs()
}

fn parse_input(input: &str) -> Vec<Planet> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn simulate(planets: &mut Vec<Planet>) {
    let mut acc = vec![Vec3::new(0, 0, 0); planets.len()];

    for (i, a) in planets.iter().enumerate() {
        for (j, b) in planets.iter().enumerate() {
            if i == j {
                continue;
            }

            acc[i].x += get_change(a.pos.x, b.pos.x);
            acc[i].y += get_change(a.pos.y, b.pos.y);
            acc[i].z += get_change(a.pos.z, b.pos.z);
        }
    }

    for (i, a) in acc.iter().enumerate() {
        let vel = planets[i].vel.add(&a);
        let pos = planets[i].pos.add(&vel);

        planets[i].vel = vel;

        planets[i].pos = pos;
    }
}

fn get_change(a: isize, b: isize) -> isize {
    if a == b {
        0
    } else {
        (b - a) / (b - a).abs()
    }
}

// TODO: gotta be a better way to extract this information...
fn get_xs(planets: &Vec<Planet>) -> (isize, isize, isize, isize, isize, isize, isize, isize) {
    (
        planets[0].pos.x,
        planets[1].pos.x,
        planets[2].pos.x,
        planets[3].pos.x,
        planets[0].vel.x,
        planets[1].vel.x,
        planets[2].vel.x,
        planets[3].vel.x,
    )
}

fn get_ys(planets: &Vec<Planet>) -> (isize, isize, isize, isize, isize, isize, isize, isize) {
    (
        planets[0].pos.y,
        planets[1].pos.y,
        planets[2].pos.y,
        planets[3].pos.y,
        planets[0].vel.y,
        planets[1].vel.y,
        planets[2].vel.y,
        planets[3].vel.y,
    )
}

fn get_zs(planets: &Vec<Planet>) -> (isize, isize, isize, isize, isize, isize, isize, isize) {
    (
        planets[0].pos.z,
        planets[1].pos.z,
        planets[2].pos.z,
        planets[3].pos.z,
        planets[0].vel.z,
        planets[1].vel.z,
        planets[2].vel.z,
        planets[3].vel.z,
    )
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Planet {
    pos: Vec3,
    vel: Vec3,
}

impl FromStr for Planet {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

        let cap = re.captures(input).unwrap();

        Ok(Self {
            pos: Vec3::new(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            ),
            vel: Vec3::new(0, 0, 0),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let planet: Planet = "<x=23, y=-42, z=100>".parse().unwrap();

        assert_eq!(planet.pos, Vec3::new(23, -42, 100));
    }

    #[test]
    fn part1_test() {
        let mut planets = parse_input(
            "\
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        );

        assert_eq!(part1(&mut planets, 100), 1940);
    }

    #[test]
    fn part2_test() {
        let mut planets = parse_input(
            "\
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        );

        assert_eq!(part2(&mut planets), 4686774924);
    }
}
