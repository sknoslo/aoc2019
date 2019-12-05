#[derive(Debug)]
enum RAM {
    Unloaded,
    Loaded(Vec<isize>),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Position,
    Immediate,
}

impl From<isize> for Mode {
    fn from(i: isize) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("I don't know that mode!"),
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    ip: isize,
    memory: RAM,
    input: Vec<isize>,  // not sure exactly how input works yet.
    output: Vec<isize>, // this probably makes sense for output.
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

impl Computer {
    pub fn new() -> Self {
        Self {
            ip: 0,
            memory: RAM::Unloaded,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn load(&mut self, program: &Vec<isize>) {
        self.ip = 0;
        self.memory = RAM::Loaded(program.clone());
    }

    pub fn write(&mut self, address: isize, value: isize) {
        match self.memory {
            RAM::Unloaded => panic!("No program is loaded"), // TODO: Handle errors?
            RAM::Loaded(ref mut memory) => memory[address as usize] = value,
        }
    }

    pub fn writem(&mut self, address: isize, value: isize, mode: Mode) {
        match mode {
            Mode::Position => {
                let target = self.read(address);
                self.write(target, value);
            }
            Mode::Immediate => {
                self.write(address, value);
            }
        }
    }

    pub fn read(&self, address: isize) -> isize {
        match self.memory {
            RAM::Unloaded => panic!("No program is loaded"), // TODO: Handle errors?
            RAM::Loaded(ref memory) => memory[address as usize],
        }
    }

    pub fn readm(&self, address: isize, mode: Mode) -> isize {
        match mode {
            Mode::Position => self.read(self.read(address)),
            Mode::Immediate => self.read(address),
        }
    }

    pub fn send(&mut self, value: isize) {
        self.input.push(value);
    }

    pub fn receive(&self) -> std::slice::Iter<isize> {
        self.output.iter()
    }

    fn read_input(&mut self) -> isize {
        self.input.pop().expect("no values in input!")
    }

    fn write_output(&mut self, value: isize) {
        self.output.push(value);
    }

    pub fn run(&mut self) {
        loop {
            match get_opcode_and_mode_garbage(self.read(self.ip)) {
                (1, m1, m2, m3) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;
                    let p3 = self.ip + 3;

                    self.writem(p3, self.readm(p1, m1) + self.readm(p2, m2), m3);
                    self.ip = self.ip + 4;
                }
                (2, m1, m2, m3) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;
                    let p3 = self.ip + 3;

                    self.writem(p3, self.readm(p1, m1) * self.readm(p2, m2), m3);
                    self.ip = self.ip + 4;
                }
                (3, m1, _, _) => {
                    let input = self.read_input();

                    let p1 = self.ip + 1;

                    self.writem(p1, input, m1);
                    self.ip = self.ip + 2;
                }
                (4, m1, _, _) => {
                    let p1 = self.ip + 1;

                    self.write_output(self.readm(p1, m1));
                    self.ip = self.ip + 2;
                }
                (5, m1, m2, _) => {
                    let p1 = self.ip + 1;
                    let p2 = self.ip + 2;

                    if self.readm(p1, m1) != 0 {
                        self.ip = self.readm(p2, m2);
                    }
                }
                (99, _, _, _) => break,
                _ => panic!("That shouldn't have happened"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garbage_opcode_thing() {
        assert_eq!(
            get_opcode_and_mode_garbage(1002),
            (2, Mode::Position, Mode::Immediate, Mode::Position)
        );
        assert_eq!(
            get_opcode_and_mode_garbage(10099),
            (99, Mode::Position, Mode::Position, Mode::Immediate)
        );
    }
}
