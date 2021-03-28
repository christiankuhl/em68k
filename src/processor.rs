// This is the place for the basic processor implementation like the evaluation
// loop and addressing capabilities, which are implemented in CPU.memory_handle().
// The details about how said MemoryHandles behave are implemented in the memory
// module.

use crate::fields::{EAMode, OpResult, Size};
use crate::instructions::Instruction;
use crate::memory::{MemoryHandle, RamPtr, RegPtr};
use crate::parser::parse_instruction;
use crate::devices::Signal;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::rc::Rc;
use termion::{color, cursor};

pub struct CPU {
    pub pc: u32,          // Program counter
    pub sr: u32,          // Status register
    pub dr: [RegPtr; 8],  // Data registers
    pub ar: [RegPtr; 8],  // Address registers
    pub ssp: RegPtr,      // Supervisory stack pointer
    pub ram: RamPtr,      // Pointer to RAM
    pub nxt: Instruction, // Next instruction (debugger)
    pub prev: u32,        // Last program counter (debugger)
    jmp: u32,             // Last jump location (debugger)
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
    pub fn new(pc: u32, sr: u32, dr: [RegPtr; 8], ar: [RegPtr; 8], ssp: RegPtr, ram: RamPtr) -> Self {
        CPU { pc, sr, dr, ar, ssp, ram, nxt: Instruction::NOP, prev: 0, jmp: 0 }
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
            // panic!("Illegal instruction!");
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
            EAMode::AddressDirect(register) => MemoryHandle::new(Some(Rc::clone(&self.ar[register].clone())), None, None, self),
            EAMode::AddressIndirect(register) => {
                let ptr = *self.ar[register].borrow() as usize;
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            EAMode::AddressPostincr(register, size) => {
                let ptr = (*self.ar[register]).borrow().clone() as usize;
                if register == 7 && size as u32 == 1 {
                    *self.ar[register].borrow_mut() += 2;
                } else {
                    *self.ar[register].borrow_mut() += size as u32;
                }
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            EAMode::AddressPredecr(register, size) => {
                if register == 7 && size as u32 == 1 {
                    *self.ar[register].borrow_mut() -= 2;
                } else {
                    *self.ar[register].borrow_mut() -= size as u32;
                }
                let ptr = *self.ar[register].borrow() as usize;
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            EAMode::AddressDisplacement(register, displacement) => {
                let ptr = (*self.ar[register].borrow() as i32 + displacement as i32) as u32 as usize;
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
                ptr += *self.ar[register].borrow() as i32;
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
                ptr += *self.ar[register].borrow() as i32;
                MemoryHandle::new(None, Some(ptr as usize), None, self)
            }
            EAMode::AbsoluteShort(ptr) => MemoryHandle::new(None, Some(ptr), None, self),
            EAMode::AbsoluteLong(ptr) => MemoryHandle::new(None, Some(ptr), None, self),
            EAMode::Immediate(data) => MemoryHandle::new(None, None, Some(data), self),
            EAMode::PCIndex8Bit(iregister, displacement, size, scale, da) => {
                let index_handle = if da == 0 {
                    self.memory_handle(EAMode::DataDirect(iregister))
                } else {
                    self.memory_handle(EAMode::AddressDirect(iregister))
                };
                let mut ptr = index_handle.read(size).sign_extend() as i32;
                ptr *= 1 << scale;
                ptr += displacement as i32;
                ptr += self.pc as i32;
                MemoryHandle::new(None, Some(ptr as usize), None, self)
            }
            EAMode::PCDisplacement(displacement) => {
                let ptr = (self.pc as i32 + displacement as i32 - 2) as usize;
                MemoryHandle::new(None, Some(ptr), None, self)
            }
            _ => panic!("Invalid addressing mode!"),
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
        let raw_mem = self.ram.borrow();
        let ptr = (self.pc as isize + 2 * offset) as usize;
        u16::from_be_bytes([raw_mem[ptr], raw_mem[ptr + 1]])
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
    pub fn disassemble(&mut self, lines: usize) -> VecDeque<(u32, Vec<u16>, String)> {
        let initial_pc = self.pc;
        let mut disassembly = VecDeque::with_capacity(lines);
        let mut opcodes = Vec::new();
        let length = (self.pc - self.jmp) / 2;
        for j in 0..length {
            opcodes.push(self.lookahead(j as isize - length as isize));
        }
        disassembly.push_back((self.jmp, opcodes, self.nxt.as_asm(self)));
        for _ in 0..lines - 1 {
            let pc = self.pc;
            let mut opcodes = Vec::new();
            let opcode = self.next_instruction();
            let instr = parse_instruction(opcode, self);
            let length = (self.pc - pc) / 2;
            for j in 0..length {
                opcodes.push(self.lookahead(j as isize - length as isize));
            }
            let instr_txt = match instr {
                Some(instruction) => instruction.as_asm(self),
                None => String::from("ERR"),
            };
            disassembly.push_back((pc, opcodes, instr_txt));
        }
        self.pc = initial_pc;
        disassembly
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("\n");
        s.push_str(&format!("{r}╔═════════════════════════════════╦", r = cursor::Goto(1, 2)));
        s.push_str(&format!("{r}║ CPU state                       ║", r = cursor::Goto(1, 3)));
        s.push_str(&format!("{r}╟────┬───────────┬────┬───────────╫", r = cursor::Goto(1, 4)));
        for j in 0..8 {
            s.push_str(&format!(
                "{r}║ A{j} │  {a:08x} │ D{j} │  {d:08x} ║\n",
                j = j,
                a = *self.ar[j].borrow(),
                d = *self.dr[j].borrow(),
                r = cursor::Goto(1, (j + 5) as u16),
            ));
        }
        s.push_str(&format!("{r}╟────┼─┬─┬─┬─┬─┬─┼────┼───────────╢", r = cursor::Goto(1, 13)));
        s.push_str(&format!("{r}║    │S│X│N│Z│V│C│    │           ║", r = cursor::Goto(1, 14)));
        s.push_str(&format!("{r}╟────┼─┼─┼─┼─┼─┼─┼────┼───────────╢", r = cursor::Goto(1, 15)));
        s.push_str(&format!(
            "{r}║ SR │{}│{}│{}│{}│{}│{}│ PC │  {:08x} ║\n",
            self.ccr(CCR::S) as u8,
            self.ccr(CCR::X) as u8,
            self.ccr(CCR::N) as u8,
            self.ccr(CCR::Z) as u8,
            self.ccr(CCR::V) as u8,
            self.ccr(CCR::C) as u8,
            self.pc,
            r = cursor::Goto(1, 16)
        ));
        s.push_str(&format!("{r}╚════╧═╧═╧═╧═╧═╧═╧════╧═══════════╩", r = cursor::Goto(1, 17)));
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
    pub fn update(&mut self, cpu: &mut CPU) {
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
                        self.cursor = *cursor + 1;
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
            r = cursor::Goto(36, 2)
        ));
        result.push_str(&format!(
            "{r} Next instructions                                                     ║\n",
            r = cursor::Goto(36, 3)
        ));
        result.push_str(&format!(
            "{r}──────────┬──────────────────────────┬─────────────────────────────────╢\n",
            r = cursor::Goto(36, 4)
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
                r = cursor::Goto(36, (j + 5) as u16),
                sym = symbol,
            ));
        }
        result.push_str(&format!(
            "{r}══════════╧══════════════════════════╧═════════════════════════════════╝\n",
            r = cursor::Goto(36, (self.length + 5) as u16)
        ));
        write!(f, "{}", result)
    }
}
