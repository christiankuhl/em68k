//TEMP
// #![allow(unused_variables)]

use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
mod processor;
mod memory;
mod parser;
mod instructions;
use processor::CPU;
use memory::{RamPtr, RAM_SIZE};
mod fields;

pub struct Emulator {
    cpu: CPU,    // CPU
    ram: RamPtr, // Pointer to RAM
}

impl Emulator {
    pub fn run(&mut self, program: &str) {
        self.initialize(program);
        loop {
            self.cpu.clock_cycle();
            self.hardware_update();
        }
    }
    fn initialize(&mut self, progname: &str) {
        let program = fs::read(progname).expect("Program does not exist!");
        let mut raw_mem = self.ram.as_ref().borrow_mut();
        for (j, &b) in program.iter().enumerate() {
            raw_mem[j + 0x400] = b;
        }
        self.cpu.pc = 0x400;
        self.cpu.ssp.replace(0x400);
    }
    fn hardware_update(&mut self) {
        println!("\nHardware output:");
        println!("0x100: {:02x?}", &self.ram.borrow()[0x100..0x108]);
        println!("0x1000: {:02x?}", &self.ram.borrow()[0x1000..0x1004]);
        println!("0x2000: {:02x?}", &self.ram.borrow()[0x2000..0x2004]);
    }
    pub fn new() -> Emulator {
        let ram = RamPtr::new(RefCell::new([0u8; RAM_SIZE]));
        let ar = [
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0x104)),
        ];
        let dr = [
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
        ];
        let ssp = Rc::new(RefCell::new(0));
        let cpu = CPU { pc: 0, sr: 0, dr: dr, ar: ar, ssp: ssp, ram: Rc::clone(&ram) };
        Emulator { cpu: cpu, ram: Rc::clone(&ram) }
    }
}

fn main() {
    let mut em = Emulator::new();
    em.run("examples/strtolower.bin");
}
