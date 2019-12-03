pub struct Computer {
    ip: usize,
    memory: Vec<usize>,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            ip: 0,
            memory: Vec::new(),
        }
    }

    pub fn load(&mut self, program: &Vec<usize>) {
        self.ip = 0;
        self.memory = program.clone();
    }

    pub fn write(&mut self, address: usize, value: usize) {
        self.memory[address] = value;
    }

    pub fn read(&self, address: usize) -> usize {
        self.memory[address]
    }

    pub fn run(&mut self) {
        loop {
            match self.memory[self.ip] {
                1 => {
                    let a = self.memory[self.ip + 1];
                    let b = self.memory[self.ip + 2];
                    let target = self.memory[self.ip + 3];

                    self.memory[target] = self.memory[a] + self.memory[b];
                    self.ip = self.ip + 4;
                }
                2 => {
                    let a = self.memory[self.ip + 1];
                    let b = self.memory[self.ip + 2];
                    let target = self.memory[self.ip + 3];

                    self.memory[target] = self.memory[a] * self.memory[b];
                    self.ip = self.ip + 4;
                }
                99 => break,
                _ => panic!("That shouldn't have happened"),
            }
        }
    }
}
