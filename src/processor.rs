// This is the place for the basic processor implementation like the evaluation
// loop and addressing capabilities, which are implemented in CPU.memory_handle().
// The details about how said MemoryHandles behave are implemented in the memory
// module.

use crate::fields::{EAMode, OpResult, Size};
use crate::instructions::Instruction;
use crate::memory::{MemoryHandle, BusPtr, RegPtr};
use crate::parser::parse_instruction;
use crate::devices::Signal;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::rc::Rc;
use std::io;
use std::io::prelude::*;
use termion::{clear, color, cursor};


#[derive(Clone)]
pub struct CPU {
    pub pc: u32,                // Program counter
    pub sr: u32,                // Status register
    pub dr: [RegPtr; 8],        // Data registers
    pub ar: [RegPtr; 8],        // Address registers
    pub ssp: RegPtr,            // Supervisory stack pointer
    pub bus: BusPtr,            // Address Bus
    pub nxt: Instruction,       // Next instruction (debugger)
    pub prev: u32,              // Last program counter (debugger)
    pub jmp: u32,               // Last jump location (debugger)
    pub irq: VecDeque<IRQ>,     // Interrupt request queue
}

#[derive(Copy, Clone)]
pub struct IRQ {
    pub level: u32
}


#[derive(Debug)]
pub struct CCRFlags {
    pub c: Option<bool>,
    pub v: Option<bool>,
    pub z: Option<bool>,
    pub n: Option<bool>,
    pub x: Option<bool>,
}

pub enum CCR {
    C = 0,
    V = 1,
    Z = 2,
    N = 3,
    X = 4,
    S = 13,
}

impl CCRFlags {
    pub fn new() -> CCRFlags {
        CCRFlags { c: None, v: None, z: None, n: None, x: None }
    }
    pub fn set(&self, cpu: &mut CPU) {
        let mut ccr = cpu.sr as usize;
        if let Some(value) = self.c {
            set_bit(&mut ccr, CCR::C as usize, value)
        };
        if let Some(value) = self.v {
            set_bit(&mut ccr, CCR::V as usize, value)
        };
        if let Some(value) = self.z {
            set_bit(&mut ccr, CCR::Z as usize, value)
        };
        if let Some(value) = self.n {
            set_bit(&mut ccr, CCR::N as usize, value)
        };
        if let Some(value) = self.x {
            set_bit(&mut ccr, CCR::X as usize, value)
        };
        cpu.sr = ccr as u32;
    }
}

impl CPU {
    pub fn new(pc: u32, sr: u32, dr: [RegPtr; 8], ar: [RegPtr; 8], ssp: RegPtr, bus: BusPtr) -> Self {
        CPU { pc, sr, dr, ar, ssp, bus, nxt: Instruction::NOP, prev: 0, jmp: 0, irq: VecDeque::new() }
    }
    pub fn clock_cycle(&mut self) -> Signal {
        let next_instruction = self.nxt;
        self.prev = self.pc;
        match next_instruction.execute(self) {
            Signal::Quit => return Signal::Quit,
            _ => {}
        }
        self.jmp = self.pc;
        let opcode = self.next_instruction();
        if let Some(instruction) = parse_instruction(opcode, self) {
            self.nxt = instruction;
            Signal::Ok
        } else {
            self.nxt = Instruction::NOP;
            Signal::Ok
        }
    }
    pub fn next_instruction(&mut self) -> u16 {
        let instr = self.lookahead(0);
        self.pc += 2;
        instr
    }
    pub fn memory_handle(&mut self, mode: EAMode) -> MemoryHandle {
        match mode {
            EAMode::DataDirect(register) => MemoryHandle::new(Some(Rc::clone(&self.dr[register])), None, None, self),
            EAMode::AddressDirect(register) => MemoryHandle::new(Some(self.ar(register)), None, None, self),
            EAMode::AddressIndirect(register) => {
                let ptr = *self.ar(register).borrow() as usize;
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            EAMode::AddressPostincr(register, size) => {
                let ptr = (*self.ar(register)).borrow().clone() as usize;
                if register == 7 && size == Size::Byte {
                    *self.ar(register).borrow_mut() += 2;
                } else {
                    *self.ar(register).borrow_mut() += size as u32;
                }
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            EAMode::AddressPredecr(register, size) => {
                if register == 7 && size == Size::Byte {
                    *self.ar(register).borrow_mut() -= 2;
                } else {
                    *self.ar(register).borrow_mut() -= size as u32;
                }
                let ptr = *self.ar(register).borrow() as usize;
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            EAMode::AddressDisplacement(register, displacement) => {
                let ptr = (*self.ar(register).borrow() as i32 + displacement as i32) as u32 as usize;
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            EAMode::AddressIndex8Bit(register, iregister, displacement, size, scale, da) => {
                let index_handle = if da == 0 {
                    self.memory_handle(EAMode::DataDirect(iregister))
                } else {
                    self.memory_handle(EAMode::AddressDirect(iregister))
                };
                let mut ptr = index_handle.read(size).sign_extend() as i32;
                ptr *= 1 << scale;
                ptr += displacement as i32;
                ptr += *self.ar(register).borrow() as i32;
                MemoryHandle::new(None, Some(ptr as usize), None, self)
            }
            EAMode::AddressIndexBase(register, iregister, displacement, size, scale, da) => {
                let index_handle = if da == 0 {
                    self.memory_handle(EAMode::DataDirect(iregister))
                } else {
                    self.memory_handle(EAMode::AddressDirect(iregister))
                };
                let mut ptr = index_handle.read(size).sign_extend() as i32;
                ptr *= 1 << scale;
                ptr += displacement;
                ptr += *self.ar(register).borrow() as i32;
                MemoryHandle::new(None, Some(ptr as usize), None, self)
            }
            EAMode::AbsoluteShort(ptr) => MemoryHandle::new(None, Some(ptr), None, self),
            EAMode::AbsoluteLong(ptr) => MemoryHandle::new(None, Some(ptr), None, self),
            EAMode::Immediate(data) => MemoryHandle::new(None, None, Some(data), self),
            EAMode::PCIndex8Bit(iregister, displacement, size, scale, da, pc) => {
                let index_handle = if da == 0 {
                    self.memory_handle(EAMode::DataDirect(iregister))
                } else {
                    self.memory_handle(EAMode::AddressDirect(iregister))
                };
                let mut ptr = index_handle.read(size).sign_extend() as i32;
                ptr *= 1 << scale;
                ptr += displacement as i32;
                ptr += pc as i32;
                MemoryHandle::new(None, Some(ptr as usize), None, self)
            }
            EAMode::PCDisplacement(displacement, pc) => {
                let ptr = (pc as i32 + displacement as i32) as usize;
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            _ => panic!("Invalid addressing mode!"),
        }
    }
    pub fn ar(&mut self, register: usize) -> RegPtr {
        if self.in_supervisor_mode() && register == 7 {
            Rc::clone(&self.ssp)
        } else {
            Rc::clone(&self.ar[register])
        }
    }
    pub fn supervisor_mode(&mut self, value: bool) {
        let mut sr = self.sr as usize;
        set_bit(&mut sr, CCR::S as usize, value);
        self.sr = sr as u32;
    }
    pub fn in_supervisor_mode(&self) -> bool {
        self.ccr(CCR::S)
    }
    pub fn lookahead(&self, offset: isize) -> u16 {
        let ptr = (self.pc as isize + 2 * offset) as usize;
        self.bus.borrow_mut().read(ptr, Size::Word).inner() as u16
    }
    pub fn ccr(&self, bit: CCR) -> bool {
        self.sr & (1 << (bit as u8)) != 0
    }
    pub fn immediate_operand(&mut self, size: Size) -> OpResult {
        let extword = self.next_instruction();
        match size {
            Size::Byte => OpResult::Byte((extword & 0xff) as u8),
            Size::Word => OpResult::Word(extword),
            Size::Long => {
                let extword2 = self.next_instruction();
                OpResult::Long(((extword as u32) << 16) + extword2 as u32)
            }
        }
    }
    pub fn memory_address(&mut self, mode: EAMode) -> u32 {
        if let Some(ptr) = self.memory_handle(mode).ptr() {
            ptr as u32
        } else {
            panic!("Invalid addressing mode!")
        }
    }
    pub fn disassemble(&self, lines: usize) -> VecDeque<(u32, Vec<u16>, String)> {
        let mut cpu = (*self).clone();
        let mut disassembly = VecDeque::with_capacity(lines);
        let mut opcodes = Vec::new();
        let length = (cpu.pc - cpu.jmp) / 2;
        for j in 0..length {
            opcodes.push(self.lookahead(j as isize - length as isize));
        }
        disassembly.push_back((cpu.jmp, opcodes, cpu.nxt.as_asm(&cpu)));
        for _ in 0..lines - 1 {
            let pc = cpu.pc;
            let mut opcodes = Vec::new();
            let opcode = cpu.next_instruction();
            let instr = parse_instruction(opcode, &mut cpu);
            let length = (cpu.pc - pc) / 2;
            for j in 0..length {
                opcodes.push(cpu.lookahead(j as isize - length as isize));
            }
            let instr_txt = match instr {
                Some(instruction) => instruction.as_asm(&cpu),
                None => String::from("dc"),
            };
            disassembly.push_back((pc, opcodes, instr_txt));
        }
        disassembly
    }
    pub fn interrupt_mask(&self) -> u32 {
        (self.sr & 0x700) >> 8
    }
    pub fn serve_interrupt_requests(&mut self) {
        self.irq.extend(self.bus.borrow_mut().interrupt_requests());
        if let Some(irq) = self.irq.pop_front() {
            if irq.level == 7 || irq.level > self.interrupt_mask() {
                println!("Interrupt (level {}) occured!", irq.level);
                let trap = Instruction::TRAP { vector: 24 + irq.level as usize };
                trap.execute(self);
            }
        }
    }
    pub fn poll_devices(&self) -> Signal {
        self.bus.borrow().poll_devices()
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("\n");
        s.push_str(&format!("{r}╔══════════════════════════════════╦", r = cursor::Goto(1, 2)));
        s.push_str(&format!("{r}║ CPU state                        ║", r = cursor::Goto(1, 3)));
        s.push_str(&format!("{r}╟─────┬───────────┬────┬───────────╫", r = cursor::Goto(1, 4)));
        for j in 0..8 {
            let ar = if !self.in_supervisor_mode() || j != 7 { *self.ar[j].borrow() } else { *self.ssp.borrow() };
            s.push_str(&format!(
                "{r}║ A{j}  │  {a:08x} │ D{j} │  {d:08x} ║\n",
                j = j,
                a = ar,
                d = *self.dr[j].borrow(),
                r = cursor::Goto(1, (j + 5) as u16),
            ));
        }
        s.push_str(&format!("{r}╟─────┼─┬─┬─┬─┬─┬─┼────┼───────────╢", r = cursor::Goto(1, 13)));
        s.push_str(&format!("{r}║ IRQ │S│X│N│Z│V│C│    │           ║", r = cursor::Goto(1, 14)));
        s.push_str(&format!("{r}╟─────┼─┼─┼─┼─┼─┼─┼────┼───────────╢", r = cursor::Goto(1, 15)));
        s.push_str(&format!(
            "{r}║ {:03b} │{}│{}│{}│{}│{}│{}│ PC │  {:08x} ║\n",
            self.interrupt_mask(),
            self.ccr(CCR::S) as u8,
            self.ccr(CCR::X) as u8,
            self.ccr(CCR::N) as u8,
            self.ccr(CCR::Z) as u8,
            self.ccr(CCR::V) as u8,
            self.ccr(CCR::C) as u8,
            self.pc,
            r = cursor::Goto(1, 16)
        ));
        s.push_str(&format!("{r}╚═════╧═╧═╧═╧═╧═╧═╧════╧═══════════╩", r = cursor::Goto(1, 17)));
        write!(f, "{}", s)
    }
}

pub fn set_bit(bitfield: &mut usize, bit: usize, value: bool) {
    if value {
        *bitfield |= 1 << (bit as u8);
    } else {
        *bitfield &= !(1 << (bit as u8));
    }
}

pub fn get_bit(bitfield: usize, bit: usize) -> bool {
    bitfield & (1 << bit) != 0
}

pub type DisassemblySection = VecDeque<(u32, Vec<u16>, String)>;

pub struct Disassembly {
    pub disassembly: DisassemblySection,
    pub cursor: usize,
    pub length: usize,
    pub breakpoints: HashSet::<u32>,
}

impl Disassembly {
    pub fn new(lines: usize) -> Self {
        Self { disassembly: VecDeque::with_capacity(lines), cursor: 0, length: lines, breakpoints: HashSet::new() }
    }
    pub fn update(&mut self, cpu: &CPU) {
        if self.disassembly.is_empty() {
            self.disassembly = cpu.disassemble(self.length);
        }
        let mut disassembled = HashMap::<u32, usize>::with_capacity(self.length);
        for (j, line) in self.disassembly.iter().enumerate() {
            disassembled.insert(line.0, j);
        }
        let mut jumped = false;
        if self.cursor < self.length {
            if cpu.jmp != self.disassembly[self.cursor].0 {
                match disassembled.get(&cpu.jmp) {
                    Some(cursor) => {
                        jumped = true;
                        self.cursor = *cursor;
                    }
                    None => self.cursor = 0,
                }
            }
        } else {
            self.cursor = 0
        }
        if self.cursor == 0 {
            self.disassembly = cpu.disassemble(self.length);
        } else if (self.cursor >= self.length / 2 + 1) && !jumped {
            let mut disassembly = cpu.disassemble(self.length - self.cursor + 1);
            self.disassembly.pop_front();
            self.disassembly.push_back(disassembly.pop_back().unwrap());
        }
        if self.cursor < self.length / 2 + 1 {
            self.cursor += 1;
        }
    }
}

impl fmt::Display for Disassembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!(
            "{r}═══════════════════════════════════════════════════════════════════════╗\n",
            r = cursor::Goto(37, 2)
        ));
        result.push_str(&format!(
            "{r} Next instructions                                                     ║\n",
            r = cursor::Goto(37, 3)
        ));
        result.push_str(&format!(
            "{r}──────────┬──────────────────────────┬─────────────────────────────────╢\n",
            r = cursor::Goto(37, 4)
        ));
        for (j, line) in self.disassembly.iter().enumerate() {
            let mut out = String::new();
            for word in &line.1 {
                out.push_str(&format!("{:04x} ", word));
            }
            let mut symbol = String::from(" ");
            let mut color = format!("{}", color::Fg(color::Reset));
            if self.breakpoints.contains(&line.0) {
                symbol = format!("{r}*{n}", 
                    n = color::Fg(color::Reset),
                    r = color::Fg(color::Red),);
                if j + 1 == self.cursor {
                    symbol.push_str(&format!("{g}", g = color::Fg(color::Green)));
                    color = format!("{}", color::Fg(color::Green));
                } 
            } else if j + 1 == self.cursor {
                symbol = format!("{g}>", g = color::Fg(color::Green));
                color = format!("{}", color::Fg(color::Green));
            } 
            result.push_str(&format!(
                "{r}{sym}{a:08x}{n} │ {col}{o:<25}{n}│{col} {i:<32}{n}║\n",
                n = color::Fg(color::Reset),
                col = color,
                o = out,
                i = line.2,
                a = line.0,
                r = cursor::Goto(37, (j + 5) as u16),
                sym = symbol,
            ));
        }
        result.push_str(&format!(
            "{r}══════════╧══════════════════════════╧═════════════════════════════════╝\n",
            r = cursor::Goto(37, (self.length + 5) as u16)
        ));
        write!(f, "{}", result)
    }
}

pub struct Debugger {
    disassembly: Disassembly,
    code_running: bool,
    last_cmd: DebugCommand,
    variables: HashSet<u32>,
    call_graph: (String, usize),
}

#[derive(PartialEq, Clone)]
enum DebugCommand {
    Quit,
    SetBreakpoint(Option<String>),
    DeleteBreakpoint(Option<String>),
    Continue,
    Step,
    Jump(Option<String>),
    Watch(Option<String>),
    Unwatch(Option<String>),
}

impl Debugger {
    pub fn new() -> Box<Self> {
        Box::new(Debugger { 
            disassembly: Disassembly::new(12),
            code_running: false,
            last_cmd: DebugCommand::Step,
            variables: HashSet::new(),
            call_graph: (String::new(), 0),
        })
    }
    fn set_breakpoint(&mut self, breakpoint: &Option<String>, cpu: &CPU, delete: bool) {
        if let Some(address) = parse_address(breakpoint) {
            if delete {
                self.disassembly.breakpoints.remove(&address);
            } else {
                self.disassembly.breakpoints.insert(address);
            }
            self.draw_user_interface(cpu);
            if delete {
                println!("Breakpoint deleted.");
            } else {
                println!("Breakpoint created.");
            }
        } else {
            self.draw_user_interface(cpu);
            println!("Invalid address!");
        }
    }
    fn watch_address(&mut self, address: &Option<String>, cpu: &CPU, watch_delete: bool) {
        if let Some(address) = parse_address(address) {
            if watch_delete {
                self.variables.insert(address);
            } else {
                self.variables.remove(&address);
            }
            self.draw_user_interface(cpu);
        } else {
            self.draw_user_interface(cpu);
            println!("Invalid address!");
        }
    }
    fn get_command(&mut self) -> DebugCommand {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut cmd = input.split_whitespace();
        match cmd.next() {
            Some("q") => return DebugCommand::Quit,
            Some("s") | Some("n") => return DebugCommand::Step,
            Some("b") => return DebugCommand::SetBreakpoint(cmd.next().map(String::from)),
            Some("d") => return DebugCommand::DeleteBreakpoint(cmd.next().map(String::from)),
            Some("j") => return DebugCommand::Jump(cmd.next().map(String::from)),
            Some("w") => return DebugCommand::Watch(cmd.next().map(String::from)),
            Some("u") => return DebugCommand::Unwatch(cmd.next().map(String::from)),
            Some("c") => return DebugCommand::Continue,
            _ => return self.last_cmd.clone(),
        }
    }
    fn draw_user_interface(&mut self, cpu: &CPU) {
        println!("{}", clear::All);
        print!("{c}{tl}{cpu}", c = clear::All, tl = cursor::Goto(1, 1), cpu = cpu);
        print!("{tr}{dis}", tr = cursor::Goto(10, 10), dis = self.disassembly);
        print!("{r} Next instruction: {n}", r = cursor::Goto(37, 3), n = cpu.nxt.as_asm(cpu));
        if !self.variables.is_empty() {
            println!("{r}Watched memory locations", r = cursor::Goto(1, 6 + self.disassembly.length as u16));
            for var in self.variables.iter() {
                println!("{:08x}: {}", var, cpu.bus.borrow_mut().read(*var as usize, Size::Long))
            }
        }
        println!("{r}\nDebugger attached. Enter n to single step, c to continue, b/d <addr> to enter/delete a breakpoint at addr, j <addr> to jump to <addr> or q to quit.", 
            r = cursor::Goto(1, (7 + self.disassembly.length + self.variables.len()) as u16));
        print!("{r}> ", r = cursor::Goto(1, (9 + self.disassembly.length + self.variables.len()) as u16));
        io::stdout().flush().expect("");
    }
    pub fn update(&mut self, cpu: &mut CPU) -> Signal {
        if !self.code_running || self.disassembly.breakpoints.contains(&cpu.jmp) {
            self.update_call_graph(cpu);
            self.code_running = false;
            self.disassembly.update(cpu);
            self.draw_user_interface(cpu);
            let cmd = self.get_command();
            match &cmd {
                DebugCommand::Quit => {
                    // std::fs::write("callgraph", &self.call_graph.0);
                    Signal::Quit
                },
                DebugCommand::SetBreakpoint(b) => {
                    self.set_breakpoint(&b, cpu, false);
                    Signal::NoOp
                },
                DebugCommand::DeleteBreakpoint(b) => {
                    self.set_breakpoint(&b, cpu, true);
                    Signal::NoOp
                },
                DebugCommand::Watch(a) => {
                    self.watch_address(&a, cpu, true);
                    Signal::NoOp
                },
                DebugCommand::Unwatch(a) => {
                    self.watch_address(&a, cpu, false);
                    Signal::NoOp
                },
                DebugCommand::Continue => {
                    self.code_running = true;
                    Signal::Ok
                },
                DebugCommand::Step => {
                    self.last_cmd = cmd;
                    Signal::Ok
                }
                DebugCommand::Jump(a) => {
                    if let Some(address) = parse_address(a) {
                        cpu.pc = address;
                        cpu.nxt = Instruction::NOP;
                        self.last_cmd = cmd;
                        Signal::Ok
                    } else {
                        Signal::NoOp
                    }
                }
            }
        } else {
            self.update_call_graph(cpu);
            Signal::Ok
        }
    }
    fn update_call_graph(&mut self, cpu: &CPU) {
        if self.call_graph.0.is_empty() {
            self.call_graph.0.push_str(&format!("{:08x}", cpu.pc))
        }
        match cpu.nxt {
            Instruction::BSR { displacement: _ } => {
                let mut newline = "\n".to_string();
                self.call_graph.1 += 1;
                for _ in 0..self.call_graph.1 {
                    newline.push_str("    ")
                }
                newline.push_str(&cpu.nxt.as_asm(cpu));
                self.call_graph.0.push_str(&newline);
            }
            Instruction::RTS => {
                self.call_graph.1 -= 1;
            }
            _ => ()
        }
    }
}

fn parse_address(address: &Option<String>) -> Option<u32> {
    match address {
        Some(addr) => {
            match u32::from_str_radix(&addr, 16) {
                Ok(address) => Some(address),
                Err(_) => None
            }
        }
        None => None
    }
}