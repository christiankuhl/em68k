use crate::memory::{BusPtr, RAM_SIZE};
use crate::fields::{OpResult, Size};
use crate::processor::{Disassembly, CPU, get_bit};
use std::cell::RefCell;
use std::mem::discriminant;
use std::io::{Stdin, Stdout, stdin, stdout};
use std::rc::Rc;
use std::time::{Instant, Duration};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor};
use minifb::{Window, WindowOptions};
use std::fs;

pub type DeviceList = Vec<(usize, usize, Box<dyn Device>)>;

pub enum Signal {
    Ok,
    Quit,
    Attach(Box<dyn Device>),
    Detach,
    NoOp,
}

pub trait Device: {
    // fn init(&mut self, bus: BusPtr);
    fn update(&mut self, cpu: &CPU) -> Signal;
    fn read(&self, address: usize, size: Size) -> OpResult;
    fn write(&mut self, address: usize, result: OpResult);
}

pub struct Debugger {
    // ram: RamPtr,
    disassembly: Disassembly,
    stdin: Stdin,
    stdout: MouseTerminal<RawTerminal<Stdout>>,
    code_running: bool,
}

impl Debugger {
    pub fn new() -> Box<Self> {
        let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
        Box::new(Debugger { 
            // ram: Rc::new(RefCell::new(vec![0; RAM_SIZE])), 
            disassembly: Disassembly::new(12),
            stdin: stdin(),
            stdout: stdout,
            code_running: false,
        })
    }
    fn set_breakpoint(&mut self, cpu: &CPU) -> Signal {
        let addr;
        {
            println!("Enter breakpoint address: ");
            self.stdout.suspend_raw_mode().expect("Error exiting raw mode!");
            let mut stdin = self.stdin.lock();
            addr = stdin.read_line().unwrap().expect("Error reading breakpoint!");
        }
        match u32::from_str_radix(&addr, 16) {
            Ok(address) => {
                self.disassembly.breakpoints.insert(address);
                self.draw_user_interface(cpu);
                println!("Breakpoint created.");
            },
            Err(_) => {
                self.draw_user_interface(cpu);
                println!("Invalid address!");
            }
        }
        Signal::NoOp        
    }
    fn get_command(&mut self) -> DebugCommand {
        self.stdout.activate_raw_mode().expect("Error entering raw mode!");
        let stdin = self.stdin.lock();
        for c in stdin.events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Char(' ')) => return DebugCommand::Step,
                Event::Key(Key::Char('q')) => return DebugCommand::Quit,
                Event::Key(Key::Char('b')) => return DebugCommand::SetBreakpoint,
                Event::Key(Key::Char('d')) => return DebugCommand::DeleteBreakpoint,
                Event::Key(Key::Char('c')) => return DebugCommand::Continue,
                _ => return DebugCommand::None,
            }
        }
        DebugCommand::None
    }
    fn draw_user_interface(&mut self, cpu: &CPU) {
        println!("{}", clear::All);
        print!("{c}{tl}{cpu}", c = clear::All, tl = cursor::Goto(1, 1), cpu = cpu);
        print!("{tr}{dis}", tr = cursor::Goto(10, 10), dis = self.disassembly);
        print!("{r} Next instruction: {n}", r = cursor::Goto(36, 3), n = cpu.nxt.as_asm(cpu));
        println!("{r}\nDebugger attached. Press space to single step, c to continue, b/d to enter/delete a breakpoint or q to quit.", 
            r = cursor::Goto(1, 6 + self.disassembly.length as u16));
        println!("{}", self.code_running);
    }
}

impl Device for Debugger {
    // fn init(&mut self, ram: RamPtr) {
    //     self.ram = ram;
    // }
    fn update(&mut self, cpu: &CPU) -> Signal {
        if !self.code_running || self.disassembly.breakpoints.contains(&cpu.jmp) {
            self.code_running = false;
            self.disassembly.update(cpu);
            self.draw_user_interface(cpu);
            match self.get_command() {
                DebugCommand::Quit => Signal::Quit,
                DebugCommand::SetBreakpoint => {
                    self.set_breakpoint(cpu)
                },
                DebugCommand::Continue => {
                    self.code_running = true;
                    Signal::Ok
                }
                _ => Signal::NoOp
            }
        } else {
            Signal::Ok
        }
    }
    fn read(&self, address: usize, size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) { }
}

pub struct ASMStream;

impl Device for ASMStream {
    // fn init(&mut self, _ram: RamPtr) {}
    fn update(&mut self, cpu: &CPU) -> Signal {
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
    fn read(&self, address: usize, size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) { }
}

#[derive(PartialEq)]
enum DebugCommand {
    Quit,
    SetBreakpoint,
    DeleteBreakpoint,
    Continue,
    Step,
    None,
}

impl PartialEq for Signal {
    fn eq(&self, other: &Signal) -> bool {
        discriminant(&self) == discriminant(&other)
    }
}

impl Signal {
    pub fn add(&mut self, rhs: &Self) {
        match rhs {
            Self::Quit => *self = Self::Quit,
            Self::NoOp => match *self {
                Self::Quit => *self = Self::Quit,
                _ => *self = Self::NoOp,
            }
            _ => {}
        }
    }
}

pub struct Timer {
    now: Instant
}

impl Timer {
    pub fn new() -> Box<Self> {
        Box::new(Self { now: Instant::now() })
    }
}

impl Device for Timer {
    // fn init(&mut self, _ram: RamPtr) {
    // }
    fn update(&mut self, cpu: &CPU) -> Signal {
        // if self.now.elapsed() > Duration::from_millis(2) {
        //     ram[0x00fffa21] = ram[0x00fffa21].wrapping_add(1);
        //     self.now = Instant::now();
        // }
        cpu.bus.borrow_mut().write(0x00fffa21, OpResult::Byte(1));
        Signal::Ok
    }
    fn read(&self, address: usize, size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) { }
}

pub struct Monitor {
    window: Window,
    buffer: Vec<u32>,
    counter: usize,
}

impl Monitor {
    pub fn new() -> Box<Monitor> {
        let window = Window::new(
            "Test - ESC to exit",
            640,
            400,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        let buffer: Vec<u32> = vec![0; 640 * 400];
        Box::new(Monitor { window, buffer, counter: 0 })
    }
}

impl Device for Monitor {
    // fn init(&mut self, _ram: RamPtr) {
    // }
    fn update(&mut self, cpu: &CPU) -> Signal {
        // self.counter = self.counter.wrapping_add(1);
        // if self.counter % 3600 != 0 {
        //     return Signal::Ok
        // }
        // for (j, p) in self.buffer.iter_mut().enumerate() {
        //     if &cpu.ram.borrow()[0xff8000 + j / 8] & (1 << (7 - j % 8)) > 0 {
        //         *p = 0xffffff;
        //     } else {
        //         *p = 0;
        //     }
        // }
        // self.window 
        //     .update_with_buffer(&self.buffer, 640, 400)
        //     .unwrap();
        Signal::Ok
    }
    fn read(&self, address: usize, size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) { }
}


pub struct Floppy {
    content: Vec<u8>,
}

impl Floppy {
    pub fn new(image: &str) -> Box<Self> {
        let content = fs::read(image).expect("Disk image does not exist!");
        Box::new(Self { content })
    }
}

impl Device for Floppy {
    // fn init(&mut self, ram: RamPtr) {
    //     let mut raw_mem = ram.as_ref().borrow_mut();
    //     println!("{:08x}", 0xfa0000 + self.content.len());
    //     for (j, &b) in self.content.iter().enumerate() {
    //         raw_mem[j + 0xfa0000 as usize] = b;
    //     }
    // }
    fn update(&mut self, _cpu: &CPU) -> Signal { 
        Signal::Ok
    }
    fn read(&self, address: usize, size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) { }
}

