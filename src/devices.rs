use crate::processor::CPU;
use crate::memory::{RamPtr, RAM_SIZE};
use std::io::{stdin, stdout, Read, Write};
use std::cell::RefCell;
use std::rc::Rc;

pub type DeviceList = Vec<Box<dyn Device>>;

pub trait Device {
    fn init(&mut self, ram: RamPtr);
    fn update(&mut self, cpu: &CPU);
}

pub struct Debugger {
    ram: RamPtr,
}

impl Debugger {
    pub fn new() -> Box<Self> {
        Box::new(Debugger { ram: Rc::new(RefCell::new([0; RAM_SIZE])) })
    }
}

impl Device for Debugger {
    fn init(&mut self, ram: RamPtr) {
        self.ram = ram;
    }
    fn update(&mut self, cpu: &CPU) {
        println!("{:?}", cpu);
        println!("Next instruction: {:}", cpu.nxt.as_asm(cpu));
        pause();
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue, CTRL+C to quit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

