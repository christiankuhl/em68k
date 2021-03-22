use std::rc::Rc;
use crate::processor::{CPU, CCR, CCRFlags, set_bit, get_bit};
use crate::memory::{OpResult, MemoryHandle, Size};
use crate::parser::parse_extension_word;

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
    EXT { opmode: usize, register: usize },
    ASLRMEM { dr: usize, mode: usize, earegister: usize },
    LSLRMEM { dr: usize, mode: usize, earegister: usize },
    DBCC { condition: Condition, register: usize },
    MOVEM { size: Size, dr: usize, mode: usize, earegister: usize },
    ABCD { rx: usize, ry: usize, rm: usize },
    SBCD { rx: usize, ry: usize, rm: usize },
    ADDI { size: Size, mode: usize, earegister: usize },
    ANDI { size: Size, mode: usize, earegister: usize },
    CLR { size: Size, mode: usize, earegister: usize },
    CMPI { size: Size, mode: usize, earegister: usize },
    EORI { size: Size, mode: usize, earegister: usize },
    NEG { size: Size, mode: usize, earegister: usize },
    NEGX { size: Size, mode: usize, earegister: usize },
    NOT { size: Size, mode: usize, earegister: usize },
    ORI { size: Size, mode: usize, earegister: usize },
    SUBI { size: Size, mode: usize, earegister: usize },
    TST { size: Size, mode: usize, earegister: usize },
    BRA { displacement: usize },
    BSR { displacement: usize },
    CMPM { ax: usize, ay: usize, size: Size },
    ADDX { rx: usize, ry: usize, rm: usize, size: Size },
    SUBX { rx: usize, ry: usize, rm: usize, size: Size },
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
    SCC { condition: Condition, mode: usize, earegister: usize },
    ASLRREG { register: usize, count: usize, size: Size, dr: usize, ir: usize },
    LSLRREG { register: usize, count: usize, size: Size, dr: usize, ir: usize },
    ROXLR { register: usize, count: usize, size: Size, dr: usize, ir: usize },
    ROLR { register: usize, count: usize, size: Size, dr: usize, ir: usize },
    MOVEQ { register: usize, data: usize },
    EXG { mode: usize, rx: usize, ry: usize },
    CHK { register: usize, size: Size, mode: usize, earegister: usize },
    MOVEA { register: usize, size: Size, mode: usize, earegister: usize },
    ADDQ { data: usize, size: Size, mode: usize, earegister: usize },
    SUBQ { data: usize, size: Size, mode: usize, earegister: usize },
    BCC { condition: Condition, displacement: usize },
    ADD { register: usize, opmode: usize, mode: usize, earegister: usize },
    AND { register: usize, opmode: usize, mode: usize, earegister: usize },
    CMP { register: usize, opmode: usize, mode: usize, earegister: usize },
    EOR { register: usize, opmode: usize, mode: usize, earegister: usize },
    OR { register: usize, opmode: usize, mode: usize, earegister: usize },
    SUB { register: usize, opmode: usize, mode: usize, earegister: usize },
    MOVE { size: Size, destreg: usize, destmode: usize, srcmode: usize, srcreg: usize },
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
                cpu.sr &= (0xff00 | extword) as u32;
            }
            Self::ANDISR => {
                cpu.sr &= cpu.next_instruction() as u32;
            }
            Self::EORICCR => {
                let extword = cpu.next_instruction() as u32;
                cpu.sr ^= 0x001f & extword;
            }
            Self::EORISR => {
                cpu.sr ^= cpu.next_instruction() as u32;
            }
            Self::ILLEGAL => {
                let trap = Self::TRAP { vector: 4 };
                trap.execute(cpu);
            }
            Self::NOP => {}
            Self::ORICCR => {
                let extword = cpu.next_instruction();
                cpu.sr |= (0x001f & extword) as u32;
            }
            Self::ORISR => {
                cpu.sr |= cpu.next_instruction() as u32;
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
                    cpu.sr = ram_handle.read(2).inner();
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
                cpu.sr |= ccr as u32;
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
                    cpu.sr = extword as u32;
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
                let ccr = CCRFlags {
                    c: Some(false),
                    v: Some(false),
                    z: Some(res == 0),
                    n: Some(res & (1 << 31) > 0),
                    x: None
                };
                ccr.set(cpu);
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
                cpu.supervisor_mode(true);
                let mut ssp = cpu.ssp.as_ref().borrow_mut();
                *ssp -= 4;
                let mut ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                ram_handle.write(OpResult::Long(cpu.pc));
                *ssp -= 2;
                ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                ram_handle.write(OpResult::Word((cpu.sr & 0xffff) as u16));
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
            Self::BCHGS { mode, earegister } => {
                change_bit(mode, earegister, None, cpu, Mode::Flip);
            }
            Self::BCLRS { mode, earegister } => {
                change_bit(mode, earegister, None, cpu, Mode::Clear);
            }
            Self::BSETS { mode, earegister } => {
                change_bit(mode, earegister, None, cpu, Mode::Set);
            }
            Self::BTSTS { mode, earegister } => {
                change_bit(mode, earegister, None, cpu, Mode::None);
            }
            Self::JMP { mode, earegister } => {
                let addr = cpu.memory_address(mode, earegister);
                cpu.pc = addr - 2;
            }
            Self::JSR { mode, earegister } => {
                cpu.pc = cpu.memory_address(mode, earegister) - 2;
                let mut sp = cpu.ar[7].as_ref().borrow_mut();
                *sp -= 4;
                let ram_handle = MemoryHandle { reg: None, ptr: Some(*sp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                ram_handle.write(OpResult::Long(cpu.pc + 2));
            }
            Self::MOVECCR { mode, earegister } => {
                let src = cpu.memory_handle(mode, earegister, Size::Word).read(2).inner();
                cpu.sr &= 0xff00;
                cpu.sr |= src;
            }
            Self::MOVEFROMSR { mode, earegister } => {
                let dest = cpu.memory_handle(mode, earegister, Size::Word);
                dest.write(OpResult::Word((cpu.sr & 0x8e0) as u16));
            }
            Self::MOVETOSR { mode, earegister } => {
                let src = cpu.memory_handle(mode, earegister, Size::Word).read(2).inner();
                cpu.sr = src & 0x8e0;
            }
            Self::PEA { mode, earegister } => {
                let addr = cpu.memory_address(mode, earegister);
                let mut sp = cpu.ar[7].as_ref().borrow_mut();
                *sp -= 4;
                let ram_handle = MemoryHandle { reg: None, ptr: Some(*sp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                ram_handle.write(OpResult::Long(cpu.pc + 2));
            }
            Self::TAS { mode, earegister } => {
                let handle = cpu.memory_handle(mode, earegister, Size::Byte);
                let mut operand = (handle.read(1).inner() & 0xff) as u8;
                let ccr = CCRFlags { x: None, 
                    n: Some(operand & (1 << 7) != 0), 
                    z: Some(operand == 0),
                    v: Some(false),
                    c: Some(false),
                };
                ccr.set(cpu);
                operand |= 1 << 7;
                handle.write(OpResult::Byte(operand));     
            }
            Self::EXT { opmode, register } => {
                let mut ccr = CCRFlags { x: None, n: None, z: None, v: Some(false), c: Some(false) };
                {
                    let mut reg = cpu.dr[register].as_ref().borrow_mut();
                    if opmode == 2 {
                        let lower = (*reg & 0xff) as i8;
                        *reg &= 0xffff0000;
                        *reg += ((lower as u16) & 0xffff) as u32;
                        ccr.z = Some(lower == 0);
                        ccr.n = Some(lower < 0);
                    } else {
                        let lower = (*reg & 0xffff) as i16;
                        *reg = lower as u32;
                        ccr.z = Some(lower == 0);
                        ccr.n = Some(lower < 0);
                    }
                }
                ccr.set(cpu);
            }
            Self::ASLRMEM { dr, mode, earegister } => {
                let handle = cpu.memory_handle(mode, earegister, Size::Word);
                let mut ccr = CCRFlags::new();
                let mut value = handle.read(2).inner() as i16;
                let msb = get_bit(value as usize, 15);
                if dr == 0 {
                    let lsb = get_bit(value as usize, 0);
                    value = value >> 1;
                    ccr.c = Some(lsb);
                    ccr.x = Some(lsb);
                } else {
                    value = value << 1;
                    ccr.c = Some(msb);
                    ccr.x = Some(msb);
                }
                handle.write(OpResult::Word(value as u16));
                let new_msb = get_bit(value as usize, 15);
                ccr.z = Some(value == 0);
                ccr.n = Some(value < 0);
                ccr.v = Some(new_msb != msb);
                ccr.set(cpu);
            }
            Self::LSLRMEM { dr, mode, earegister } => {
                let handle = cpu.memory_handle(mode, earegister, Size::Word);
                let mut ccr = CCRFlags::new();
                let mut value = handle.read(2).inner() as u16;
                let msb;
                if dr == 0 {
                    let lsb = get_bit(value as usize, 0);
                    value = value >> 1;
                    ccr.c = Some(lsb);
                    ccr.x = Some(lsb);
                } else {
                    msb = get_bit(value as usize, 15);
                    value = value << 1;
                    ccr.c = Some(msb);
                    ccr.x = Some(msb);
                }
                handle.write(OpResult::Word(value as u16));
                let new_msb = get_bit(value as usize, 15);
                ccr.z = Some(value == 0);
                ccr.n = Some((value as i16) < 0);
                ccr.v = Some(false);
                ccr.set(cpu);
            }
            Self::DBCC { condition, register } => {
                let displacement = cpu.next_instruction() as i32;
                let counter_reg = cpu.memory_handle(0, register, Size::Byte);
                let mut counter = counter_reg.read(1).inner() as i8;
                if !condition.evaluate(cpu) {
                    counter -= 1;
                    counter_reg.write(OpResult::Byte(counter as u8));
                    if counter != -1 {
                        cpu.pc = (cpu.pc as i32 + displacement - 2) as u32;
                    }
                } 
            }
            Self::MOVEM { size, dr, mode, earegister } => {
                // FIXME: Handle address register
                let mut register_mask = cpu.next_instruction();
                let oplength = size as usize;
                if dr == 0 {
                    let mut tgt = cpu.memory_handle(mode, earegister, size);
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
                    let mut src = cpu.memory_handle(mode, earegister, size);
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
                let handle = cpu.memory_handle(mode, earegister, size);
                let operand = handle.read(size as usize);
                let summand = cpu.immediate_operand(size);
                let res = operand.add(summand);
                let result = res.0;
                let ccr = res.1;
                handle.write(result);
                ccr.set(cpu);
            }
            Self::ANDI { size, mode, earegister } => {
                let dest = cpu.memory_handle(mode, earegister, size);
                let operand = dest.read(size as usize);
                let src = cpu.immediate_operand(size);
                let res = src.and(operand);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::CLR { size, mode, earegister } => {
                let dest = cpu.memory_handle(mode, earegister, size);
                let res = dest.read(size as usize).clear();
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::CMPI { size, mode, earegister } => {
                let operand = cpu.memory_handle(mode, earegister, size).read(size as usize);
                let src = cpu.immediate_operand(size);
                let res = operand.sub(src);
                let ccr = res.1;
                ccr.set(cpu);
            }
            Self::EORI { size, mode, earegister } => {
                let dest = cpu.memory_handle(mode, earegister, size);
                let operand = dest.read(size as usize);
                let src = cpu.immediate_operand(size);
                let res = src.xor(operand);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::NEG { size, mode, earegister } => {
                let handle = cpu.memory_handle(mode, earegister, size);
                let operand = handle.read(size as usize);
                let res = size.zero().sub(operand);
                let result = res.0;
                let ccr = res.1;
                handle.write(result);
                ccr.set(cpu);
            }
            Self::NEGX { size, mode, earegister } => {}
            Self::NOT { size, mode, earegister } => {}
            Self::ORI { size, mode, earegister } => {
                let dest = cpu.memory_handle(mode, earegister, size);
                let operand = dest.read(size as usize);
                let src = cpu.immediate_operand(size);
                let res = src.or(operand);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::SUBI { size, mode, earegister } => {
                let handle = cpu.memory_handle(mode, earegister, size);
                let operand = handle.read(size as usize);
                let subtrahend = cpu.immediate_operand(size);
                let res = operand.sub(subtrahend);
                let result = res.0;
                let ccr = res.1;
                handle.write(result);
                ccr.set(cpu);
            }
            Self::TST { size, mode, earegister } => {
                let operand = cpu.memory_handle(mode, earegister, size).read(size as usize).inner();
                let mut ccr = CCRFlags::new();
                ccr.n = Some((operand as i32) < 0);
                ccr.z = Some(operand == 0);
                ccr.v = Some(false);
                ccr.c = Some(false);
                ccr.set(cpu);
            }
            Self::BRA { displacement } => {
                cpu.pc = if displacement == 0 {
                    let displacement_i16 = cpu.next_instruction() as i16;
                    (cpu.pc as i32 + (displacement_i16 as i32) - 4) as u32 + 2
                } else {
                    (cpu.pc as i32 + (displacement as i8 as i32) - 2) as u32 + 2
                }
            }
            Self::BSR { displacement } => {
                let pc = if displacement == 0 {
                    let displacement_i16 = cpu.next_instruction() as i16;
                    (cpu.pc as i32 + (displacement_i16 as i32) - 4) as u32 + 2
                } else {
                    (cpu.pc as i32 + (displacement as i8 as i32) - 2) as u32 + 2
                };
                let mut sp = cpu.ar[7].as_ref().borrow_mut();
                *sp -= 4;
                let ram_handle = MemoryHandle { reg: None, ptr: Some(*sp as usize), mem: Some(Rc::clone(&cpu.ram)) };
                ram_handle.write(OpResult::Long(cpu.pc + 2));
                cpu.pc = pc;
            }
            Self::CMPM { ax, ay, size } => {
                let src = cpu.memory_handle(3, ay, size).read(size as usize);
                let dest = cpu.memory_handle(3, ax, size);
                let res = dest.read(size as usize).sub(src);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::ADDX { rx, ry, rm, size } => {
                let src;
                let dest;
                if rm == 0 {
                    src = cpu.memory_handle(0, ry, size);
                    dest = cpu.memory_handle(0, rx, size); 
                } else {
                    src = cpu.memory_handle(4, ry, size);
                    dest = cpu.memory_handle(4, rx, size);
                }
                let x = cpu.ccr(CCR::X);
                let operand = match src.read(size as usize) {
                    OpResult::Byte(op) => OpResult::Byte((op.wrapping_add(x as u8) & 0xff) as u8),
                    OpResult::Word(op) => OpResult::Word((op.wrapping_add(x as u16) & 0xffff) as u16),
                    OpResult::Long(op) => OpResult::Long(op.wrapping_add(x as u32))
                };
                let res = dest.read(size as usize).add(operand);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::SUBX { rx, ry, rm, size } => {
                let src;
                let dest;
                if rm == 0 {
                    src = cpu.memory_handle(0, ry, size);
                    dest = cpu.memory_handle(0, rx, size); 
                } else {
                    src = cpu.memory_handle(4, ry, size);
                    dest = cpu.memory_handle(4, rx, size);
                }
                let x = cpu.ccr(CCR::X);
                let operand = match src.read(size as usize) {
                    OpResult::Byte(op) => OpResult::Byte((op.wrapping_sub(x as u8) & 0xff) as u8),
                    OpResult::Word(op) => OpResult::Word((op.wrapping_sub(x as u16) & 0xffff) as u16),
                    OpResult::Long(op) => OpResult::Long(op.wrapping_sub(x as u32))
                };
                let res = dest.read(size as usize).sub(operand);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::BCHG { register, mode, earegister } => {
                change_bit(mode, earegister, Some(register), cpu, Mode::Flip);
            }
            Self::BCLR { register, mode, earegister } => {
                change_bit(mode, earegister, Some(register), cpu, Mode::Clear);
            }
            Self::BSET { register, mode, earegister } => {
                change_bit(mode, earegister, Some(register), cpu, Mode::Set);
            }
            Self::BTST { register, mode, earegister } => {
                change_bit(mode, earegister, Some(register), cpu, Mode::None);
            }
            Self::DIVS { register, mode, earegister } => {}
            Self::DIVU { register, mode, earegister } => {}
            Self::LEA { register, mode, earegister } => {
                let addr = cpu.memory_handle(mode, earegister, Size::Long).read(4).inner();
                let mut addrreg = cpu.ar[register].as_ref().borrow_mut();
                *addrreg = addr;
            }
            Self::MULS { register, mode, earegister } => {}
            Self::MULU { register, mode, earegister } => {}
            Self::NBCD { register, mode, earegister } => {}
            Self::MOVEP { dregister, opmode, aregister } => {
                let oplength = 1 << ((opmode % 2) + 1);
                let mut ram_handle = cpu.memory_handle(5, aregister, Size::Byte);
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
            Self::SCC { condition, mode, earegister } => {
                let dest = cpu.memory_handle(mode, earegister, Size::Byte);
                if condition.evaluate(cpu) {
                    dest.write(OpResult::Byte(0xff));
                } else {
                    dest.write(OpResult::Byte(0));
                }
            }
            Self::ASLRREG { register, count, size, dr, ir } => {
                let shift_count = if ir == 0 {
                    ((count - 1) % 8) + 1
                } else {
                    (*cpu.dr[count].as_ref().borrow() % 64) as usize
                };
                let bitsize = 8 * size as usize;
                let handle = cpu.memory_handle(0, register, size);
                let mut ccr = CCRFlags::new();
                let mut value = handle.read(size as usize).inner() as i32;
                let msb = get_bit(value as usize, bitsize - 1);
                let mut new_msb;
                let mut lsb;
                let mut msb_changed = false;
                if dr == 0 {
                    for _ in 0..shift_count {
                        new_msb = get_bit(value as usize, bitsize - 1);
                        lsb = get_bit(value as usize, 0);
                        value = value >> 1;
                        ccr.c = Some(lsb);
                        ccr.x = Some(lsb);
                        if new_msb != msb {
                            msb_changed = true;
                        }
                    }
                } else {
                    for _ in 0..shift_count {
                        new_msb = get_bit(value as usize, bitsize - 1);
                        value = value << 1;
                        ccr.c = Some(new_msb);
                        ccr.x = Some(new_msb);
                        if new_msb != msb {
                            msb_changed = true;
                        }
                    }
                }
                match size {
                    Size::Byte => handle.write(OpResult::Word((value & 0xff) as u16)),
                    Size::Word => handle.write(OpResult::Word((value & 0xffff) as u16)),
                    Size::Long => handle.write(OpResult::Long(value as u32))
                }
                ccr.z = Some(value == 0);
                ccr.n = Some(value < 0);
                ccr.v = Some(msb_changed);
                ccr.set(cpu);
            }
            Self::LSLRREG { register, count, size, dr, ir } => {
                let shift_count = if ir == 0 {
                    ((count - 1) % 8) + 1
                } else {
                    (*cpu.dr[count].as_ref().borrow() % 64) as usize
                };
                let bitsize = 8 * size as usize;
                let handle = cpu.memory_handle(0, register, size);
                let mut ccr = CCRFlags::new();
                let mut value = handle.read(size as usize).inner() as u32;
                let msb = get_bit(value as usize, bitsize - 1);
                let mut new_msb;
                let mut lsb;
                let mut msb_changed = false;
                if dr == 0 {
                    for _ in 0..shift_count {
                        new_msb = get_bit(value as usize, bitsize - 1);
                        lsb = get_bit(value as usize, 0);
                        value = value >> 1;
                        ccr.c = Some(lsb);
                        ccr.x = Some(lsb);
                        if new_msb != msb {
                            msb_changed = true;
                        }
                    }
                } else {
                    for _ in 0..shift_count {
                        new_msb = get_bit(value as usize, bitsize - 1);
                        value = value << 1;
                        ccr.c = Some(new_msb);
                        ccr.x = Some(new_msb);
                        if new_msb != msb {
                            msb_changed = true;
                        }
                    }
                }
                match size {
                    Size::Byte => handle.write(OpResult::Word((value & 0xff) as u16)),
                    Size::Word => handle.write(OpResult::Word((value & 0xffff) as u16)),
                    Size::Long => handle.write(OpResult::Long(value))
                }
                ccr.z = Some(value == 0);
                ccr.n = Some(msb);
                ccr.v = Some(msb_changed);
                ccr.set(cpu);
            }
            Self::ROXLR { register, count, size, dr, ir } => {}
            Self::ROLR { register, count, size, dr, ir } => {}
            Self::MOVEQ { register, data } => {
                cpu.dr[register].as_ref().replace((data & 0xff) as i8 as u32);
            }
            Self::EXG { mode, rx, ry } => {}
            Self::CHK { register, size, mode, earegister } => {}
            Self::MOVEA { register, size, mode, earegister } => {
                match size {
                    Size::Long => {
                        let src = cpu.memory_handle(mode, earegister, size).read(4).inner();
                        let mut dest = cpu.ar[register].as_ref().borrow_mut();
                        *dest = src;
                    } 
                    Size::Word => {
                        let src = cpu.memory_handle(mode, earegister, size).read(2).inner() as i16;
                        let mut dest = cpu.ar[register].as_ref().borrow_mut();
                        *dest = src as u32;
                    }
                    _ => panic!("Invalid operand size!")
                } 
            }
            Self::ADDQ { data, size, mode, earegister } => {}
            Self::SUBQ { data, size, mode, earegister } => {}
            Self::BCC { condition, displacement } => {
                let pc = if displacement == 0 {
                        let displacement_i16 = cpu.next_instruction() as i16;
                        (cpu.pc as i32 + (displacement_i16 as i32) - 4) as u32 + 2
                    } else {
                        (cpu.pc as i32 + (displacement as i8 as i32) - 2) as u32 + 2
                    };
                if condition.evaluate(cpu) {
                    cpu.pc = pc;
                } 
            }
            Self::ADD { register, opmode, mode, earegister } => {
                let bytesize = Size::from(opmode % 4);
                let drhandle = cpu.memory_handle(0, register, Size::Byte);
                let ophandle = cpu.memory_handle(mode, earegister, bytesize);
                let dr = drhandle.read(bytesize as usize);
                let op = ophandle.read(bytesize as usize);
                let res = dr.add(op);
                let ccr = res.1;
                let result = res.0;
                match opmode >> 2 {
                    0 => {
                        drhandle.write(result);
                    },
                    1 => {
                        ophandle.write(result);
                    },
                    _ => {}
                }
                ccr.set(cpu);
            }
            Self::AND { register, opmode, mode, earegister } => {}
            Self::CMP { register, opmode, mode, earegister } => {}
            Self::EOR { register, opmode, mode, earegister } => {}
            Self::OR { register, opmode, mode, earegister } => {}
            Self::SUB { register, opmode, mode, earegister } => {}
            Self::MOVE { size, destreg, destmode, srcmode, srcreg } => {
                let src = cpu.memory_handle(srcmode, srcreg, size);
                let dest = cpu.memory_handle(destmode, destreg, size);
                let result = src.read(size as usize);
                dest.write(result);
                let ccr = CCRFlags { c: Some(false), 
                                     v: Some(false), 
                                     z: Some(result.inner() == 0), 
                                     n: Some(result.sign_extend() < 0),
                                     x: None };
                ccr.set(cpu);
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
            Self::BCHGS { mode, earegister } => format!("bchg #{:},{:}", cpu.lookahead(1), addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::BCLRS { mode, earegister } => format!("bclr #{:},{:}", cpu.lookahead(1), addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::BSETS { mode, earegister } => format!("bset #{:},{:}", cpu.lookahead(1), addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::BTSTS { mode, earegister } => format!("btst #{:},{:}", cpu.lookahead(1), addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::JMP { mode, earegister } => format!("jmp {:}", addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::JSR { mode, earegister } => format!("jsr {:}", addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::MOVECCR { mode, earegister } => format!("move {:},ccr", addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::MOVEFROMSR { mode, earegister } => format!("move sr,{:}", addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::MOVETOSR { mode, earegister } => format!("move {:},ccr", addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::PEA { mode, earegister } => format!("pea {:}", addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::TAS { mode, earegister } => format!("tas {:}", addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::EXT { opmode, register } => format!("ext.{:} d{:}", if opmode == 2 { "w" } else { "l" }, register),
            Self::ASLRMEM { dr, mode, earegister } => format!("as{:} {:}", if dr == 0 { "r" } else { "l" }, addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::LSLRMEM { dr, mode, earegister } => format!("ls{:} {:}", if dr == 0 { "r" } else { "l" }, addr_as_asm(mode, earegister, Size::Byte, cpu)),
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
            Self::ADDI { size, mode, earegister } => format!("addi.{:} #${:x},{:}", size.as_asm(), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::ANDI { size, mode, earegister } => format!("andi.{:} #${:x},{:}", size.as_asm(), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::CLR { size, mode, earegister } => format!("clr.{:} {:}", size.as_asm(), addr_as_asm(mode, earegister, size, cpu)),
            Self::CMPI { size, mode, earegister } => format!("cmpi.{:} #${:x},{:}", size.as_asm(), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::EORI { size, mode, earegister } => format!("eori.{:} #${:x},{:}", size.as_asm(), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::NEG { size, mode, earegister } => format!("neg.{:} {:}", size.as_asm(), addr_as_asm(mode, earegister, size, cpu)),
            Self::NEGX { size, mode, earegister } => format!("negx.{:} {:}", size.as_asm(), addr_as_asm(mode, earegister, size, cpu)),
            Self::NOT { size, mode, earegister } => format!("not.{:} {:}", size.as_asm(), addr_as_asm(mode, earegister, size, cpu)),
            Self::ORI { size, mode, earegister } => format!("ori.{:} #${:x},{:}", size.as_asm(), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::SUBI { size, mode, earegister } => format!("subi.{:} #${:x},{:}", size.as_asm(), cpu.lookahead(0), addr_as_asm(mode, earegister, size, cpu)),
            Self::TST { size, mode, earegister } => format!("tst.{:} {:}", size.as_asm(), addr_as_asm(mode, earegister, size, cpu)),
            Self::BRA { displacement } => {
                let pc = if displacement == 0 {
                    let displacement_i16 = cpu.lookahead(0) as i16;
                    (cpu.pc as i32 + (displacement_i16 as i32) - 2) as u32 + 2
                } else {
                    (cpu.pc as i32 + (displacement as i8 as i32) - 2) as u32 + 2
                };
                format!("bra ${:08x}", pc)
            }
            Self::BSR { displacement } => String::from("bsr"),
            Self::CMPM { ax, ay, size } => String::from("cmpm"),
            Self::ADDX { rx, ry, rm, size } => String::from("addx"),
            Self::SUBX { rx, ry, rm, size } => String::from("subx"),
            Self::BCHG { register, mode, earegister } => format!("bchg d{:},{:}", register, addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::BCLR { register, mode, earegister } => format!("bclr d{:},{:}", register, addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::BSET { register, mode, earegister } => format!("bset d{:},{:}", register, addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::BTST { register, mode, earegister } => format!("btst d{:},{:}", register, addr_as_asm(mode, earegister, Size::Byte, cpu)),
            Self::DIVS { register, mode, earegister } => String::from("divs"),
            Self::DIVU { register, mode, earegister } => String::from("divu"),
            Self::LEA { register, mode, earegister } => String::from("lea"),           
            Self::MULS { register, mode, earegister } => String::from("muls"),
            Self::MULU { register, mode, earegister } => String::from("mulu"),
            Self::NBCD { register, mode, earegister } => String::from("nbcd"),
            Self::MOVEP { dregister, opmode, aregister } => {
                let oplength = Size::from((opmode % 2) + 1);
                if (opmode - 4) / 2 == 0 {
                    format!("movep.{:} (d16,a{:}),d{:}", oplength.as_asm(), aregister, dregister)
                    
                } else {
                    format!("movep.{:} d{:},(d16,a{:})", oplength.as_asm(), dregister, aregister)
                }
            }
            Self::SCC { condition, mode, earegister } => String::from("scc"),
            Self::ASLRREG { register, count, size, dr, ir } => String::from("aslrreg"),
            Self::LSLRREG { register, count, size, dr, ir } => String::from("lslrreg"),
            Self::ROXLR { register, count, size, dr, ir } => String::from("roxlr"),
            Self::ROLR { register, count, size, dr, ir } => String::from("rolr"),
            Self::MOVEQ { register, data } => format!("moveq #${:02x},d{:}", data, register),
            Self::EXG { mode, rx, ry } => String::from("exg"),
            Self::CHK { register, size, mode, earegister } => String::from("chk"),
            Self::MOVEA { register, size, mode, earegister } => format!("movea.{:} {:},a{:}", size.as_asm(), addr_as_asm(mode, earegister, size, cpu), register),
            Self::ADDQ { data, size, mode, earegister } => String::from("addq"),
            Self::SUBQ { data, size, mode, earegister } => String::from("subq"),
            Self::BCC { condition, displacement } => {
                let pc = if displacement == 0 {
                        let displacement_i16 = cpu.lookahead(0) as i16;
                        (cpu.pc as i32 + (displacement_i16 as i32) - 2) as u32 + 2
                    } else {
                        (cpu.pc as i32 + (displacement as i8 as i32) - 2) as u32 + 2
                    };
                format!("b{:} ${:08x}", condition.as_asm(), pc)
            },
            Self::ADD { register, opmode, mode, earegister } => {
                let bytesize = Size::from(opmode % 4);
                match opmode >> 2 {
                    0 => {
                        format!("add.{:} {:},d{:}", bytesize.as_asm(), addr_as_asm(mode, earegister, bytesize, cpu), register)
                    },
                    _ => {
                        format!("add.{:} d{:},{:}", bytesize.as_asm(), register, addr_as_asm(mode, earegister, bytesize, cpu))
                    }
                }
            }
            Self::AND { register, opmode, mode, earegister } => String::from("and"),
            Self::CMP { register, opmode, mode, earegister } => String::from("cmp"),
            Self::EOR { register, opmode, mode, earegister } => String::from("eor"),
            Self::OR { register, opmode, mode, earegister } => String::from("or"),
            Self::SUB { register, opmode, mode, earegister } => String::from("sub"),
            Self::MOVE { size, destreg, destmode, srcmode, srcreg } => {
                format!("move.{:} {:},{:}", size.as_asm(), addr_as_asm(srcmode, srcreg, size, cpu), addr_as_asm(destmode, destreg, size, cpu))
            }
        }        
    }
}

fn privilege_violation(cpu: &mut CPU) {
    cpu.supervisor_mode(true);
    let mut ssp = cpu.ssp.as_ref().borrow_mut();
    *ssp -= 4;
    let mut ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
    ram_handle.write(OpResult::Long(cpu.pc));
    *ssp -= 2;
    ram_handle = MemoryHandle { reg: None, ptr: Some(*ssp as usize), mem: Some(Rc::clone(&cpu.ram)) };
    ram_handle.write(OpResult::Word((cpu.sr & 0xffff) as u16));
    cpu.pc = 0x20;
}

fn addr_as_asm(mode: usize, earegister: usize, size: Size, cpu: &CPU) -> String {
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
                        format!("({:x}a{:},{:}{:}.{:}*{:})", displ, earegister, da_flag, iregister, size.as_asm(), scale)
                    }
                    // Address Register Indirect with Index (Base Displacement) Mode
                    ExtensionWord::FEW { da, register: iregister, wl: _, scale, bs: _, is: _, bdsize: _, iis: _ } => {
                        let da_flag = if da == 0 { "d" } else { "a" };
                        let mut displacement: u32 = 0;
                        let (bdsize, _) = extword.remaining_length();
                        for j in 0..bdsize {
                            displacement += (cpu.lookahead(j + 1) * (1 << (8 * (bdsize - j - 1)))) as u32;
                        }
                        format!("({:x}a{:},{:}{:}.{:}*{:})", displacement as i32, earegister, da_flag, iregister, size.as_asm(), scale)
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
                        Size::Byte => format!("#${:02x}", (extword & 0xff) as u8),
                        Size::Word => format!("#${:04x}", extword),
                        Size::Long => format!("#${:04x}{:04x}", extword, cpu.lookahead(1))
                    }
                }
                _ => panic!("Invalid register!"),
            }
        }
        _ => panic!("Invalid addressing mode!"),
    }
}

#[derive(Copy, Clone)]
pub enum Condition {
    T = 0,
    F = 1,
    HI = 2,
    LS = 3,
    CC = 4,
    CS = 5,
    NE = 6,
    EQ = 7,
    VC = 8,
    VS = 9,
    PL = 10,
    MI = 11,
    GE = 12,
    LT = 13,
    GT = 14,
    LE = 15
}

impl Condition {
    pub fn from(condition: usize) -> Self {
        match condition {
            0 => Self::T,
            1 => Self::F,
            2 => Self::HI,
            3 => Self::LS,
            4 => Self::CC,
            5 => Self::CS,
            6 => Self::NE,
            7 => Self::EQ,
            8 => Self::VC,
            9 => Self::VS,
            10 => Self::PL,
            11 => Self::MI,
            12 => Self::GE,
            13 => Self::LT,
            14 => Self::GT,
            15 => Self::LE,
            _ => panic!("Invalid condition code!")
        }
    }
    pub fn as_asm(&self) -> String {
        match *self {
            Self::T => String::from("t"),
            Self::F => String::from("f"),
            Self::HI => String::from("hi"),
            Self::LS => String::from("ls"),
            Self::CC => String::from("cc"),
            Self::CS => String::from("cs"),
            Self::NE => String::from("ne"),
            Self::EQ => String::from("eq"),
            Self::VC => String::from("vc"),
            Self::VS => String::from("vs"),
            Self::PL => String::from("pl"),
            Self::MI => String::from("mi"),
            Self::GE => String::from("ge"),
            Self::LT => String::from("lt"),
            Self::GT => String::from("gt"),
            Self::LE => String::from("le"),
        }
    }
    pub fn evaluate(&self, cpu: &CPU) -> bool {
        match *self {
            Self::T => true,
            Self::F => false,
            Self::HI => !cpu.ccr(CCR::C) && !cpu.ccr(CCR::Z),
            Self::LS => cpu.ccr(CCR::C) || cpu.ccr(CCR::Z),
            Self::CC => !cpu.ccr(CCR::C),
            Self::CS => cpu.ccr(CCR::C),
            Self::NE => !cpu.ccr(CCR::Z),
            Self::EQ => cpu.ccr(CCR::Z),
            Self::VC => !cpu.ccr(CCR::V),
            Self::VS => cpu.ccr(CCR::V),
            Self::PL => !cpu.ccr(CCR::N),
            Self::MI => cpu.ccr(CCR::N),
            Self::GE => (cpu.ccr(CCR::N) && cpu.ccr(CCR::V)) || (!cpu.ccr(CCR::N) && !cpu.ccr(CCR::V)),
            Self::LT => (cpu.ccr(CCR::N) && !cpu.ccr(CCR::V)) || (!cpu.ccr(CCR::N) && cpu.ccr(CCR::V)),
            Self::GT => (cpu.ccr(CCR::N) && cpu.ccr(CCR::V) && !cpu.ccr(CCR::Z)) || (!cpu.ccr(CCR::N) && !cpu.ccr(CCR::V) && !cpu.ccr(CCR::Z)),
            Self::LE => cpu.ccr(CCR::Z) || (cpu.ccr(CCR::N) && !cpu.ccr(CCR::V)) || (!cpu.ccr(CCR::N) && cpu.ccr(CCR::V)),
        }
    }
}

enum Mode {
    Flip,
    Clear,
    Set,
    None
}

fn change_bit(mode: usize, earegister: usize, register: Option<usize>, cpu: &mut CPU, opmode: Mode) {
    let extword = if register == None {
        cpu.next_instruction() as usize
    } else {
        *cpu.dr[register.unwrap()].borrow() as usize
    };
    let handle: MemoryHandle;
    let bitnumber;
    let size;
    if mode == 0 {
        handle = cpu.memory_handle(mode, earegister, Size::Long);
        bitnumber = extword % 32;
        size = 4;
    } else {
        handle = cpu.memory_handle(mode, earegister, Size::Byte);
        bitnumber = extword % 8;
        size = 1;
    }
    let mut bitfield = handle.read(size).inner();
    let mut value = get_bit(bitfield as usize, bitnumber);
    let mut ccr = CCRFlags::new();
    ccr.z = Some(!value);
    ccr.set(cpu);
    match opmode {
        Mode::Clear => value = false,
        Mode::Flip => value = !value,
        Mode::Set => value = true,
        Mode::None => {}
    }
    set_bit(&mut bitfield, bitnumber, value);
    if mode == 0 {
        handle.write(OpResult::Long(bitfield));
    } else {
        handle.write(OpResult::Byte((bitfield & 0xff) as u8));
    }
}

