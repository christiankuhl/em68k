use std::rc::Rc;
mod parser;
mod instructions;
use parser::parse_instruction;

const RAM_SIZE: usize = 1 << 20;

pub struct CPU {
    pc: u32,                    // Program counter
    ccr: u8,                    // Condition code register
    dr: [u32; 8],               // Data registers
    ar: [u32; 8],               // Address registers
    ram: Rc<[u8; RAM_SIZE]>     // RAM
}

pub struct Machine {
    cpu: CPU,                 // CPU
    ram: Rc<[u8; RAM_SIZE]>   // RAM
}

impl Machine {
    pub fn run(&mut self, program: &str) {
        self.initialize(program);
        loop {
            self.cpu.clock_cycle();
            self.hardware_update();
        }
    }
    fn initialize(&mut self, program: &str) {}
    fn hardware_update(&mut self) {}
    pub fn new() -> Machine {
        let ram = Rc::new([0u8; RAM_SIZE]);
        let cpu = CPU { pc: 0, ccr: 0, dr: [0u32; 8], ar: [0u32; 8], ram: Rc::clone(&ram) };
        Machine { cpu: cpu, ram: Rc::clone(&ram) }
    }
}

impl CPU {
    pub fn clock_cycle(&mut self) {
        let opcode = self.read_word();
        if let Some(instruction) = parser::parse_instruction(opcode) {
            instruction.execute(self);
        } else {
            panic!("Illegal instruction!");
        }
    }
    fn read_word(&self) -> u16 {
        u16::from_be_bytes([self.ram[self.pc as usize], self.ram[self.pc as usize + 2]])
    }
}

fn main() {

}
