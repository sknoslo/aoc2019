use aoc2019::vec3::Vec3;
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
    // let mut seen = HashSet::new();
    let mut steps = 0;
    let mut last = 0;

    for _ in 0..100 {
        steps += 1;
        simulate(&mut planets);

        let sum: isize = planets
            .iter()
            .map(|p| energy(&p.pos) * energy(&p.vel))
            .sum();

        println!("{} => diff: {}", sum, sum - last);
        last = sum;
    }

    steps
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
        planets[i].vel.add(&a);
        let vel = planets[i].vel;
        planets[i].pos.add(&vel);
    }
}

fn get_change(a: isize, b: isize) -> isize {
    if a == b {
        0
    } else {
        (b - a) / (b - a).abs()
    }
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
        // <x={}, y={}, z={}>
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
}
