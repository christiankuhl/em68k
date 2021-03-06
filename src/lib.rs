use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
mod instructions;
pub mod memory;
mod parser;
pub mod processor;
use memory::Bus;
use processor::{CPU, Debugger};
mod conversions;
pub mod devices;
use devices::Signal;
pub mod fields;
use fields::{EAMode, OpResult};
pub mod atari;

pub struct Configuration {
    pub base_address: u32,
    pub start_address: u32,
    pub initial_ssp: u32,
    pub bus: Bus,
    pub memory_layout: Vec<(usize, OpResult)>
}

pub struct Emulator {
    cpu: CPU,
    base_address: usize,
}

impl Emulator {
    pub fn run(&mut self, program: &str, debug: bool) {
        self.load(program);
        let mut debugger = if debug { Some(Debugger::new()) } else { None };
        let mut idle = false;
        loop {
            if !idle {
                match self.cpu.clock_cycle() {
                    Signal::Quit => break,
                    _ => {}
                }
                if !debug {
                    self.cpu.serve_interrupt_requests();
                }
            } else {
                idle = false;
            }
            if debug {
                match debugger.as_mut().unwrap().update(&mut self.cpu) {
                    Signal::Quit => return,
                    Signal::NoOp => {
                        idle = true;
                    }
                    _ => (),
                };
            }
            match self.cpu.poll_devices() {
                Signal::Quit => break,
                _ => (),
            }
        }
    }
    fn load(&mut self, progname: &str) {
        let program = fs::read(progname).expect("Program does not exist!");
        for (j, &b) in program.iter().enumerate() {
            self.cpu.bus.borrow_mut().write(j + self.base_address, OpResult::Byte(b));
        }
    }
    pub fn new(config: Configuration) -> Emulator {
        let ar = [
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)),
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
        let busptr = Rc::new(RefCell::new(config.bus));
        let mut cpu = CPU::new(0, 0, dr, ar, ssp, Rc::clone(&busptr));
        cpu.pc = config.start_address;
        cpu.ssp.replace(config.initial_ssp);
        cpu.supervisor_mode(true);
        for (addr, val) in config.memory_layout {
            let handle = cpu.memory_handle(EAMode::AbsoluteLong(addr));
            handle.write(val);
        }
        Emulator { cpu: cpu, base_address: config.base_address as usize }
    }
}