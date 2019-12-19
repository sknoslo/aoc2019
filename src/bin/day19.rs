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

fn part2(program: &Vec<isize>) -> usize {
    let mut comp = Computer::with_queue_io();

    let mut map = Vec::with_capacity(100 * 100);

    for y in 0..100 {
        for x in 0..100 {
            comp.load(&program);

            comp.send(x);
            comp.send(y);

            comp.run();
            let out = comp.read_output().expect("no output!");

            map.push(out);
        }
    }

    let mut line = vec!['.'; 100];

    for y in 0..100 {
        for x in 0..100 {
            line[x] = match map[y * 100 + x] {
                1 => '#',
                _ => '.',
            };
        }

        println!("{}", line.iter().collect::<String>());
    }

    println!("no go do math");

    0
}
