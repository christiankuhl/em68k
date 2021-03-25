use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
mod instructions;
mod memory;
mod parser;
mod processor;
use memory::{RamPtr, RAM_SIZE};
use processor::CPU;
mod conversions;
mod fields;
mod devices;
use devices::{DeviceList, Debugger};

pub struct Emulator {
    cpu: CPU,
    ram: RamPtr,
    devices: DeviceList,
}

impl Emulator {
    pub fn run(&mut self, program: &str) {
        self.initialize(program);
        loop {
            self.cpu.clock_cycle();
            for device in self.devices.iter_mut() {
                device.update(&mut self.cpu);
            }
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
    pub fn new(mut devices: DeviceList) -> Emulator {
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
        let cpu = CPU::new(0, 0, dr, ar, ssp, Rc::clone(&ram));
        for device in devices.iter_mut() {
            device.init(Rc::clone(&ram));
        }
        Emulator { cpu: cpu, ram: Rc::clone(&ram), devices: devices }
    }
}

fn main() {    
    let mut dev = DeviceList::new();
    dev.push(Debugger::new());
    let mut em = Emulator::new(dev);
    em.run("examples/strtolower.bin");
}
