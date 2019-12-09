use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::cell::Cell;

pub type Intcode = i64;

pub struct Process {
    pub input: Vec<Intcode>,  // Queue of input values
    pub output: Vec<Intcode>, // Queue of output values

    mem: Vec<Intcode>, // Each process has its own memory
    ip: Cell<usize>,   // Instruction Pointer, keeps track of execution
}

#[derive(PartialEq)]
pub enum Status {
    WaitForInput,
    NewOutput,
    Halt,
}

#[derive(FromPrimitive)]
enum Opcode {
    ADD = 1,   // Addition
    MUL = 2,   // Multiplication
    IN = 3,    // Read input
    OUT = 4,   // Write output
    JIT = 5,   // Jump If True
    JIF = 6,   // Jump If False
    LT = 7,    // Less Than check
    EQ = 8,    // Equal check
    HALT = 99, // End of program
}

// Memory access modes
#[derive(FromPrimitive)]
enum MemMode {
    Address = 0,
    Immediate = 1,
}

impl Process {
    pub fn new(program: &[Intcode]) -> Process {
        Process {
            input: Vec::new(),
            output: Vec::new(),
            mem: program.to_vec(),
            ip: Cell::new(0),
        }
    }

    pub fn exec(&mut self) -> Status {
        loop {
            let intcode = self.mem.get(self.next_ip()).expect("Bad address");
            let opcode = Opcode::from_i64(intcode % 100).expect("Bad Opcode");

            match opcode {
                Opcode::ADD => {
                    let pmodes = Process::get_param_modes(intcode, 2);
                    let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
                    let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
                    self.mem_write(self.next_ip(), p0 + p1);
                }
                Opcode::MUL => {
                    let pmodes = Process::get_param_modes(intcode, 2);
                    let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
                    let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
                    self.mem_write(self.next_ip(), p0 * p1);
                }
                Opcode::IN => {
                    if self.input.is_empty() {
                        return Status::WaitForInput;
                    } else {
                        let input = self.input.remove(0);
                        self.mem_write(self.next_ip(), input);
                    }
                }
                Opcode::OUT => {
                    let pmodes = Process::get_param_modes(intcode, 1);
                    let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
                    self.output.push(p0);
                    return Status::NewOutput;
                }
                Opcode::JIT => {
                    let pmodes = Process::get_param_modes(intcode, 2);
                    let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
                    let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
                    if p0 != 0 {
                        self.ip.set(p1 as usize);
                    }
                }
                Opcode::JIF => {
                    let pmodes = Process::get_param_modes(intcode, 2);
                    let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
                    let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
                    if p0 == 0 {
                        self.ip.set(p1 as usize);
                    }
                }
                Opcode::LT => {
                    let pmodes = Process::get_param_modes(intcode, 2);
                    let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
                    let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
                    if p0 < p1 {
                        self.mem_write(self.next_ip(), 1);
                    } else {
                        self.mem_write(self.next_ip(), 0);
                    }
                }
                Opcode::EQ => {
                    let pmodes = Process::get_param_modes(intcode, 2);
                    let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
                    let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
                    if p0 == p1 {
                        self.mem_write(self.next_ip(), 1);
                    } else {
                        self.mem_write(self.next_ip(), 0);
                    }
                }
                Opcode::HALT => {
                    return Status::Halt;
                }
            }
        }
    }

    fn get_param_modes(intcode: &Intcode, count: u32) -> Vec<MemMode> {
        (0..count)
            .map(|i| {
                let m = (intcode / ((10 as Intcode).pow(i + 2))) % 10;
                MemMode::from_i64(m).expect("Bad MemMode")
            })
            .collect()
    }

    fn mem_read(&self, index: usize, mode: &MemMode) -> Intcode {
        match mode {
            MemMode::Address => {
                let addr = self.mem[index] as usize;
                self.mem[addr]
            }
            MemMode::Immediate => self.mem[index],
        }
    }

    fn mem_write(&mut self, index: usize, value: Intcode) {
        // Writes are always in MemMode::Address mode
        let addr = self.mem[index] as usize;
        self.mem[addr] = value;
    }

    fn next_ip(&self) -> usize {
        let aux = self.ip.get();
        self.ip.set(self.ip.get() + 1);
        aux
    }
}
