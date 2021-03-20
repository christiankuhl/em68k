use std::rc::Rc;
use crate::{CPU, CCR};
use crate::memory::{OpResult, MemoryHandle};
use crate::parser::parse_extension_word;

#[derive(Debug)]
pub enum Instruction {
    ANDICCR,
    ANDISR,
    EORICCR,
    EORISR,
    ILLEGAL,
    NOP,
    ORICCR,
    ORISR,
    RESET,
    RTE,
    RTR,
    RTS,
    STOP,
    TRAPV,
    LINK { register: usize },
    SWAP { register: usize },
    UNLK { register: usize },
    TRAP { vector: usize },
    MOVEUSP { register: usize, dr: usize },
    BCHGS { mode: usize, earegister: usize },
    BCLRS { mode: usize, earegister: usize },
    BSETS { mode: usize, earegister: usize },
    BTSTS { mode: usize, earegister: usize },
    JMP { mode: usize, earegister: usize },
    JSR { mode: usize, earegister: usize },
    MOVECCR { mode: usize, earegister: usize },
    MOVEFROMSR { mode: usize, earegister: usize },
    MOVETOSR { mode: usize, earegister: usize },
    PEA { mode: usize, earegister: usize },
    TAS { mode: usize, earegister: usize },
    EXT { mode: usize, earegister: usize },
    ASLRMEM { mode: usize, earegister: usize },
    LSLRMEM { mode: usize, earegister: usize },
    DBCC { condition: usize, register: usize },
    MOVEM { size: usize, dr: usize, mode: usize, earegister: usize },
    ABCD { rx: usize, ry: usize, rm: usize },
    SBCD { rx: usize, ry: usize, rm: usize },
    ADDI { size: usize, mode: usize, earegister: usize },
    ANDI { size: usize, mode: usize, earegister: usize },
    CLR { size: usize, mode: usize, earegister: usize },
    CMPI { size: usize, mode: usize, earegister: usize },
    EORI { size: usize, mode: usize, earegister: usize },
    NEG { size: usize, mode: usize, earegister: usize },
    NEGX { size: usize, mode: usize, earegister: usize },
    NOT { size: usize, mode: usize, earegister: usize },
    ORI { size: usize, mode: usize, earegister: usize },
    SUBI { size: usize, mode: usize, earegister: usize },
    TST { size: usize, mode: usize, earegister: usize },
    BRA { displacement: usize },
    BSR { displacement: usize },
    CMPM { ax: usize, ay: usize, size: usize },
    ADDX { rx: usize, ry: usize, rm: usize, size: usize },
    SUBX { rx: usize, ry: usize, rm: usize, size: usize },
    BCHG { register: usize, mode: usize, earegister: usize },
    BCLR { register: usize, mode: usize, earegister: usize },
    BSET { register: usize, mode: usize, earegister: usize },
    BTST { register: usize, mode: usize, earegister: usize },
    DIVS { register: usize, mode: usize, earegister: usize },
    DIVU { register: usize, mode: usize, earegister: usize },
    LEA { register: usize, mode: usize, earegister: usize },
    MULS { register: usize, mode: usize, earegister: usize },
    MULU { register: usize, mode: usize, earegister: usize },
    NBCD { register: usize, mode: usize, earegister: usize },
    MOVEP { dregister: usize, opmode: usize, aregister: usize },
    SCC { condition: usize, mode: usize, earegister: usize },
    ASLRREG { register: usize, count: usize, size: usize, dr: usize, lr: usize },
    LSLRREG { register: usize, count: usize, size: usize, dr: usize, lr: usize },
    ROXLR { register: usize, count: usize, size: usize, dr: usize, lr: usize },
    ROLR { register: usize, count: usize, size: usize, dr: usize, lr: usize },
    MOVEQ { register: usize, data: usize },
    EXG { mode: usize, rx: usize, ry: usize },
    CHK { register: usize, size: usize, mode: usize, earegister: usize },
    MOVEA { register: usize, size: usize, mode: usize, earegister: usize },
    ADDQ { data: usize, size: usize, mode: usize, earegister: usize },
    SUBQ { data: usize, size: usize, mode: usize, earegister: usize },
    BCC { condition: usize, displacement: usize },
    ADD { register: usize, opmode: usize, mode: usize, earegister: usize },
    AND { register: usize, opmode: usize, mode: usize, earegister: usize },
    CMP { register: usize, opmode: usize, mode: usize, earegister: usize },
    EOR { register: usize, opmode: usize, mode: usize, earegister: usize },
    OR { register: usize, opmode: usize, mode: usize, earegister: usize },
    SUB { register: usize, opmode: usize, mode: usize, earegister: usize },
    MOVE { size: usize, destreg: usize, destmode: usize, srcmode: usize, srcreg: usize },
}

pub enum ExtensionWord {
    BEW { da: usize, register: usize, wl: usize, scale: usize, displacement: usize },
    FEW { da: usize, register: usize, wl: usize, scale: usize, bs: usize, is: usize, bdsize: usize, iis: usize },
}

impl ExtensionWord {
    pub fn remaining_length(&self) -> (usize, usize) {
        match *self {
            Self::FEW { da, register, wl, scale, bs, is, bdsize, iis } => {
                let bdsize_out;
                if bdsize == 2 || bdsize == 3 {
                    bdsize_out = bdsize - 1;
                } else {
                    bdsize_out = 0;
                }
                match iis {
                    2 | 6 => (bdsize_out, 1),
                    3 | 7 => (bdsize_out, 2),
                    _ => (bdsize_out, 0),
                }
            }
            _ => (0, 0),
        }
    }
}

impl Instruction {
    pub fn execute(&self, cpu: &mut CPU) {
        match *self {
            Self::ANDICCR => {
                let extword = cpu.next_instruction();
                cpu.sr &= 0xff00 | extword;
            }
            Self::ANDISR => {
                cpu.sr &= cpu.next_instruction();
            }
            Self::EORICCR => {
                let extword = cpu.next_instruction();
                cpu.sr ^= 0x001f & extword;
            }
            Self::EORISR => {
                cpu.sr ^= cpu.next_instruction();
            }
            Self::ILLEGAL => {
                let trap = Self::TRAP { vector: 4 };
                trap.execute(cpu);
            }
            Self::NOP => {}
            Self::ORICCR => {
                let extword = cpu.next_instruction();
                cpu.sr |= 0x001f & extword;
            }
            Self::ORISR => {
                cpu.sr |= cpu.next_instruction();
            }
            Self::RESET => {
                if !cpu.in_supervisor_mode() {
                    privilege_violation(cpu);
                }
            }
            Self::RTE => {
                if !cpu.in_supervisor_mode() {
                    privilege_violation(cpu);
                } else {
                    let mut ssp = cpu.ssp.as_ref().borrow_mut();
                    let mut ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                    cpu.sr = ram_handle.read(2).inner() as u16;
                    *ssp += 2;
                    ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                    cpu.pc = ram_handle.read(4).inner();
                    *ssp += 4;
                    // FIXME: Do the actual restore
                }
            }
            Self::RTR => {
                let mut sp = cpu.ar[7].as_ref().borrow_mut();
                let mut ram_handle = MemoryHandle { reg: None, ptr: Some(*sp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                let ccr = ram_handle.read(2).inner() as u16 & 0x00ff;
                cpu.sr &= 0xff00;
                cpu.sr |= ccr;
                *sp += 2;
                ram_handle = MemoryHandle { reg: None, ptr: Some(*sp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                cpu.pc = ram_handle.read(4).inner();
                *sp += 4;
            }
            Self::RTS => {
                let mut sp = cpu.ar[7].as_ref().borrow_mut();
                let ram_handle = MemoryHandle { reg: None, ptr: Some(*sp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                cpu.pc = ram_handle.read(4).inner();
                *sp += 4;
            }
            Self::STOP => {
                if !cpu.in_supervisor_mode() {
                    privilege_violation(cpu);
                } else {
                    let extword = cpu.next_instruction();
                    cpu.sr = extword;
                    // FIXME: Implement actual CPU STOP
                }
            }
            Self::TRAPV => {
                if cpu.sr & (1 << (CCR::V as u8)) != 0 {
                    let trap = Self::TRAP { vector: 7 };
                    trap.execute(cpu);        
                }
            }
            Self::LINK { register } => {
                let displacement = cpu.next_instruction();
                let mut sp = cpu.ar[7].as_ref().borrow_mut();
                *sp -= 4;
                let mut ar = cpu.ar[register].as_ref().borrow_mut();
                let ram_handle = MemoryHandle { reg: None, ptr: Some(*sp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                ram_handle.write(OpResult::Long(*ar));
                *ar = *sp;
                *sp = (*sp as i32 + displacement as i32) as u32;
            }
            Self::SWAP { register } => {
                let res;
                {
                    let mut reg = cpu.dr[register].as_ref().borrow_mut();
                    *reg = (*reg & 0xffff0000 >> 16) + (*reg & 0xffff << 16);
                    res = *reg;
                }
                cpu.set_ccr(CCR::V, false);
                cpu.set_ccr(CCR::N, res & (1 << 31) > 0);
                cpu.set_ccr(CCR::Z, res == 0);
                cpu.set_ccr(CCR::C, false);
            }
            Self::UNLK { register } => {
                let mut sp = cpu.ar[7].as_ref().borrow_mut();
                let mut ar = cpu.ar[register].as_ref().borrow_mut();
                *sp = *ar;
                let ram_handle = MemoryHandle { reg: None, ptr: Some(*sp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                *ar = ram_handle.read(4).inner();
                *sp += 4;
            }
            Self::TRAP { vector } => {
                cpu.set_ccr(CCR::S, true);
                let mut ssp = cpu.ssp.as_ref().borrow_mut();
                *ssp -= 4;
                let mut ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                ram_handle.write(OpResult::Long(cpu.pc));
                *ssp -= 2;
                ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                ram_handle.write(OpResult::Word(cpu.sr));
                cpu.pc = (4 * vector + 0x7E) as u32;
            }
            Self::MOVEUSP { register, dr } => {
                if !cpu.in_supervisor_mode() {
                    privilege_violation(cpu);
                } else {
                    if dr == 0 {
                        let ar = cpu.ar[register].as_ref().borrow();
                        let mut usp = cpu.ar[7].as_ref().borrow_mut(); 
                        *usp = *ar;
                    } else {
                        let mut ar = cpu.ar[register].as_ref().borrow_mut();
                        let usp = cpu.ar[7].as_ref().borrow(); 
                        *ar = *usp;
                    }
                }
            }
            Self::BCHGS { mode, earegister } => {}
            Self::BCLRS { mode, earegister } => {}
            Self::BSETS { mode, earegister } => {}
            Self::BTSTS { mode, earegister } => {}
            Self::JMP { mode, earegister } => {}
            Self::JSR { mode, earegister } => {}
            Self::MOVECCR { mode, earegister } => {
                let src = cpu.memory_handle(mode, earegister, 2).read(2).inner() as u16;
                cpu.sr &= 0xff00;
                cpu.sr |= src;
            }
            Self::MOVEFROMSR { mode, earegister } => {
                let dest = cpu.memory_handle(mode, earegister, 2);
                dest.write(OpResult::Word(cpu.sr & 0x8e0));
            }
            Self::MOVETOSR { mode, earegister } => {
                let src = cpu.memory_handle(mode, earegister, 2).read(2).inner() as u16;
                cpu.sr = src & 0x8e0;
            }
            Self::PEA { mode, earegister } => {}
            Self::TAS { mode, earegister } => {}
            Self::EXT { mode, earegister } => {}
            Self::ASLRMEM { mode, earegister } => {}
            Self::LSLRMEM { mode, earegister } => {}
            Self::DBCC { condition, register } => {}
            Self::MOVEM { size, dr, mode, earegister } => {
                // FIXME: Handle address register
                let mut register_mask = cpu.next_instruction();
                let oplength = 1 << (size + 1);
                if dr == 0 {
                    let mut tgt = cpu.memory_handle(mode, earegister, oplength);
                    let mut result;
                    // In Control and postincrement mode the mask order is A7..D0 (LSB first), reversed for predecrement
                    if mode == 4 {
                        register_mask = register_mask.reverse_bits();
                        tgt.offset(-(oplength as isize));
                    }
                    for j in 0..16 {
                        if register_mask & (1 << j) != 0 {
                            let register;
                            if j < 8 {
                                register = cpu.dr[j].as_ref().borrow();
                            } else {
                                register = cpu.ar[j].as_ref().borrow();
                            }
                            if oplength == 2 {
                                result = OpResult::Word((*register & 0xffff) as u16)
                            } else {
                                result = OpResult::Long(*register);
                            }
                            tgt.write(result);
                            if mode == 4 {
                                tgt.offset(-(oplength as isize));
                            } else {
                                tgt.offset(oplength as isize);
                            }
                        }
                    }
                } else if dr == 1 {
                    let mut src = cpu.memory_handle(mode, earegister, oplength);
                    let mut result;
                    for j in 0..16 {
                        if register_mask & (1 << j) != 0 {
                            let mut register;
                            if j < 8 {
                                register = cpu.dr[j].as_ref().borrow_mut();
                            } else {
                                register = cpu.ar[j].as_ref().borrow_mut();
                            }
                            if oplength == 2 {
                                result = ((src.read(oplength).inner() & 0xffff) as i16) as u32
                            } else {
                                result = src.read(oplength).inner()
                            }
                            *register = result;
                            src.offset(oplength as isize);
                        }
                    }
                }
            }
            Self::ABCD { rx, ry, rm } => {}
            Self::SBCD { rx, ry, rm } => {}
            Self::ADDI { size, mode, earegister } => {
                let extword = cpu.next_instruction();
                let handle = cpu.memory_handle(mode, earegister, 1 << size);
                let operand = handle.read(1 << size).inner();
                let overflow;
                let negative;
                let zero;
                match size {
                    0 => {
                        let summand = (extword & 0x00ff) as u8;
                        let res = (operand as u8).overflowing_add(summand);
                        overflow = res.1;
                        negative = (res.0 as i32) < 0;
                        zero = res.0 == 0;
                        handle.write(OpResult::Byte(res.0));
                    },
                    1 => {
                        let res = (operand as u16).overflowing_add(extword);
                        overflow = res.1;
                        negative = (res.0 as i32) < 0;
                        zero = res.0 == 0;
                        handle.write(OpResult::Word(res.0));
                    },
                    2 => {
                        let extword2 = cpu.next_instruction();
                        let summand = ((extword as u32) << 16) + extword2 as u32;
                        let res = operand.overflowing_add(summand);
                        overflow = res.1;
                        negative = (res.0 as i32) < 0;
                        zero = res.0 == 0;
                        handle.write(OpResult::Long(res.0));
                    },
                    _ => panic!("Invalid operand size!")
                }
                cpu.set_ccr(CCR::V, overflow);
                cpu.set_ccr(CCR::N, negative);
                cpu.set_ccr(CCR::Z, zero);
                cpu.set_ccr(CCR::C, overflow);
                cpu.set_ccr(CCR::X, overflow);   
            }
            Self::ANDI { size, mode, earegister } => {}
            Self::CLR { size, mode, earegister } => {}
            Self::CMPI { size, mode, earegister } => {
                let operand = cpu.memory_handle(mode, earegister, 1 << size).read(1 << size);
                let extword = cpu.next_instruction();
                match operand {
                    OpResult::Byte(b) => {
                        let res = (b as i8).overflowing_sub((extword & 0xff) as i8);
                        cpu.set_ccr(CCR::N, res.0 < 0);
                        cpu.set_ccr(CCR::Z, res.0 == 0);
                        cpu.set_ccr(CCR::V, res.1);
                        cpu.set_ccr(CCR::C, res.1);
                    },
                    OpResult::Word(w) => {
                        let res = (w as i16).overflowing_sub(extword as i16);
                        cpu.set_ccr(CCR::N, res.0 < 0);
                        cpu.set_ccr(CCR::Z, res.0 == 0);
                        cpu.set_ccr(CCR::V, res.1);
                        cpu.set_ccr(CCR::C, res.1);
                    },
                    OpResult::Long(l) => {
                        let extword2 = cpu.next_instruction();
                        let sub = (((extword as u32) << 16) + extword2 as u32) as i32;
                        let res = (l as i32).overflowing_sub(sub);
                        cpu.set_ccr(CCR::N, res.0 < 0);
                        cpu.set_ccr(CCR::Z, res.0 == 0);
                        cpu.set_ccr(CCR::V, res.1);
                        cpu.set_ccr(CCR::C, res.1);
                    },
                }
            }
            Self::EORI { size, mode, earegister } => {}
            Self::NEG { size, mode, earegister } => {}
            Self::NEGX { size, mode, earegister } => {}
            Self::NOT { size, mode, earegister } => {}
            Self::ORI { size, mode, earegister } => {}
            Self::SUBI { size, mode, earegister } => {
                let extword = cpu.next_instruction();
                let handle = cpu.memory_handle(mode, earegister, 1 << size);
                let operand = handle.read(1 << size).inner();
                let overflow;
                let negative;
                let zero;
                match size {
                    0 => {
                        let subtrahend = (extword & 0x00ff) as u8;
                        let res = (operand as u8).overflowing_sub(subtrahend);
                        overflow = res.1;
                        negative = (res.0 as i32) < 0;
                        zero = res.0 == 0;
                        handle.write(OpResult::Byte(res.0));
                    },
                    1 => {
                        let res = (operand as u16).overflowing_sub(extword);
                        overflow = res.1;
                        negative = (res.0 as i32) < 0;
                        zero = res.0 == 0;
                        handle.write(OpResult::Word(res.0));
                    },
                    2 => {
                        let extword2 = cpu.next_instruction();
                        let subtrahend = ((extword as u32) << 16) + extword2 as u32;
                        let res = operand.overflowing_add(subtrahend);
                        overflow = res.1;
                        negative = (res.0 as i32) < 0;
                        zero = res.0 == 0;
                        handle.write(OpResult::Long(res.0));
                    },
                    _ => panic!("Invalid operand size!")
                }
                cpu.set_ccr(CCR::V, overflow);
                cpu.set_ccr(CCR::N, negative);
                cpu.set_ccr(CCR::Z, zero);
                cpu.set_ccr(CCR::C, overflow);
                cpu.set_ccr(CCR::X, overflow);
            }
            Self::TST { size, mode, earegister } => {}
            Self::BRA { displacement } => {
                if displacement == 0 {
                    let displacement_i16 = cpu.next_instruction() as i16;
                    cpu.pc = (cpu.pc as i32 + (displacement_i16 as i32) - 4) as u32;
                } else {
                    cpu.pc = (cpu.pc as i32 + (displacement as i8 as i32) - 2) as u32;
                }
            }
            Self::BSR { displacement } => {}
            Self::CMPM { ax, ay, size } => {}
            Self::ADDX { rx, ry, rm, size } => {}
            Self::SUBX { rx, ry, rm, size } => {}
            Self::BCHG { register, mode, earegister } => {}
            Self::BCLR { register, mode, earegister } => {}
            Self::BSET { register, mode, earegister } => {}
            Self::BTST { register, mode, earegister } => {}
            Self::DIVS { register, mode, earegister } => {}
            Self::DIVU { register, mode, earegister } => {}
            Self::LEA { register, mode, earegister } => {}
            Self::MULS { register, mode, earegister } => {}
            Self::MULU { register, mode, earegister } => {}
            Self::NBCD { register, mode, earegister } => {}
            Self::MOVEP { dregister, opmode, aregister } => {
                let oplength = 1 << ((opmode % 2) + 1);
                let mut ram_handle = cpu.memory_handle(5, aregister, 0);
                let mut result: u32 = 0;
                if (opmode - 4) / 2 == 0 {
                    if oplength == 2 {
                        result = ram_handle.read(2).inner();
                        ram_handle.offset(2);
                        result += ram_handle.read(2).inner() >> 8;
                        cpu.dr[dregister].as_ref().replace(result);
                    } else {
                        for j in 0..3 {
                            result += ram_handle.read(2).inner() << (16 - 8*j);
                            ram_handle.offset(2);
                        }
                        result += ram_handle.read(2).inner() >> 8;
                        cpu.dr[dregister].as_ref().replace(result);
                    }
                } else {
                    result = *cpu.dr[dregister].as_ref().borrow();
                    ram_handle.offset(6);
                    for _ in 0..oplength {
                        ram_handle.write(OpResult::Byte((result & 0xff) as u8));
                        result = result >> 8 ;
                        ram_handle.offset(-2);
                    }
                }
            }
            Self::SCC { condition, mode, earegister } => {}
            Self::ASLRREG { register, count, size, dr, lr } => {}
            Self::LSLRREG { register, count, size, dr, lr } => {}
            Self::ROXLR { register, count, size, dr, lr } => {}
            Self::ROLR { register, count, size, dr, lr } => {}
            Self::MOVEQ { register, data } => {
                cpu.dr[register].as_ref().replace((data & 0xff) as i8 as u32);
            }
            Self::EXG { mode, rx, ry } => {}
            Self::CHK { register, size, mode, earegister } => {}
            Self::MOVEA { register, size, mode, earegister } => {
                if size == 2 {
                    let src = cpu.memory_handle(mode, earegister, 4).read(4).inner();
                    let mut dest = cpu.ar[register].as_ref().borrow_mut();
                    *dest = src;
                } else if size == 3 {
                    let src = cpu.memory_handle(mode, earegister, 2).read(2).inner() as i16;
                    let mut dest = cpu.ar[register].as_ref().borrow_mut();
                    *dest = src as u32;
                }
            }
            Self::ADDQ { data, size, mode, earegister } => {}
            Self::SUBQ { data, size, mode, earegister } => {}
            Self::BCC { condition, displacement } => {}
            Self::ADD { register, opmode, mode, earegister } => {
                let bytesize = 1 << (opmode % 4);
                let drhandle = cpu.memory_handle(0, register, 0);
                let ophandle = cpu.memory_handle(mode, earegister, bytesize);
                let dr = drhandle.read(bytesize).inner();
                let op = ophandle.read(bytesize).inner();
                let result: OpResult;
                let overflow;
                let negative;
                let zero;
                match opmode % 4 {
                    0 => {
                        let res = (dr as u8).overflowing_add(op as u8);
                        result = OpResult::Byte(res.0);
                        overflow = res.1;
                        negative = (res.0 as i8) < 0;
                        zero = res.0 == 0;
                    }, 
                    1 => {
                        let res = (dr as u16).overflowing_add(op as u16);
                        result = OpResult::Word(res.0);
                        overflow = res.1;
                        negative = (res.0 as i16) < 0;
                        zero = res.0 == 0;
                    },
                    2 => {
                        let res = dr.overflowing_add(op);
                        result = OpResult::Long(res.0);
                        overflow = res.1;
                        negative = (res.0 as i32) < 0;
                        zero = res.0 == 0;
                    },
                    _ => panic!("Invalid Opmode!")
                }
                match opmode >> 2 {
                    0 => {
                        drhandle.write(result);
                    },
                    1 => {
                        ophandle.write(result);
                    },
                    _ => {}
                }
                cpu.set_ccr(CCR::V, overflow);
                cpu.set_ccr(CCR::N, negative);
                cpu.set_ccr(CCR::Z, zero);
                cpu.set_ccr(CCR::C, overflow);
                cpu.set_ccr(CCR::X, overflow);
            }
            Self::AND { register, opmode, mode, earegister } => {}
            Self::CMP { register, opmode, mode, earegister } => {}
            Self::EOR { register, opmode, mode, earegister } => {}
            Self::OR { register, opmode, mode, earegister } => {}
            Self::SUB { register, opmode, mode, earegister } => {}
            Self::MOVE { size, destreg, destmode, srcmode, srcreg } => {
                // #FIXME: update CCR
                let memsize = if size > 1 { 8 - 2 * size } else { size };
                let src = cpu.memory_handle(srcmode, srcreg, memsize);
                let dest = cpu.memory_handle(destmode, destreg, memsize);
                dest.write(src.read(memsize));
            }
        }
    }
    pub fn as_asm(&self, cpu: &CPU) -> String {
        match *self {
            Self::ANDICCR => format!("andi #${:04x},ccr", cpu.lookahead(0)),
            Self::ANDISR => format!("andi #${:04x},sr", cpu.lookahead(0)),
            Self::EORICCR => format!("eori #${:04x},ccr", cpu.lookahead(0)),
            Self::EORISR => format!("eori #${:04x},sr", cpu.lookahead(0)),
            Self::ILLEGAL => String::from("illegal"),
            Self::NOP => String::from("nop"),
            Self::ORICCR => format!("ori #${:04x},ccr", cpu.lookahead(0)),
            Self::ORISR => format!("ori #${:04x},sr", cpu.lookahead(0)),
            Self::RESET => String::from("reset"),
            Self::RTE => String::from("rte"),
            Self::RTR => String::from("rtr"),
            Self::RTS => String::from("rts"),
            Self::STOP => String::from("stop"),
            Self::TRAPV => String::from("trapv"),
            Self::LINK { register } => format!("link a{:},#${:04x}", register, cpu.lookahead(0)),
            Self::SWAP { register } => format!("swap d{:}", register),
            Self::UNLK { register } => format!("unlk a{:}", register),
            Self::TRAP { vector } => format!("trap #{:}", vector),
            Self::MOVEUSP { register, dr } => {
                if dr == 0 {
                    format!("move usp,a{:}", register)
                } else {
                    format!("move a{:},usp", register)
                }
            }
            Self::BCHGS { mode, earegister } => format!("bchgs {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::BCLRS { mode, earegister } => format!("bclrs {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::BSETS { mode, earegister } => format!("bsets {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::BTSTS { mode, earegister } => format!("btsts {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::JMP { mode, earegister } => format!("jmp {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::JSR { mode, earegister } => format!("jsr {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::MOVECCR { mode, earegister } => format!("move {:},ccr", addr_as_asm(mode, earegister, 0, cpu)),
            Self::MOVEFROMSR { mode, earegister } => format!("move sr,{:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::MOVETOSR { mode, earegister } => format!("move {:},ccr", addr_as_asm(mode, earegister, 0, cpu)),
            Self::PEA { mode, earegister } => format!("pea {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::TAS { mode, earegister } => format!("tas {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::EXT { mode, earegister } => format!("ext {:}", addr_as_asm(mode, earegister, 0, cpu)),
            Self::ASLRMEM { mode, earegister } => String::from("aslrmem"),
            Self::LSLRMEM { mode, earegister } => String::from("lslrmem"),
            Self::DBCC { condition, register } => String::from("dbcc"),
            Self::MOVEM { size, dr, mode, earegister } => {
                // // FIXME: Handle address register
                // let mut register_mask = cpu.next_instruction();
                // let oplength = 1 << (size + 1);
                // if dr == 0 {
                //     let mut tgt = cpu.memory_handle(mode, earegister, oplength);
                //     let mut result;
                //     // In Control and postincrement mode the mask order is A7..D0 (LSB first), reversed for predecrement
                //     if mode == 4 {
                //         register_mask = register_mask.reverse_bits();
                //         tgt.offset(-(oplength as isize));
                //     }
                //     for j in 0..16 {
                //         if register_mask & (1 << j) != 0 {
                //             let register;
                //             if j < 8 {
                //                 register = cpu.dr[j].as_ref().borrow();
                //             } else {
                //                 register = cpu.ar[j].as_ref().borrow();
                //             }
                //             if oplength == 2 {
                //                 result = OpResult::Word((*register & 0xffff) as u16)
                //             } else {
                //                 result = OpResult::Long(*register);
                //             }
                //             tgt.write(result);
                //             if mode == 4 {
                //                 tgt.offset(-(oplength as isize));
                //             } else {
                //                 tgt.offset(oplength as isize);
                //             }
                //         }
                //     }
                // } else if dr == 1 {
                //     let mut src = cpu.memory_handle(mode, earegister, oplength);
                //     let mut result;
                //     for j in 0..16 {
                //         if register_mask & (1 << j) != 0 {
                //             let mut register;
                //             if j < 8 {
                //                 register = cpu.dr[j].as_ref().borrow_mut();
                //             } else {
                //                 register = cpu.ar[j].as_ref().borrow_mut();
                //             }
                //             if oplength == 2 {
                //                 result = ((src.read(oplength).inner() & 0xffff) as i16) as u32
                //             } else {
                //                 result = src.read(oplength).inner()
                //             }
                //             *register = result;
                //             src.offset(oplength as isize);
                //         }
                //     }
                // }
                String::from("")
            }
            Self::ABCD { rx, ry, rm } => String::from("abcd"),
            Self::SBCD { rx, ry, rm } => String::from("sbcd"),
            Self::ADDI { size, mode, earegister } => format!("addi.{:} #${:x},{:}", size_as_asm(1 << size), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::ANDI { size, mode, earegister } => format!("andi.{:} #${:x},{:}", size_as_asm(1 << size), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::CLR { size, mode, earegister } => format!("clr.{:} {:}", size_as_asm(1 << size), addr_as_asm(mode, earegister, size, cpu)),
            Self::CMPI { size, mode, earegister } => format!("cmpi.{:} #${:x},{:}", size_as_asm(1 << size), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::EORI { size, mode, earegister } => format!("eori.{:} #${:x},{:}", size_as_asm(1 << size), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::NEG { size, mode, earegister } => format!("neg.{:} {:}", size_as_asm(1 << size), addr_as_asm(mode, earegister, size, cpu)),
            Self::NEGX { size, mode, earegister } => format!("negx.{:} {:}", size_as_asm(1 << size), addr_as_asm(mode, earegister, size, cpu)),
            Self::NOT { size, mode, earegister } => format!("not.{:} {:}", size_as_asm(1 << size), addr_as_asm(mode, earegister, size, cpu)),
            Self::ORI { size, mode, earegister } => format!("ori.{:} #${:x},{:}", size_as_asm(1 << size), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::SUBI { size, mode, earegister } => format!("subi.{:} #${:x},{:}", size_as_asm(1 << size), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::TST { size, mode, earegister } => format!("tst.{:} {:}", size_as_asm(1 << size), addr_as_asm(mode, earegister, size, cpu)),
            Self::BRA { displacement } => {
                let pc;
                if displacement == 0 {
                    let displacement_i16 = cpu.lookahead(0) as i16;
                    pc = (cpu.pc as i32 + (displacement_i16 as i32) - 2) as u32;
                } else {
                    pc = (cpu.pc as i32 + (displacement as i8 as i32) - 2) as u32;
                }
                format!("bra ${:08x}", pc)
            }
            Self::BSR { displacement } => String::from("bsr"),
            Self::CMPM { ax, ay, size } => String::from("cmpm"),
            Self::ADDX { rx, ry, rm, size } => String::from("addx"),
            Self::SUBX { rx, ry, rm, size } => String::from("subx"),
            Self::BCHG { register, mode, earegister } => String::from("bchg"),
            Self::BCLR { register, mode, earegister } => String::from("bclr"),
            Self::BSET { register, mode, earegister } => String::from("bset"),
            Self::BTST { register, mode, earegister } => String::from("btst"),
            Self::DIVS { register, mode, earegister } => String::from("divs"),
            Self::DIVU { register, mode, earegister } => String::from("divu"),
            Self::LEA { register, mode, earegister } => String::from("lea"),           
            Self::MULS { register, mode, earegister } => String::from("muls"),
            Self::MULU { register, mode, earegister } => String::from("mulu"),
            Self::NBCD { register, mode, earegister } => String::from("nbcd"),
            Self::MOVEP { dregister, opmode, aregister } => {
                let oplength = 1 << ((opmode % 2) + 1);
                if (opmode - 4) / 2 == 0 {
                    format!("movep.{:} (d16,a{:}),d{:}", size_as_asm(oplength), aregister, dregister)
                    
                } else {
                    format!("movep.{:} d{:},(d16,a{:})", size_as_asm(oplength), dregister, aregister)
                }
            }
            Self::SCC { condition, mode, earegister } => String::from("scc"),
            Self::ASLRREG { register, count, size, dr, lr } => String::from("aslrreg"),
            Self::LSLRREG { register, count, size, dr, lr } => String::from("lslrreg"),
            Self::ROXLR { register, count, size, dr, lr } => String::from("roxlr"),
            Self::ROLR { register, count, size, dr, lr } => String::from("rolr"),
            Self::MOVEQ { register, data } => format!("moveq #${:02x},d{:}", data, register),
            Self::EXG { mode, rx, ry } => String::from("exg"),
            Self::CHK { register, size, mode, earegister } => String::from("chk"),
            Self::MOVEA { register, size, mode, earegister } => format!("movea.{:} {:},a{:}", size_as_asm(8-2*size), addr_as_asm(mode, earegister, size, cpu), register),
            Self::ADDQ { data, size, mode, earegister } => String::from("addq"),
            Self::SUBQ { data, size, mode, earegister } => String::from("subq"),
            Self::BCC { condition, displacement } => String::from("bcc"),
            Self::ADD { register, opmode, mode, earegister } => {
                let bytesize = 1 << (opmode % 4);
                match opmode >> 2 {
                    0 => {
                        format!("add.{:} {:},d{:}", size_as_asm(bytesize), addr_as_asm(mode, earegister, bytesize, cpu), register)
                    },
                    _ => {
                        format!("add.{:} d{:},{:}", size_as_asm(bytesize), register, addr_as_asm(mode, earegister, bytesize, cpu))
                    }
                }
            }
            Self::AND { register, opmode, mode, earegister } => String::from("and"),
            Self::CMP { register, opmode, mode, earegister } => String::from("cmp"),
            Self::EOR { register, opmode, mode, earegister } => String::from("eor"),
            Self::OR { register, opmode, mode, earegister } => String::from("or"),
            Self::SUB { register, opmode, mode, earegister } => String::from("sub"),
            Self::MOVE { size, destreg, destmode, srcmode, srcreg } => {
                let memsize = if size > 1 { 8 - 2 * size } else { size };
                format!("move.{:} {:},{:}", size_as_asm(memsize), addr_as_asm(srcmode, srcreg, memsize, cpu), addr_as_asm(destmode, destreg, memsize, cpu))
            }
        }        
    }
}

fn privilege_violation(cpu: &mut CPU) {
    cpu.set_ccr(CCR::S, true);
    let mut ssp = cpu.ssp.as_ref().borrow_mut();
    *ssp -= 4;
    let mut ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
    ram_handle.write(OpResult::Long(cpu.pc));
    *ssp -= 2;
    ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
    ram_handle.write(OpResult::Word(cpu.sr));
    cpu.pc = 0x20;
}

fn size_as_asm(size: usize) -> String {
    match size {
        1 => String::from("b"),
        2 => String::from("w"),
        _ => String::from("l"),
    }
}

fn addr_as_asm(mode: usize, earegister: usize, size: usize, cpu: &CPU) -> String {
    match mode {
        // Data register direct mode
        0 => format!("d{:}", earegister),
        // Address register direct mode
        1 => format!("a{:}", earegister),
        // Address register indirect mode
        2 => format!("(a{:})", earegister),
        // Address register indirect with postincrement mode
        3 => format!("(a{:})+", earegister),
        // Address register indirect with predecrement mode
        4 => format!("-(a{:})", earegister),
        // Address register indirect with displacement mode
        5 => format!("{:x}(a{:})", cpu.lookahead(0) as i16, earegister),
        6 => {
            let opcode = cpu.lookahead(0);
            if let Some(extword) = parse_extension_word(opcode) {
                match extword {
                    // Address Register Indirect with Index (8-Bit Displacement) Mode
                    ExtensionWord::BEW { da, register: iregister, wl: _, scale, displacement } => {
                        let da_flag = if da == 0 { "d" } else { "a" };
                        let displ = (displacement & 0xff) as i8;
                        format!("({:x}a{:},{:}{:}.{:}*{:})", displ, earegister, da_flag, iregister, size_as_asm(size), scale)
                    }
                    // Address Register Indirect with Index (Base Displacement) Mode
                    ExtensionWord::FEW { da, register: iregister, wl: _, scale, bs: _, is: _, bdsize: _, iis: _ } => {
                        let da_flag = if da == 0 { "d" } else { "a" };
                        let mut displacement: u32 = 0;
                        let (bdsize, _) = extword.remaining_length();
                        for j in 0..bdsize {
                            displacement += (cpu.lookahead(j + 1) * (1 << (8 * (bdsize - j - 1)))) as u32;
                        }
                        format!("({:x}a{:},{:}{:}.{:}*{:})", displacement as i32, earegister, da_flag, iregister, size_as_asm(size), scale)
                    }
                }
            } else {
                panic!("Invalid extension word!")
            }
        }
        7 => {
            let extword = cpu.lookahead(0);
            match earegister {
                0 => {
                    // Absolute Short Addressing Mode
                    format!("({:04x}).w", extword)
                },
                1 => {
                    // Absolute Long Addressing Mode
                    let extword2 = cpu.lookahead(1);
                    let mut ptr = extword2 as usize;
                    ptr += (extword as usize) << 16;
                    format!("({:08x}).l", ptr)
                }
                2 => {
                    // Program Counter Indirect with Displacement Mode
                    format!("({:04x},pc", extword)
                },
                // 3 => {
                //     // Program Counter Indirect with Index (8-Bit Displacement) Mode
                //     // Program Counter Indirect with Index (Base Displacement) Mode
                //     // Program Counter Memory Indirect Preindexed Mode
                // },
                4 => {
                    // Immediate Data
                    match size {
                        1 => format!("#${:02x}", (extword & 0xff) as u8),
                        2 => format!("#${:04x}", extword),
                        4 => format!("#${:04x}{:04x}", extword, cpu.lookahead(1)),
                        _ => panic!("Invalid operand size!")
                    }
                }
                _ => panic!("Invalid register!"),
            }
        }
        _ => panic!("Invalid addressing mode!"),
    }
}
