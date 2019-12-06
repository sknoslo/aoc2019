use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/06.txt").trim();

    let input = parse_input(input);

    let orbit_map = build_map(input);

    let p1 = part1(&orbit_map);

    println!("part 1: {}", p1);
}

fn part1(map: &HashMap<String, Vec<String>>) -> isize {
    let mut stack: Vec<(isize, String)> = Vec::new();
    let mut checksum = 0;

    stack.push((0, "COM".to_string()));

    while let Some((depth, obj_name)) = stack.pop() {
        checksum += depth;

        if let Some(orbiters) = map.get(&obj_name) {
            for orbiter in orbiters.iter() {
                stack.push((depth + 1, orbiter.to_string()));
            }
        }
    }

    checksum
}

fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(")").collect();
            (
                parts.get(0).expect("Bad input").to_string(),
                parts.get(1).expect("Bad input").to_string(),
            )
        })
        .collect()
}

fn build_map(input: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut orbit_map = HashMap::new();

    for (orbitee, orbiter) in input {
        let orbiters = orbit_map.entry(orbitee).or_insert(vec![]);

        orbiters.push(orbiter);
    }

    orbit_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1_test() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let input = parse_input(input);
        let map = build_map(input);

        assert_eq!(part1(&map), 42);
    }
}
