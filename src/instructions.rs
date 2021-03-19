use std::rc::Rc;
use crate::{CPU, CCR};
use crate::memory::{OpResult, MemoryHandle};

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
    MOVEP { register: usize, mode: usize, earegister: usize },
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
            Self::ANDICCR => {}
            Self::ANDISR => {}
            Self::EORICCR => {}
            Self::EORISR => {}
            Self::ILLEGAL => {}
            Self::NOP => {}
            Self::ORICCR => {}
            Self::ORISR => {}
            Self::RESET => {}
            Self::RTE => {}
            Self::RTR => {}
            Self::RTS => {}
            Self::STOP => {}
            Self::TRAPV => {}
            Self::LINK { register } => {
                let displacement = cpu.next_instruction();
                let mut sp = cpu.ar[7].as_ref().borrow_mut();
                *sp -= 2;
                let mut ar = cpu.ar[register].as_ref().borrow_mut();
                let ram_handle = MemoryHandle { reg: None, ptr: Some(*ar as usize), mem: Some(Rc::clone(&cpu.ram)) };
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
            Self::UNLK { register } => {}
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
            Self::MOVEUSP { register, dr } => {}
            Self::BCHGS { mode, earegister } => {}
            Self::BCLRS { mode, earegister } => {}
            Self::BSETS { mode, earegister } => {}
            Self::BTSTS { mode, earegister } => {}
            Self::JMP { mode, earegister } => {}
            Self::JSR { mode, earegister } => {}
            Self::MOVECCR { mode, earegister } => {}
            Self::MOVEFROMSR { mode, earegister } => {}
            Self::MOVETOSR { mode, earegister } => {}
            Self::PEA { mode, earegister } => {}
            Self::TAS { mode, earegister } => {}
            Self::EXT { mode, earegister } => {}
            Self::ASLRMEM { mode, earegister } => {}
            Self::LSLRMEM { mode, earegister } => {}
            Self::DBCC { condition, register } => {}
            Self::MOVEM { size, dr, mode, earegister } => {}
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
                        handle.write(OpResult::Long(operand + summand));
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
            Self::CMPI { size, mode, earegister } => {}
            Self::EORI { size, mode, earegister } => {}
            Self::NEG { size, mode, earegister } => {}
            Self::NEGX { size, mode, earegister } => {}
            Self::NOT { size, mode, earegister } => {}
            Self::ORI { size, mode, earegister } => {}
            Self::SUBI { size, mode, earegister } => {}
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
            Self::MOVEP { register, mode, earegister } => {}
            Self::SCC { condition, mode, earegister } => {}
            Self::ASLRREG { register, count, size, dr, lr } => {}
            Self::LSLRREG { register, count, size, dr, lr } => {}
            Self::ROXLR { register, count, size, dr, lr } => {}
            Self::ROLR { register, count, size, dr, lr } => {}
            Self::MOVEQ { register, data } => {}
            Self::EXG { mode, rx, ry } => {}
            Self::CHK { register, size, mode, earegister } => {}
            Self::MOVEA { register, size, mode, earegister } => {}
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
                let mut memsize = size;
                if size == 2 {
                    memsize = 4;
                } else if size == 3 {
                    memsize = 2;
                }
                let src = cpu.memory_handle(srcmode, srcreg, memsize);
                let dest = cpu.memory_handle(destmode, destreg, memsize);
                dest.write(src.read(memsize));
            }
        }
    }
}
