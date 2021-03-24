// This is the place for the basic processor implementation like the evaluation
// loop and addressing capabilities, which are implemented in CPU.memory_handle().
// The details about how said MemoryHandles behave are implemented in the memory
// module.

use crate::fields::{EAMode, Size, OpResult};
use crate::memory::{MemoryHandle, RamPtr, RegPtr};
use crate::parser::parse_instruction;
use crate::instructions::Instruction;
use std::fmt;
use std::rc::Rc;

pub struct CPU {
    pub pc: u32,           // Program counter
    pub sr: u32,           // Status register
    pub dr: [RegPtr; 8],   // Data registers
    pub ar: [RegPtr; 8],   // Address registers
    pub ssp: RegPtr,       // Supervisory stack pointer
    pub ram: RamPtr,       // Pointer to RAM
    pub nxt: Instruction,  // Next instruction
    pub prev: u32,         // Last program counter for debugging
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
        CPU { pc, sr, dr, ar, ssp, ram, nxt: Instruction::NOP, prev: 0 }
    }
    pub fn clock_cycle(&mut self) {
        let next_instruction = self.nxt;
        self.prev = self.pc;
        next_instruction.execute(self);
        let opcode = self.next_instruction();
        if let Some(instruction) = parse_instruction(opcode, self) {
            self.nxt = instruction;
        } else {
            panic!("Illegal instruction!");
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
            EAMode::AddressDirect(register) => {
                MemoryHandle::new(Some(Rc::clone(&self.ar[register].clone())), None, None, self)
            }
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
                let ptr = (*self.ar[register].borrow() + displacement as u32) as usize;
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
            _ => panic!("Invalid addressing mode!"),
        }
    }
    pub fn supervisor_mode(&mut self, value: bool) {
        set_bit(&mut (self.sr as usize), CCR::S as usize, value);
    }
    pub fn in_supervisor_mode(&self) -> bool {
        self.ccr(CCR::S)
    }
    pub fn lookahead(&self, offset: usize) -> u16 {
        let raw_mem = *self.ram.borrow();
        let ptr = self.pc as usize + 2 * offset;
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
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("\nRegisters:\n");
        for j in 0..8 {
            s.push_str(&format!(
                "A{j}: {a:08x}     D{j}: {d:08x}\n",
                j = j,
                a = *self.ar[j].borrow(),
                d = *self.dr[j].borrow()
            ));
        }
        s.push_str(&format!(
            "\nCCR: X: {:} N: {:} Z: {:} V: {:} C: {:}",
            self.ccr(CCR::X) as u8,
            self.ccr(CCR::N) as u8,
            self.ccr(CCR::Z) as u8,
            self.ccr(CCR::V) as u8,
            self.ccr(CCR::C) as u8
        ));
        s.push_str(&format!("\nPC: {:08x}", self.prev));
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
