use aoc2019::computer::io::IoDevice;
use aoc2019::computer::{parse_program, Computer};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    let program = parse_program(include_str!("../../input/23.txt").trim());

    let p1 = part1(&program);
    println!("part 1: {}", p1);

    let p2 = part2(&program);
    println!("part 2: {}", p2);
}

fn part1(program: &Vec<isize>) -> isize {
    let num_computers = 50;
    let mut computers = Vec::new();
    let mut cables = Vec::new();

    let mut message_router = vec![VecDeque::new(); num_computers];

    for addr in 0..num_computers {
        let mut cable = Cable::new();
        cable.register(addr);

        let cable = Rc::new(RefCell::new(cable));

        let mut comp = Computer::new(Some(cable.clone()), Some(cable.clone()));
        comp.load(&program);

        cables.push(cable.clone());
        computers.push(comp);
    }

    loop {
        for addr in 0..num_computers {
            computers[addr].step();

            let mut cable = cables[addr].borrow_mut();

            if let Some(message) = cable.recv() {
                if message.addr == 255 {
                    // puzzle solution
                    return message.y;
                }
                message_router[message.addr].push_front(message);
            }

            if let Some(message) = message_router[addr].pop_back() {
                cable.send(message);
            }
        }
    }
}

fn part2(program: &Vec<isize>) -> isize {
    let num_computers = 50;
    let mut computers = Vec::new();
    let mut cables = Vec::new();

    let mut nat_message = None;
    let mut last_nat_y = None;
    let mut total_messages = 0;

    let mut message_router = vec![VecDeque::new(); num_computers];

    for addr in 0..num_computers {
        let mut cable = Cable::new();
        cable.register(addr);

        let cable = Rc::new(RefCell::new(cable));

        let mut comp = Computer::new(Some(cable.clone()), Some(cable.clone()));
        comp.load(&program);

        cables.push(cable.clone());
        computers.push(comp);
    }

    loop {
        let mut total_idle = 0;

        for addr in 0..num_computers {
            computers[addr].step();

            let mut cable = cables[addr].borrow_mut();

            if let Some(message) = cable.recv() {
                if message.addr == 255 {
                    nat_message = Some(message);
                } else {
                    total_messages += 1;
                    message_router[message.addr].push_front(message);
                }
            }

            if let Some(message) = message_router[addr].pop_back() {
                total_messages -= 1;
                cable.send(message);
            }

            if cable.idle {
                total_idle += 1;
            }
        }

        if let Some(message) = nat_message {
            // no messages queued, send the nat message to addr 0
            if total_messages == 0 && total_idle == num_computers {
                if let Some(y) = last_nat_y {
                    // puzzle answer
                    if y == message.y {
                        return y;
                    }
                }

                total_messages = 1;
                last_nat_y = Some(message.y);
                nat_message = None;
                message_router[0].push_front(message);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Packet {
    addr: usize,
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Cable {
    idle: bool,
    in_queue: VecDeque<isize>,
    out_queue: VecDeque<isize>,
}

impl Cable {
    fn new() -> Self {
        Self {
            idle: true,
            in_queue: VecDeque::new(),
            out_queue: VecDeque::new(),
        }
    }

    fn register(&mut self, addr: usize) {
        self.in_queue.push_front(addr as isize);
    }

    fn send(&mut self, packet: Packet) {
        self.in_queue.push_front(packet.x);
        self.in_queue.push_front(packet.y);
    }

    fn recv(&mut self) -> Option<Packet> {
        if self.out_queue.len() < 3 {
            return None;
        }

        Some(Packet {
            addr: self.out_queue.pop_back().unwrap() as usize,
            x: self.out_queue.pop_back().unwrap(),
            y: self.out_queue.pop_back().unwrap(),
        })
    }
}

impl IoDevice for Cable {
    fn read(&mut self) -> Option<isize> {
        if let Some(value) = self.in_queue.pop_back() {
            self.idle = false;
            Some(value)
        } else {
            self.idle = true;

            Some(-1)
        }
    }

    fn write(&mut self, value: isize) {
        self.idle = false;
        self.out_queue.push_front(value);
    }
}
