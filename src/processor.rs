use std::fmt;
use std::rc::Rc;
use std::io::{stdin, stdout, Read, Write};
use crate::parser::{parse_extension_word, parse_instruction};
use crate::instructions::ExtensionWord::*;
use crate::memory::{MemoryHandle, OpResult, Size, RamPtr, RegPtr};

pub struct CPU {
    pub pc: u32,         // Program counter
    pub sr: u32,         // Status register
    pub dr: [RegPtr; 8], // Data registers
    pub ar: [RegPtr; 8], // Address registers
    pub ssp: RegPtr,     // Supervisory stack pointer
    pub ram: RamPtr,     // Pointer to RAM
}

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
    S = 13
}

impl CCRFlags {
    pub fn new() -> CCRFlags {
        CCRFlags {c: None, v: None, z: None, n: None, x: None}
    }
    pub fn set(&self, cpu: &mut CPU) {
        if let Some(value) = self.c { set_bit(&mut cpu.sr, CCR::C as usize, value) };
        if let Some(value) = self.v { set_bit(&mut cpu.sr, CCR::V as usize, value) };
        if let Some(value) = self.z { set_bit(&mut cpu.sr, CCR::Z as usize, value) };
        if let Some(value) = self.n { set_bit(&mut cpu.sr, CCR::N as usize, value) };
        if let Some(value) = self.x { set_bit(&mut cpu.sr, CCR::X as usize, value) };
    }
}

impl CPU {
    pub fn clock_cycle(&mut self) {
        let opcode = self.next_instruction();
        if let Some(instruction) = parse_instruction(opcode) {
            println!("{:?}", self);
            println!("Next instruction: {:}", instruction.as_asm(self));
            pause();
            instruction.execute(self);
        } else {
            panic!("Illegal instruction!");
        }
    }
    pub fn next_instruction(&mut self) -> u16 {
        let instr = self.lookahead(0); 
        self.pc += 2;
        instr
    }
    pub fn memory_handle(&mut self, mode: usize, register: usize, size: Size) -> MemoryHandle {
        match mode {
            // Data register direct mode
            0 => MemoryHandle { reg: Some(Rc::clone(&self.dr[register])), ptr: None, mem: None },
            // Address register direct mode
            1 => MemoryHandle { reg: Some(Rc::clone(&self.ar[register].clone())), ptr: None, mem: None },
            // Address register indirect mode
            2 => {
                let ptr = *self.ar[register].borrow() as usize;
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            }
            // Address register indirect with postincrement mode
            3 => {
                let ptr = (*self.ar[register]).borrow().clone() as usize;
                if register == 7 && size as u32 == 1 {
                    *self.ar[register].borrow_mut() += 2;
                } else {
                    *self.ar[register].borrow_mut() += size as u32;
                }
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            }
            // Address register indirect with predecrement mode
            4 => {
                if register == 7 && size as u32 == 1 {
                    *self.ar[register].borrow_mut() -= 2;
                } else {
                    *self.ar[register].borrow_mut() -= size as u32;
                }
                let ptr = *self.ar[register].borrow() as usize;
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            }
            // Address register indirect with displacement mode
            5 => {
                let displacement = self.next_instruction() as i16;
                let ptr = (*self.ar[register].borrow() + displacement as u32) as usize;
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            }
            6 => {
                let opcode = self.next_instruction();
                if let Some(extword) = parse_extension_word(opcode) {
                    let mut ptr;
                    match extword {
                        // Address Register Indirect with Index (8-Bit Displacement) Mode
                        BEW { da, register: iregister, wl: _, scale, displacement } => {
                            if da == 0 {
                                ptr = *self.dr[iregister].borrow_mut();
                            } else {
                                ptr = *self.ar[iregister].borrow_mut();
                            }
                            ptr *= 1 << scale;
                            ptr += (displacement & 0xff) as i8 as u32;
                            MemoryHandle { reg: None, ptr: Some(ptr as usize), mem: Some(Rc::clone(&self.ram)) }
                        }
                        // Address Register Indirect with Index (Base Displacement) Mode
                        FEW { da, register: iregister, wl: _, scale, bs: _, is: _, bdsize: _, iis: _ } => {
                            if da == 0 {
                                ptr = *self.dr[iregister].borrow_mut();
                            } else {
                                ptr = *self.ar[iregister].borrow_mut();
                            }
                            ptr *= 1 << scale;
                            let mut displacement: u32 = 0;
                            let (bdsize, _) = extword.remaining_length();
                            for j in 0..bdsize {
                                displacement += (self.next_instruction() * (1 << (8 * (bdsize - j - 1)))) as u32;
                            }
                            ptr += displacement;
                            MemoryHandle { reg: None, ptr: Some(ptr as usize), mem: Some(Rc::clone(&self.ram)) }
                        }
                    }
                } else {
                    panic!("Invalid extension word!")
                }
            }
            7 => {
                let extword = self.next_instruction();
                match register {
                    // 0 => {
                    //     // Absolute Short Addressing Mode
                    // },
                    1 => {
                        // Absolute Long Addressing Mode
                        let extword2 = self.next_instruction();
                        let mut ptr = extword2 as usize;
                        ptr += (extword as usize) << 16;
                        MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
                    }
                    // 2 => {
                    //     // Program Counter Indirect with Displacement Mode
                    // },
                    // 3 => {
                    //     // Program Counter Indirect with Index (8-Bit Displacement) Mode
                    //     // Program Counter Indirect with Index (Base Displacement) Mode
                    //     // Program Counter Memory Indirect Preindexed Mode
                    // },
                    4 => {
                        // Immediate Data
                        match size {
                            Size::Byte => MemoryHandle {
                                reg: None,
                                ptr: Some(self.pc as usize - 1),
                                mem: Some(Rc::clone(&self.ram)),
                            },
                            Size::Word => MemoryHandle {
                                reg: None,
                                ptr: Some(self.pc as usize - 2),
                                mem: Some(Rc::clone(&self.ram)),
                            },
                            Size::Long => {
                                self.pc += 2;
                                MemoryHandle {
                                    reg: None,
                                    ptr: Some(self.pc as usize - 4),
                                    mem: Some(Rc::clone(&self.ram)),
                                }
                            }
                        }
                    }
                    _ => panic!("Invalid register!"),
                }
            }
            _ => panic!("Invalid addressing mode!"),
        }
    }
    pub fn supervisor_mode(&mut self, value: bool) {
        set_bit(&mut self.sr, CCR::S as usize, value);
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
    pub fn memory_address(&mut self, mode: usize, earegister: usize) -> u32 {
        if let Some(ptr) = self.memory_handle(mode, earegister, Size::Byte).ptr {
            (ptr & 0xffffffff) as u32
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
        s.push_str(&format!("\nCCR: X: {:} N: {:} Z: {:} V: {:} C: {:}", 
            self.ccr(CCR::X) as u8, 
            self.ccr(CCR::N) as u8, 
            self.ccr(CCR::Z) as u8, 
            self.ccr(CCR::V) as u8, 
            self.ccr(CCR::C) as u8));
        s.push_str(&format!("\nPC: {:08x}", self.pc));
        write!(f, "{}", s)
    }
}

pub fn set_bit(bitfield: &mut u32, bit: usize, value: bool) {
    if value {
        *bitfield |= 1 << (bit as u8);
    } else {
        *bitfield &= !(1 << (bit as u8));
    }
}

pub fn get_bit(bitfield: u32, bit: usize) -> bool {
    bitfield & (1 << bit) != 0
}


fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue, CTRL+C to quit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}