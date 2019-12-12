use aoc2019::vec3::Vec3;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let planets = parse_input(include_str!("../../input/12.txt").trim());

    let p1 = part1(&mut planets.clone());

    println!("part 1: {}", p1);

    println!("part 2: {}", "incomplete");
}

fn part1(mut planets: &mut Vec<Planet>) -> isize {
    for _ in 0..1000 {
        simulate(&mut planets);
    }

    planets.iter().map(|p| p.pos.mag() + p.vel.mag()).sum()
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

            acc[i].x = get_change(a.pos.x, b.pos.x);
            acc[i].y = get_change(a.pos.y, b.pos.y);
            acc[i].z = get_change(a.pos.z, b.pos.z);
        }
    }

    for (i, a) in acc.iter().enumerate() {
        planets[i].vel.add(&a);
        let vel = planets[i].vel;
        planets[i].pos.add(&vel);
    }
}

fn get_change(a: isize, b: isize) -> isize {
    if a < b {
        -1
    } else if b > a {
        1
    } else {
        0
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
}
