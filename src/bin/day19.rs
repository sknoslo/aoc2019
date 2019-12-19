use aoc2019::computer::{parse_program, Computer};

fn main() {
    let program = parse_program(include_str!("../../input/19.txt").trim());

    let p1 = part1(&program);
    println!("part 1: {}", p1);

    let p2 = part2(&program);
    println!("part 2: {}", p2);
}

fn part1(program: &Vec<isize>) -> usize {
    let mut comp = Computer::with_queue_io();

    let mut affected = 0;

    for y in 0..50 {
        for x in 0..50 {
            comp.load(&program);

            comp.send(x);
            comp.send(y);

            comp.run();
            let out = comp.read_output().expect("no output!");

            if out == 1 {
                affected += 1;
            }
        }
    }

    affected
}

fn part2(program: &Vec<isize>) -> isize {
    let mut comp = Computer::with_queue_io();

    // algo:
    // * init
    //   * starting at x = 99 look for the top-right corner of a box (can't be less than 99 so
    //     start there).
    // * loop
    //   * go down 1 (y+1) and search right until the end of the beam.
    //   * check (x-99, y+99) if there is a beam, we have our answer.

    let mut top_right = (99, 0);

    loop {
        comp.load(&program);

        comp.send(top_right.0);
        comp.send(top_right.1);

        comp.run();

        if comp.read_output().expect("no output!") == 1 {
            break;
        }

        top_right.1 += 1;
    }

    loop {
        loop {
            top_right.0 += 1;

            comp.load(&program);

            comp.send(top_right.0);
            comp.send(top_right.1);

            comp.run();
            if comp.read_output().expect("no output!") == 0 {
                top_right.0 -= 1;
                break;
            }
        }

        comp.load(&program);

        let bottom_left = (top_right.0 - 99, top_right.1 + 99);

        comp.send(bottom_left.0);
        comp.send(bottom_left.1);

        comp.run();
        if comp.read_output().expect("no output!") == 1 {
            return 10_000 * bottom_left.0 + top_right.1;
        }

        top_right.1 += 1;
    }
}
