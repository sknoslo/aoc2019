use std::rc::Rc;

#[derive(Debug)]
enum RAM {
    Unloaded,
    Loaded(Vec<isize>),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<isize> for Mode {
    fn from(i: isize) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("I don't know that mode!"),
        }
    }
}

pub fn parse_program(input: &str) -> Vec<isize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

#[derive(Debug)]
pub struct Computer<T: io::IoDevice + std::fmt::Debug, R: io::IoDevice + std::fmt::Debug> {
    ip: isize,
    relative_base: isize,
    memory: RAM,
    input: Rc<T>, // VecDeque<isize>,
    output: Rc<R>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ExecutionResult {
    Paused,
    Halted,
}

mod io {
    use std::cell::RefCell;

    pub trait IoDevice {
        /**
         * Read a value from an io device
         */
        fn read(&self) -> Option<isize>;

        /**
         * Write a value to an io device
         */
        fn write(&self, value: isize);
    }

    #[derive(Debug)]
    pub struct QueuedIoDevice {
        queue: RefCell<std::collections::VecDeque<isize>>,
    }

    impl QueuedIoDevice {
        pub fn new() -> Self {
            QueuedIoDevice {
                queue: RefCell::new(std::collections::VecDeque::new()),
            }
        }
    }

    impl IoDevice for QueuedIoDevice {
        fn read(&self) -> Option<isize> {
            self.queue.borrow_mut().pop_back()
        }

        fn write(&self, value: isize) {
            self.queue.borrow_mut().push_front(value);
        }
    }
}

// This function is stupid, but I don't know enough about the problem yet to make it better.
// the problem is that it needs to be an N-tuple were N is the max number of params an opcode
// will ever take. N is probably 3, but who really knows...
//
// values are read right-to-left.
fn get_opcode_and_mode_garbage(inst: isize) -> (isize, Mode, Mode, Mode) {
    (
        inst % 100,
        Mode::from(inst / 100 % 10),
        Mode::from(inst / 1000 % 10),
        Mode::from(inst / 10000 % 10),
    )
}

impl Computer<io::QueuedIoDevice, io::QueuedIoDevice> {
    pub fn new() -> Self {
        Self {
            ip: 0,
            relative_base: 0,
            memory: RAM::Unloaded,
            input: Rc::new(io::QueuedIoDevice::new()),
            output: Rc::new(io::QueuedIoDevice::new()),
        }
    }
}

impl<T: io::IoDevice + std::fmt::Debug, R: io::IoDevice + std::fmt::Debug> Computer<T, R> {
    pub fn load(&mut self, program: &Vec<isize>) {
        // intcode can write values past the end of the initial program memory, so start with a
        // vector 10x the length and see what happens. May need to include a fn to extend the
        // memory with more zeros again, or just increase the initial size.
        let mut memory = Vec::with_capacity(program.len() * 10);

        memory.extend(program.iter().cloned());
        memory.extend(vec![0; program.len() * 9].into_iter());

        self.ip = 0;
        self.memory = RAM::Loaded(memory);
    }

    pub fn write(&mut self, address: isize, value: isize) {
        match self.memory {
            RAM::Unloaded => panic!("No program is loaded"), // TODO: Handle errors?
            RAM::Loaded(ref mut memory) => memory[address as usize] = value,
        }
    }

    fn writem(&mut self, address: isize, value: isize, mode: Mode) {
        match mode {
            Mode::Position => {
                let target = self.read(address);

                self.write(target, value);
            }
            Mode::Immediate => {
                self.write(address, value);
            }
            Mode::Relative => {
                let offset = self.read(address);

                self.write(self.relative_base + offset, value);
            }
        }
    }

    pub fn read(&self, address: isize) -> isize {
        match self.memory {
            RAM::Unloaded => panic!("No program is loaded"), // TODO: Handle errors?
            RAM::Loaded(ref memory) => memory[address as usize],
        }
    }

    fn readm(&self, address: isize, mode: Mode) -> isize {
        match mode {
            Mode::Position => self.read(self.read(address)),
            Mode::Immediate => self.read(address),
            Mode::Relative => self.read(self.read(address) + self.relative_base),
        }
    }

    // TODO: Shouldn't need this, should just require external input/output ownership and write to those
    // directly.
    pub fn send(&self, value: isize) {
        self.input.write(value);
    }

    // TODO: Shouldn't need this, should just require external input/output ownership and write to those
    // directly.
    pub fn read_output(&self) -> Option<isize> {
        self.output.read()
    }

    // TODO: These methods are probably not helpful anymore
    fn read_input(&self) -> Option<isize> {
        self.input.read()
    }

    // TODO: These methods are probably not helpful anymore
    fn write_output(&mut self, value: isize) {
        self.output.write(value);
    }

    pub fn run(&mut self) -> ExecutionResult {
        loop {
            match get_opcode_and_mode_garbage(self.read(self.ip)) {
                (1, m1, m2, m3) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;
                    let p3 = self.ip + 3;

                    self.writem(p3, self.readm(p1, m1) + self.readm(p2, m2), m3);
                    self.ip += 4;
                }
                (2, m1, m2, m3) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;
                    let p3 = self.ip + 3;

                    self.writem(p3, self.readm(p1, m1) * self.readm(p2, m2), m3);
                    self.ip += 4;
                }
                (3, m1, _, _) => {
                    if let Some(input) = self.read_input() {
                        let p1 = self.ip + 1;

                        self.writem(p1, input, m1);
                        self.ip += 2;
                    } else {
                        return ExecutionResult::Paused;
                    }
                }
                (4, m1, _, _) => {
                    let p1 = self.ip + 1;

                    self.write_output(self.readm(p1, m1));
                    self.ip += 2;
                }
                (5, m1, m2, _) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;

                    if self.readm(p1, m1) != 0 {
                        self.ip = self.readm(p2, m2);
                    } else {
                        self.ip += 3;
                    }
                }
                (6, m1, m2, _) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;

                    if self.readm(p1, m1) == 0 {
                        self.ip = self.readm(p2, m2);
                    } else {
                        self.ip += 3;
                    }
                }
                (7, m1, m2, m3) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;
                    let p3 = self.ip + 3;

                    let result = self.readm(p1, m1) < self.readm(p2, m2);

                    self.writem(p3, result as isize, m3);
                    self.ip += 4;
                }
                (8, m1, m2, m3) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;
                    let p3 = self.ip + 3;

                    let result = self.readm(p1, m1) == self.readm(p2, m2);

                    self.writem(p3, result as isize, m3);
                    self.ip += 4;
                }
                (9, m1, _, _) => {
                    let p1 = self.ip + 1;

                    self.relative_base += self.readm(p1, m1);
                    self.ip += 2;
                }
                (99, _, _, _) => {
                    return ExecutionResult::Halted;
                }
                (code, m1, m2, m3) => {
                    println!("{}, {:?}, {:?}, {:?}", code, m1, m2, m3);
                    panic!("That shouldn't have happened");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_intcode_test1() {
        let program = parse_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");

        let mut comp = Computer::new();
        comp.load(&program);
        comp.run();

        let mut output = Vec::new();
        while let Some(v) = comp.read_output() {
            output.push(v);
        }
        assert_eq!(output, program);
    }

    #[test]
    fn day9_intcode_test2() {
        let program = parse_program("1102,34915192,34915192,7,4,7,99,0");

        let mut comp = Computer::new();
        comp.load(&program);
        comp.run();

        // should output a 16 digit number
        assert_eq!(format!("{}", comp.read_output().unwrap_or(0)).len(), 16);
    }

    #[test]
    fn day9_intcode_test3() {
        let program = parse_program("104,1125899906842624,99");

        let mut comp = Computer::new();
        comp.load(&program);
        comp.run();

        // should output the middle number
        assert_eq!(comp.read_output().unwrap_or(0), 1125899906842624);
    }
}
