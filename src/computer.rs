enum RAM {
    Unloaded,
    Loaded(Vec<isize>),
}

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
fn get_opcode_and_mode_garbage(inst: isize) -> (isize, isize, isize, isize) {
    (
        inst % 100,
        inst / 100 % 10,
        inst / 1000 % 10,
        inst / 10000 % 10,
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

    pub fn read(&self, address: isize) -> isize {
        match self.memory {
            RAM::Unloaded => panic!("No program is loaded"), // TODO: Handle errors?
            RAM::Loaded(ref memory) => memory[address as usize],
        }
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
                (1, _, _, _) => {
                    let a = self.read(self.ip + 1);
                    let b = self.read(self.ip + 2);
                    let target = self.read(self.ip + 3);

                    self.write(target, self.read(a) + self.read(b));
                    self.ip = self.ip + 4;
                }
                (2, _, _, _) => {
                    let a = self.read(self.ip + 1);
                    let b = self.read(self.ip + 2);
                    let target = self.read(self.ip + 3);

                    self.write(target, self.read(a) * self.read(b));
                    self.ip = self.ip + 4;
                }
                (3, _, _, _) => {
                    let input = self.read_input();

                    let target = self.read(self.ip + 1);

                    self.write(target, input);
                    self.ip = self.ip + 2;
                }
                (4, _, _, _) => {
                    let a = self.read(self.ip + 1);

                    self.write_output(self.read(a));
                    self.ip = self.ip + 2;
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
        assert_eq!(get_opcode_and_mode_garbage(1002), (2, 0, 1, 0));
        assert_eq!(get_opcode_and_mode_garbage(11299), (99, 2, 1, 1));
    }
}
