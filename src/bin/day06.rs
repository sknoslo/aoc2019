use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/06.txt").trim();

    let input = parse_input(input);

    let orbit_map = build_map(&input);

    let p1 = part1(&orbit_map);

    println!("part 1: {}", p1);

    let reverse_map = build_reverse_map(&input);

    let p2 = part2(&reverse_map);

    println!("part 2: {}", p2);
}

fn part1(map: &HashMap<String, Vec<String>>) -> isize {
    // Build a tree with root `COM`. The checksum is calculated by simply summing the depths of
    // the nodes in the tree.
    let mut stack = Vec::new();
    let mut checksum = 0;

    stack.push((0, "COM".to_string()));

    while let Some((depth, ref obj_name)) = stack.pop() {
        checksum += depth;

        if let Some(ref orbiters) = map.get(obj_name) {
            for orbiter in orbiters.iter() {
                stack.push((depth + 1, orbiter.to_string()));
            }
        }
    }

    checksum
}

fn part2(reverse_map: &HashMap<String, String>) -> isize {
    // To find a path from A to B in a tree, we can start at A and go up to the root, tracing the
    // path, and then start at B and go up to the intersection of the path from A to root, and add
    // up the total steps.
    let mut path: HashMap<String, isize> = HashMap::new();

    let mut obj_name = "SAN".to_string();
    let mut transfers = 0;

    while let Some(next_obj_name) = reverse_map.get(&obj_name) {
        path.insert(next_obj_name.to_string(), transfers);

        transfers += 1;

        obj_name = next_obj_name.to_string();
    }

    let mut obj_name = "YOU".to_string();
    let mut transfers = 0;

    while let Some(next_obj_name) = reverse_map.get(&obj_name) {
        if let Some(dist_to_san) = path.get(next_obj_name) {
            return transfers + dist_to_san;
        } else {
            obj_name = next_obj_name.to_string();
            transfers += 1
        }
    }

    panic!("No path to santa. This is probably not true and you just made a mistake.");
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

fn build_map(input: &Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut orbit_map = HashMap::new();

    for (orbitee, orbiter) in input {
        let orbiters = orbit_map.entry(orbitee.to_string()).or_insert(vec![]);

        orbiters.push(orbiter.to_string());
    }

    orbit_map
}

fn build_reverse_map(input: &Vec<(String, String)>) -> HashMap<String, String> {
    input
        .iter()
        .map(|(orbitee, orbiter)| (orbiter.to_string(), orbitee.to_string()))
        .collect()
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
        let map = build_map(&input);

        assert_eq!(part1(&map), 42);
    }

    #[test]
    fn example_part2_test() {
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
K)L
K)YOU
I)SAN";
        let input = parse_input(input);
        let map = build_reverse_map(&input);

        assert_eq!(part2(&map), 4);
    }
}
