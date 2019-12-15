use aoc2019::computer::{parse_program, Computer};

fn main() {
    println!("part 1: {}", "incomplete");
    println!("part 2: {}", "incomplete");
}

fn part1(program: &Vec<isize>) -> usize {
    // algo:
    // * map the area
    //   * maintain two stacks
    //     * one is the _next_ spot to check, (currentx, currenty, direction_to_go)?
    //     * the other is the path you've taken to get here (prevx, prevy, reverse_direction)?
    //   * go until the first stack is empty
    //   * search forward, once youve reached a deadend backtrack until you've reached the next
    //     point on the first stack?
    //   * after entire area mapped, do BFS on map to find shortest path length to the goal
}
