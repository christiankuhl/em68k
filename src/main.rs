use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
mod instructions;
mod memory;
mod parser;
mod processor;
use memory::{RAM, Bus};
use processor::{CPU, Debugger, DisassemblySection};
mod conversions;
mod devices;
use devices::{Signal, Timer, Monitor};
mod fields;
use fields::{EAMode, OpResult};
mod atari;
use atari::*;

pub struct Emulator {
    cpu: CPU,
}

impl Emulator {
    pub fn run(&mut self, program: &str) {
        self.initialize(program);
        let mut debugger = Debugger::new();
        let mut idle = false;
        loop {
            if !idle {
                match self.cpu.clock_cycle() {
                    Signal::Quit => break,
                    _ => {}
                }
            } else {
                idle = false;
            }
            match debugger.update(&mut self.cpu) {
                Signal::Quit => return,
                Signal::Attach(_) => {}
                Signal::Detach => {}
                Signal::NoOp => {
                    idle = true;
                }
                Signal::Ok => (),
            };
            
        }
    }
    fn initialize(&mut self, progname: &str) {
        let program = fs::read(progname).expect("Program does not exist!");
        {
            for (j, &b) in program.iter().enumerate() {
                self.cpu.bus.borrow_mut().write(j + BASE_ADDRESS as usize, OpResult::Byte(b));
            }
        }
        self.cpu.pc = START_ADDRESS;
        self.cpu.ssp.replace(0x3f0);
        self.cpu.supervisor_mode(true);
        for (addr, val) in MEMORY_LAYOUT.iter() {
            let handle = self.cpu.memory_handle(EAMode::AbsoluteLong(*addr));
            handle.write(*val);
        }
    }
    pub fn new(bus: Bus) -> Emulator {
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
        let busptr = Rc::new(RefCell::new(bus));
        let cpu = CPU::new(0, 0, dr, ar, ssp, Rc::clone(&busptr));
        Emulator { cpu: cpu }
    }
    fn disassemble(&mut self, progname: &str) -> DisassemblySection {
        let program = fs::read(progname).expect("Program does not exist!");
        self.initialize(progname);
        self.cpu.clock_cycle();
        println!("{}", program.len() / 2);
        self.cpu.disassemble(800)
    }
}

fn main() {
    let mut bus = Bus::new();
    bus.attach(Monitor::new(), vec![(0x38000, 0xbfd00)]);
    bus.attach(RAM::new(), vec![(0x0, 0xff7fff)]);
    // Timer A
    bus.attach(Timer::new(0xfffffa19, 0, 2457600.0), vec![(0xfffffa1f, 0xfffffa20), (0xfffffa19, 0xfffffa1a)]);
    // Timer B
    bus.attach(Timer::new(0xfffffa1b, 0, 50.0), vec![(0xfffffa1b, 0xfffffa1c), (0xfffffa21, 0xfffffa22)]);
    // Timer C 
    bus.attach(Timer::new(0xfffffa1d, 4, 200.0), vec![(0xfffffa1d, 0xfffffa1e), (0xfffffa23, 0xfffffa24)]);
    // Timer D
    bus.attach(Timer::new(0xfffffa1d, 0, 2457600.0), vec![(0xfffffa1d, 0xfffffa1e), (0xfffffa25, 0xfffffa26)]);
    let mut em = Emulator::new(bus);
    em.run("tos/TOS104GE.IMG");
}
