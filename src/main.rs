use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
mod instructions;
mod memory;
mod parser;
mod processor;
use memory::{RAM, RAM_SIZE, BusPtr, Bus};
use processor::{CPU, DisassemblySection};
mod conversions;
mod devices;
use devices::{Debugger, DeviceList, Signal, ASMStream, Timer, Monitor, Floppy};
mod fields;
use fields::{EAMode, OpResult};
mod atari;
use atari::*;

pub struct Emulator {
    cpu: CPU,
    bus: BusPtr,
}

impl Emulator {
    pub fn run(&mut self, program: &str) {
        self.initialize(program);
        // let mut detached = Vec::new();
        // let mut attached = DeviceList::new();
        loop {
            match self.cpu.clock_cycle() {
                Signal::Quit => break,
                _ => {}
            }
            {
                // self.bus.borrow_mut().update(&self.cpu);
            }
            // for (j, device) in self.devices.iter_mut().enumerate() {
            //     match device.2.update(&mut self.cpu) {
            //         Signal::Quit => return,
            //         Signal::Attach(mut new_device) => {
            //             // new_device.init(Rc::clone(&self.ram));
            //             // attached.push(new_device)
            //         }
            //         Signal::Detach => {} //detached.push(j),
            //         Signal::Ok | Signal::NoOp=> (),
            //     };
            // }
            // for j in detached.drain(0..) {
                // self.devices.remove(j);
            // }
            // for device in attached.drain(0..) {
                // self.devices.push(device);
            // }
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
        // let ram = RamPtr::new(RefCell::new(vec![0u8; RAM_SIZE]));
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
        Emulator { cpu: cpu, bus: busptr }
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
    // let mut dev = DeviceList::new();
    // dev.push(Debugger::new());
    // dev.push(Timer::new());
    // dev.push(Monitor::new());
    // dev.push(Floppy::new("examples/ST0001 Mono Demos.st"));
    // dev.push(Box::new(ASMStream));
    let mut bus = Bus::new();
    bus.attach(RAM::new(), 0x0, 0xff7fff);
    bus.attach(Monitor::new(), 0xff8000, 0xffffff);
    // bus.attach(Debugger::new(), 0x0, 0x0);
    bus.attach(Box::new(ASMStream), 0x0, 0x0);
    let mut em = Emulator::new(bus);

    // let mut result = String::new();
    // for line in em.disassemble("tos/TOS104GE.IMG").iter() {
    //     let mut out = String::new();
    //     for word in &line.1 {
    //         out.push_str(&format!("{:04x} ", word));
    //     }
    //     result.push_str(&format!(
    //         "{a:08x}  {o:<25} {i:<32}\n",
    //         o = out,
    //         i = line.2,
    //         a = line.0,
    //     ));
    // }
    // fs::write("examples/tos.asm", result);

    // em.run("examples/ballerburg/BALLER.PRG");
    // em.run("examples/abcd.bin");
    // em.run("tests/opcode_tests.bin");
    em.run("tos/TOS104GE.IMG");
}
