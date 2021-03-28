use crate::memory::{RamPtr, RAM_SIZE};
use crate::processor::{Disassembly, CPU};
use std::cell::RefCell;
use std::io::{stdin, stdout};
use std::rc::Rc;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub type DeviceList = Vec<Box<dyn Device>>;

pub enum Signal {
    Ok,
    Quit,
    Attach(Box<dyn Device>),
    Detach,
}

pub trait Device {
    fn init(&mut self, ram: RamPtr);
    fn update(&mut self, cpu: &mut CPU) -> Signal;
}

pub struct Debugger {
    ram: RamPtr,
    disassembly: Disassembly,
}

impl Debugger {
    pub fn new() -> Box<Self> {
        Box::new(Debugger { ram: Rc::new(RefCell::new(vec![0; RAM_SIZE])), disassembly: Disassembly::new(12) })
    }
    fn set_breakpoint(&mut self) -> Signal {
        let stdin = stdin();
        let mut addr = String::new();
        println!("Enter breakpoint address: ");
        stdin.read_line(&mut addr).expect("Error reading breakpoint!");
        Signal::Ok
    }
}

impl Device for Debugger {
    fn init(&mut self, ram: RamPtr) {
        self.ram = ram;
    }
    fn update(&mut self, cpu: &mut CPU) -> Signal {
        self.disassembly.update(cpu);
        print!("{c}{tl}{cpu}", c = clear::All, tl = cursor::Goto(1, 1), cpu = cpu);
        print!("{tr}{dis}", tr = cursor::Goto(10, 10), dis = self.disassembly);
        print!("{r} Next instruction: {n}", r = cursor::Goto(36, 3), n = cpu.nxt.as_asm(cpu));
        println!("{r}\nDebugger attached. Press space to single step, c to continue, b to enter a breakpoint or q to quit.", 
            r = cursor::Goto(36, 6 + self.disassembly.length as u16));
        let stdin = stdin();
        let mut _stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char(' ')) => break,
                Event::Key(Key::Char('q')) => return Signal::Quit,
                Event::Key(Key::Char('b')) => return self.set_breakpoint(),
                Event::Mouse(me) => match me {
                    MouseEvent::Press(_, x, y) => {
                        println!("{}x", termion::cursor::Goto(x, y));
                    }
                    _ => (),
                },
                _ => {}
            }
        }
        println!("{}", clear::All);
        Signal::Ok
    }
}

pub struct ASMStream;

impl Device for ASMStream {
    fn init(&mut self, _ram: RamPtr) {}
    fn update(&mut self, cpu: &mut CPU) -> Signal {
        let mut dis = cpu.disassemble(1);
        for mut instr in dis.drain(..) {
            let mut hex = String::new();
            for opcode in instr.1.drain(..) {
                hex.push_str(&format!(" {:04x}", opcode))
            }
            println!("{:08x}:{:<30} {}", instr.0, hex, instr.2);
        }
        Signal::Ok
    }
}