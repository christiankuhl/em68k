use crate::fields::{BitMode, Condition, EAMode, OpMode, OpResult, PackedBCD, Size};
use crate::fields::{EAMode::*, Size::*};
use crate::memory::MemoryHandle;
use crate::processor::{get_bit, set_bit, CCRFlags, CCR, CPU};
use crate::devices::Signal;

#[derive(Copy, Clone)]
pub enum Instruction {
    ANDICCR { extword: u16 },
    ANDISR { extword: u16 },
    EORICCR { extword: u16 },
    EORISR { extword: u16 },
    ILLEGAL,
    NOP,
    ORICCR { extword: u16 },
    ORISR { extword: u16 },
    RESET,
    RTE,
    RTR,
    RTS,
    STOP { extword: u32 },
    TRAPV,
    LINK { register: usize, displacement: i16 },
    SWAP { register: usize },
    UNLK { register: usize },
    TRAP { vector: usize },
    MOVEUSP { register: usize, dr: usize },
    BCHGS { mode: EAMode, extword: u16 },
    BCLRS { mode: EAMode, extword: u16 },
    BSETS { mode: EAMode, extword: u16 },
    BTSTS { mode: EAMode, extword: u16 },
    JMP { mode: EAMode },
    JSR { mode: EAMode },
    MOVEFROMCCR { mode: EAMode },
    MOVETOCCR { mode: EAMode },
    MOVEFROMSR { mode: EAMode },
    MOVETOSR { mode: EAMode },
    PEA { mode: EAMode },
    TAS { mode: EAMode },
    EXT { opmode: usize, register: usize },
    ASLRMEM { dr: usize, mode: EAMode },
    LSLRMEM { dr: usize, mode: EAMode },
    ROXLRMEM { dr: usize, mode: EAMode },
    ROLRMEM { dr: usize, mode: EAMode },
    DBCC { condition: Condition, register: usize, displacement: i32 },
    MOVEM { size: Size, dr: usize, mode: EAMode, register_mask: u16 },
    ABCD { rx: usize, ry: usize, rm: usize },
    SBCD { rx: usize, ry: usize, rm: usize },
    ADDI { size: Size, mode: EAMode, operand: OpResult },
    ANDI { size: Size, mode: EAMode, operand: OpResult },
    CLR { size: Size, mode: EAMode },
    CMPI { size: Size, mode: EAMode, operand: OpResult },
    EORI { size: Size, mode: EAMode, operand: OpResult },
    NEG { size: Size, mode: EAMode },
    NEGX { size: Size, mode: EAMode },
    NOT { size: Size, mode: EAMode },
    ORI { size: Size, mode: EAMode, operand: OpResult },
    SUBI { size: Size, mode: EAMode, operand: OpResult },
    TST { size: Size, mode: EAMode },
    BRA { displacement: i32 },
    BSR { displacement: i32 },
    CMPM { ax: usize, ay: usize, size: Size },
    ADDX { rx: usize, ry: usize, rm: usize, size: Size },
    SUBX { rx: usize, ry: usize, rm: usize, size: Size },
    ADDA { register: usize, opmode: usize, mode: EAMode },
    SUBA { register: usize, opmode: usize, mode: EAMode },
    CMPA { register: usize, opmode: usize, mode: EAMode },
    BCHG { register: usize, mode: EAMode },
    BCLR { register: usize, mode: EAMode },
    BSET { register: usize, mode: EAMode },
    BTST { register: usize, mode: EAMode },
    DIVS { register: usize, mode: EAMode },
    DIVU { register: usize, mode: EAMode },
    LEA { register: usize, mode: EAMode },
    MULS { register: usize, mode: EAMode },
    MULU { register: usize, mode: EAMode },
    NBCD { mode: EAMode },
    MOVEP { dregister: usize, opmode: usize, aregister: usize, displacement: i16 },
    SCC { condition: Condition, mode: EAMode },
    ASLRREG { register: usize, count: usize, size: Size, dr: usize, ir: usize },
    LSLRREG { register: usize, count: usize, size: Size, dr: usize, ir: usize },
    ROXLR { register: usize, count: usize, size: Size, dr: usize, ir: usize },
    ROLR { register: usize, count: usize, size: Size, dr: usize, ir: usize },
    MOVEQ { register: usize, data: usize },
    EXG { opmode: usize, rx: usize, ry: usize },
    CHK { register: usize, size: Size, mode: EAMode },
    MOVEA { register: usize, size: Size, mode: EAMode },
    ADDQ { data: usize, size: Size, mode: EAMode },
    SUBQ { data: usize, size: Size, mode: EAMode },
    BCC { condition: Condition, displacement: i32 },
    ADD { register: usize, opmode: OpMode, mode: EAMode },
    AND { register: usize, opmode: OpMode, mode: EAMode },
    CMP { register: usize, opmode: OpMode, mode: EAMode },
    EOR { register: usize, opmode: OpMode, mode: EAMode },
    OR { register: usize, opmode: OpMode, mode: EAMode },
    SUB { register: usize, opmode: OpMode, mode: EAMode },
    MOVE { size: Size, destmode: EAMode, srcmode: EAMode },
}

pub enum ExtensionWord {
    BEW { da: usize, register: usize, wl: usize, scale: usize, displacement: usize },
    FEW { da: usize, register: usize, wl: usize, scale: usize, bs: usize, is: usize, bdsize: usize, iis: usize },
}

impl ExtensionWord {
    pub fn remaining_length(&self) -> (usize, usize) {
        match *self {
            Self::FEW { da: _, register: _, wl: _, scale: _, bs: _, is: _, bdsize, iis } => {
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
    pub fn execute(&self, cpu: &mut CPU) -> Signal {
        match *self {
            Self::ANDICCR { extword } => {
                cpu.sr &= (0xff00 | extword) as u32;
            }
            Self::ANDISR { extword } => {
                cpu.sr &= extword as u32;
            }
            Self::EORICCR { extword } => {
                cpu.sr ^= 0x001f & extword as u32;
            }
            Self::EORISR { extword } => {
                cpu.sr ^= extword as u32;
            }
            Self::ILLEGAL => {
                let trap = Self::TRAP { vector: 4 };
                trap.execute(cpu);
            }
            Self::NOP => {}
            Self::ORICCR { extword } => {
                cpu.sr |= (0x001f & extword) as u32;
            }
            Self::ORISR { extword } => {
                cpu.sr |= extword as u32;
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
                    let mut ram_handle = MemoryHandle::new(None, Some(*ssp as usize), None, cpu);
                    cpu.sr = ram_handle.read(Word).inner();
                    *ssp += 2;
                    ram_handle.offset(2);
                    cpu.pc = ram_handle.read(Long).inner();
                    *ssp += 4;
                }
            }
            Self::RTR => {
                let _sp = cpu.ar(7);
                let mut sp = _sp.as_ref().borrow_mut();
                let mut ram_handle = MemoryHandle::new(None, Some(*sp as usize), None, cpu);
                let ccr = ram_handle.read(Word).inner() as u16 & 0x00ff;
                cpu.sr &= 0xff00;
                cpu.sr |= ccr as u32;
                *sp += 2;
                ram_handle = MemoryHandle::new(None, Some(*sp as usize), None, cpu);
                cpu.pc = ram_handle.read(Long).inner();
                *sp += 4;
            }
            Self::RTS => {
                let _sp = cpu.ar(7);
                let mut sp = _sp.as_ref().borrow_mut();
                let ram_handle = MemoryHandle::new(None, Some(*sp as usize), None, cpu);
                cpu.pc = ram_handle.read(Long).inner();
                *sp += 4;
            }
            Self::STOP { extword } => {
                if !cpu.in_supervisor_mode() {
                    privilege_violation(cpu);
                } else {
                    cpu.sr = extword;
                    return Signal::Quit
                }
            }
            Self::TRAPV => {
                if cpu.sr & (1 << (CCR::V as u8)) != 0 {
                    let trap = Self::TRAP { vector: 7 };
                    trap.execute(cpu);
                }
            }
            Self::LINK { register, displacement } => {
                let _sp = cpu.ar(7);
                let mut sp = _sp.as_ref().borrow_mut();
                *sp -= 4;
                let _ar =  cpu.ar(register);
                let mut ar = _ar.as_ref().borrow_mut();
                let ram_handle = MemoryHandle::new(None, Some(*sp as usize), None, cpu);
                ram_handle.write(OpResult::Long(*ar));
                *ar = *sp;
                *sp = (*sp as i32 + displacement as i32) as u32;
            }
            Self::SWAP { register } => {
                let res;
                {
                    let mut reg = cpu.dr[register].as_ref().borrow_mut();
                    *reg = ((*reg & 0xffff0000) >> 16) + ((*reg & 0xffff) << 16);
                    res = *reg;
                }
                let ccr = CCRFlags { c: Some(false), v: Some(false), z: Some(res == 0), n: Some(res & (1 << 31) > 0), x: None };
                ccr.set(cpu);
            }
            Self::UNLK { register } => {
                let _sp = cpu.ar(7);
                let mut sp = _sp.as_ref().borrow_mut();
                let _ar = cpu.ar(register);
                let mut ar = _ar.as_ref().borrow_mut();
                *sp = *ar;
                let ram_handle = MemoryHandle::new(None, Some(*sp as usize), None, cpu);
                *ar = ram_handle.read(Long).inner();
                *sp += 4;
            }
            Self::TRAP { vector } => {
                cpu.supervisor_mode(true);
                let mut ssp = cpu.ssp.as_ref().borrow_mut();
                *ssp -= 4;
                let mut ram_handle = MemoryHandle::new(None, Some(*ssp as usize), None, cpu);
                ram_handle.write(OpResult::Long(cpu.pc));
                *ssp -= 2;
                ram_handle.offset(-2);
                ram_handle.write(OpResult::Word(cpu.sr as u16));
                ram_handle = MemoryHandle::new(None, Some(4 * vector as usize), None, cpu);
                cpu.pc = ram_handle.read(Long).inner();
            }
            Self::MOVEUSP { register, dr } => {
                if !cpu.in_supervisor_mode() {
                    privilege_violation(cpu);
                } else {
                    if dr == 0 {
                        let _ar = cpu.ar(register); 
                        let ar = _ar.as_ref().borrow();
                        let mut usp = cpu.ar[7].as_ref().borrow_mut();
                        *usp = *ar;
                    } else {
                        let _ar = cpu.ar(register);
                        let mut ar = _ar.as_ref().borrow_mut();
                        let usp = cpu.ar[7].as_ref().borrow();
                        *ar = *usp;
                    }
                }
            }
            Self::BCHGS { mode, extword } => {
                change_bit(mode, None, Some(extword), cpu, BitMode::Flip);
            }
            Self::BCLRS { mode, extword } => {
                change_bit(mode, None, Some(extword), cpu, BitMode::Clear);
            }
            Self::BSETS { mode, extword } => {
                change_bit(mode, None, Some(extword), cpu, BitMode::Set);
            }
            Self::BTSTS { mode, extword } => {
                change_bit(mode, None, Some(extword), cpu, BitMode::None);
            }
            Self::JMP { mode } => {
                let addr = cpu.memory_address(mode);
                cpu.pc = addr;
            }
            Self::JSR { mode } => {
                let pc = cpu.pc;
                cpu.pc = cpu.memory_address(mode);
                let _sp = cpu.ar(7);
                let mut sp = _sp.as_ref().borrow_mut();
                *sp -= 4;
                let ram_handle = MemoryHandle::new(None, Some(*sp as usize), None, cpu);
                ram_handle.write(OpResult::Long(pc));
            }
            Self::MOVEFROMCCR { mode } => {
                let dest = cpu.memory_handle(mode);
                dest.write(OpResult::Word((cpu.sr & 0xff) as u16));
            }
            Self::MOVETOCCR { mode } => {
                let src = cpu.memory_handle(mode).read(Word).inner();
                cpu.sr &= 0xff00;
                cpu.sr |= src;
            }
            Self::MOVEFROMSR { mode } => {
                let dest = cpu.memory_handle(mode);
                dest.write(OpResult::Word((cpu.sr & 0xf71f) as u16));
            }
            Self::MOVETOSR { mode } => {
                let src = cpu.memory_handle(mode).read(Word).inner();
                cpu.sr = src & 0xf71f;
            }
            Self::PEA { mode } => {
                let addr = cpu.memory_address(mode);
                let _sp = cpu.ar(7);
                let mut sp = _sp.as_ref().borrow_mut();
                *sp -= 4;
                let ram_handle = MemoryHandle::new(None, Some(*sp as usize), None, cpu);
                ram_handle.write(OpResult::Long(addr));
            }
            Self::TAS { mode } => {
                let handle = cpu.memory_handle(mode);
                let mut operand = (handle.read(Byte).inner() & 0xff) as u8;
                let ccr = CCRFlags {
                    x: None,
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
            Self::ASLRMEM { dr, mode } => {
                let handle = cpu.memory_handle(mode);
                aslr(handle, Word, 1, dr, cpu);
            }
            Self::LSLRMEM { dr, mode } => {
                let handle = cpu.memory_handle(mode);
                lslr(handle, Word, 1, dr, cpu);
            }
            Self::DBCC { condition, register, displacement } => {
                let counter_reg = cpu.memory_handle(DataDirect(register));
                let mut counter = counter_reg.read(Word).inner() as i16;
                if !condition.evaluate(cpu) {
                    counter = counter.wrapping_sub(1);
                    counter_reg.write(OpResult::Word(counter as u16));
                    if counter != -1 {
                        cpu.pc = (cpu.pc as i32 + displacement - 2) as u32;
                    }
                }
            }
            Self::MOVEM { size, dr, mode, register_mask } => {
                // FIXME: Handle address register for specific architecture differences
                if dr == 0 {
                    let mut tgt = cpu.memory_handle(mode);
                    let mut result;
                    let mut _register;
                    for j in 0..16 {
                        if register_mask & (1 << j) != 0 {
                            let register;
                            if mode == AddressPredecr(0, Byte) {
                                if j < 8 {
                                    _register = cpu.ar(7 - j);
                                    register = _register.as_ref().borrow();
                                } else {
                                    register = cpu.dr[15 - j].as_ref().borrow();
                                }
                            } else {
                                if j < 8 {
                                    register = cpu.dr[j].as_ref().borrow();
                                } else {
                                    _register = cpu.ar(j - 8);
                                    register = _register.as_ref().borrow();
                                }
                            }
                            if size == Word {
                                result = OpResult::Word(*register as u16)
                            } else {
                                result = OpResult::Long(*register);
                            }
                            tgt.write(result);
                            if mode == AddressPredecr(0, Byte) {
                                tgt.offset(-(size as isize));
                            } else {
                                tgt.offset(size as isize);
                            }
                        }
                    }
                } else if dr == 1 {
                    let mut src = cpu.memory_handle(mode);
                    let mut result;
                    let mut _register;
                    for j in 0..16 {
                        if register_mask & (1 << j) != 0 {
                            let mut register;
                            if j < 8 {
                                register = cpu.dr[j].as_ref().borrow_mut();
                            } else {
                                _register = cpu.ar(j - 8); 
                                register = _register.as_ref().borrow_mut();
                            }
                            if size == Word {
                                result = src.read(size).sign_extend() as u32
                            } else {
                                result = src.read(size).inner()
                            }
                            *register = result;
                            src.offset(size as isize);
                        }
                    }
                }
            }
            Self::ABCD { rx, ry, rm } => {
                let mut ccr = CCRFlags::new();
                let src;
                let dest;
                if rm == 0 {
                    src = cpu.memory_handle(DataDirect(ry));
                    dest = cpu.memory_handle(DataDirect(rx));
                } else {
                    src = cpu.memory_handle(AddressPredecr(ry, Byte));
                    dest = cpu.memory_handle(AddressPredecr(rx, Byte));
                }
                let a = PackedBCD::from(src.read(Byte));
                let b = PackedBCD::from(dest.read(Byte));
                let (result, carry) = a.add(b, cpu.ccr(CCR::X));
                dest.write(result);
                // cc_update (GEN, UND, CASE_1, UND, N_A, source, dest, result, (long) BYTE_MASK, 0);
                ccr.x = Some(carry);
                ccr.c = Some(carry);
                if result.inner() != 0 {
                    ccr.z = Some(false)
                };
                ccr.set(cpu);
            }
            Self::SBCD { rx, ry, rm } => {
                let mut ccr = CCRFlags::new();
                let src;
                let dest;
                if rm == 0 {
                    src = cpu.memory_handle(DataDirect(ry));
                    dest = cpu.memory_handle(DataDirect(rx));
                } else {
                    src = cpu.memory_handle(AddressPredecr(ry, Byte));
                    dest = cpu.memory_handle(AddressPredecr(rx, Byte));
                }
                let a = PackedBCD::from(dest.read(Byte));
                let b = PackedBCD::from(src.read(Byte));
                let (result, carry) = a.sub(b, cpu.ccr(CCR::X));
                dest.write(result);
                ccr.c = Some(carry);
                if result.inner() != 0 {
                    ccr.z = Some(false)
                };
                // cc_update (GEN, UND, CASE_1, UND, N_A, source, dest, result, (long) BYTE_MASK, 0);
                ccr.set(cpu);
            }
            Self::ADDI { size, mode, operand } => {
                let handle = cpu.memory_handle(mode);
                let dest_operand = handle.read(size);
                let res = dest_operand.add(operand, false);
                let result = res.0;
                let ccr = res.1;
                handle.write(result);
                ccr.set(cpu);
            }
            Self::ANDI { size, mode, operand } => {
                let dest = cpu.memory_handle(mode);
                let dest_operand = dest.read(size);
                let res = operand.and(dest_operand);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::CLR { size, mode } => {
                let dest = cpu.memory_handle(mode);
                let res = dest.read(size).clear();
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::CMPI { size, mode, operand } => {
                let dest_operand = cpu.memory_handle(mode).read(size);
                let res = dest_operand.sub(operand, false);
                let ccr = res.1;
                ccr.set(cpu);
            }
            Self::EORI { size, mode, operand } => {
                let dest = cpu.memory_handle(mode);
                let dest_operand = dest.read(size);
                let res = operand.xor(dest_operand);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::NEG { size, mode } => {
                let handle = cpu.memory_handle(mode);
                let operand = handle.read(size);
                let res = size.zero().sub(operand, false);
                let result = res.0;
                let ccr = res.1;
                handle.write(result);
                ccr.set(cpu);
            }
            Self::NEGX { size, mode } => {
                let handle = cpu.memory_handle(mode);
                let x = cpu.ccr(CCR::X);
                let operand = handle.read(size);
                let res = size.zero().sub(operand, x);
                let result = res.0;
                let mut ccr = res.1;
                let dm = operand.sign_extend() < 0;
                let rm = result.sign_extend() < 0;
                ccr.v = Some(dm && rm);
                ccr.c = Some(dm || rm);
                ccr.x = ccr.c;
                handle.write(result);
                ccr.set(cpu);
            }
            Self::NOT { size, mode } => {
                let dest = cpu.memory_handle(mode);
                let operand = dest.read(size);
                let res = operand.not();
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::ORI { size, mode, operand } => {
                let dest = cpu.memory_handle(mode);
                let dest_operand = dest.read(size);
                let res = operand.or(dest_operand);
                let ccr = res.1;
                dest.write(res.0);
                ccr.set(cpu);
            }
            Self::SUBI { size, mode, operand } => {
                let handle = cpu.memory_handle(mode);
                let subtractor = handle.read(size);
                let res = subtractor.sub(operand, false);
                let result = res.0;
                let ccr = res.1;
                handle.write(result);
                ccr.set(cpu);
            }
            Self::TST { size, mode } => {
                let operand = cpu.memory_handle(mode).read(size);
                let (_, mut ccr) = operand.sub(size.zero(), false);
                ccr.v = Some(false);
                ccr.c = Some(false);
                ccr.set(cpu);
            }
            Self::BRA { displacement } => {
                cpu.pc = (cpu.pc as i32 + displacement) as u32;
            }
            Self::BSR { displacement } => {
                let pc = (cpu.pc as i32 + displacement) as u32;
                let _sp = cpu.ar(7);
                let mut sp = _sp.as_ref().borrow_mut();
                *sp -= 4;
                let ram_handle = MemoryHandle::new(None, Some(*sp as usize), None, cpu);
                ram_handle.write(OpResult::Long(cpu.pc));
                cpu.pc = pc;
            }
            Self::CMPM { ax, ay, size } => {
                let src = cpu.memory_handle(AddressPostincr(ay, size)).read(size);
                let dest = cpu.memory_handle(AddressPostincr(ax, size));
                let res = dest.read(size).sub(src, false);
                let ccr = res.1;
                ccr.set(cpu);
            }
            Self::ADDX { rx, ry, rm, size } => {
                let src;
                let dest;
                if rm == 0 {
                    src = cpu.memory_handle(DataDirect(ry));
                    dest = cpu.memory_handle(DataDirect(rx));
                } else {
                    src = cpu.memory_handle(AddressPredecr(ry, size));
                    dest = cpu.memory_handle(AddressPredecr(rx, size));
                }
                let x = cpu.ccr(CCR::X);
                let (res, mut ccr) = dest.read(size).add(src.read(size), x);
                if ccr.z != Some(false) {
                    ccr.z = None
                }
                dest.write(res);
                ccr.set(cpu);
            }
            Self::SUBX { rx, ry, rm, size } => {
                let src;
                let dest;
                if rm == 0 {
                    src = cpu.memory_handle(DataDirect(ry));
                    dest = cpu.memory_handle(DataDirect(rx));
                } else {
                    src = cpu.memory_handle(AddressPredecr(ry, size));
                    dest = cpu.memory_handle(AddressPredecr(rx, size));
                }
                let x = cpu.ccr(CCR::X);
                let (res, mut ccr) = dest.read(size).sub(src.read(size), x);
                if ccr.z != Some(false) {
                    ccr.z = None
                }
                dest.write(res);
                ccr.set(cpu);
            }
            Self::ADDA { register, opmode, mode } => {
                let size = Size::from_opcode(opmode / 4 + 1);
                let operand = cpu.memory_handle(mode).read(size);
                let reg_handle = cpu.memory_handle(AddressDirect(register));
                let (res, _) = OpResult::Long(operand.sign_extend() as u32).add(reg_handle.read(Long), false);
                reg_handle.write(res);
            }
            Self::SUBA { register, opmode, mode } => {
                let size = Size::from_opcode(opmode / 4 + 1);
                let operand = cpu.memory_handle(mode).read(size);
                let reg_handle = cpu.memory_handle(AddressDirect(register));
                let (res, _) = reg_handle.read(Long).sub(OpResult::Long(operand.sign_extend() as u32), false);
                reg_handle.write(res);
            }
            Self::CMPA { register, opmode, mode } => {
                let size = Size::from_opcode(opmode / 4 + 1);
                let arhandle = cpu.memory_handle(AddressDirect(register));
                let ophandle = cpu.memory_handle(mode);
                let ar = arhandle.read(Long);
                let op = ophandle.read(size).sign_extend() as u32;
                let res = ar.sub(OpResult::Long(op), false);
                let ccr = res.1;
                ccr.set(cpu);
            }
            Self::BCHG { register, mode } => {
                change_bit(mode, Some(register), None, cpu, BitMode::Flip);
            }
            Self::BCLR { register, mode } => {
                change_bit(mode, Some(register), None, cpu, BitMode::Clear);
            }
            Self::BSET { register, mode } => {
                change_bit(mode, Some(register), None, cpu, BitMode::Set);
            }
            Self::BTST { register, mode } => {
                change_bit(mode, Some(register), None, cpu, BitMode::None);
            }
            Self::DIVS { register, mode } => {
                let dest = cpu.memory_handle(DataDirect(register));
                let src = cpu.memory_handle(mode);
                let dividend = dest.read(Long).inner() as i32;
                let divisor = src.read(Word).inner() as i32;
                let mut ccr = CCRFlags::new();
                ccr.c = Some(false);
                if divisor == 0 {
                    let trap = Self::TRAP { vector: 4 }; // FIXME: Right trap vector
                    ccr.set(cpu);
                    return trap.execute(cpu);
                }
                let res = dividend.overflowing_div(divisor);
                if res.1 || res.0 > 0x7fff || res.0 < -0x8000 {
                    ccr.v = Some(true);
                    ccr.set(cpu);
                    return Signal::Ok
                }
                ccr.z = Some(res.0 == 0);
                ccr.v = Some(false);
                ccr.n = Some((res.0 & 0x8000) > 0);
                let rem = (dividend % divisor) * dividend.signum();
                dest.write(OpResult::Long(((rem as u32) << 16) + (res.0 as u32 & 0xffff)));
                ccr.set(cpu);
            }
            Self::DIVU { register, mode } => {
                let dest = cpu.memory_handle(DataDirect(register));
                let src = cpu.memory_handle(mode);
                let dividend = dest.read(Long).inner();
                let divisor = src.read(Word).inner() as u32;
                let mut ccr = CCRFlags::new();
                ccr.c = Some(false);
                if divisor == 0 {
                    let trap = Self::TRAP { vector: 5 };
                    ccr.set(cpu);
                    return trap.execute(cpu);
                }
                let res = dividend.overflowing_div(divisor);
                if res.0 > 0xffff {
                    ccr.v = Some(true);
                    ccr.set(cpu);
                    return Signal::Ok
                }
                ccr.z = Some(res.0 == 0);
                ccr.v = Some(false);
                ccr.n = Some((res.0 & 0x8000) > 0);
                let rem = dividend % divisor;
                let result = OpResult::Long(((rem as u32) << 16) + (res.0 as u32 & 0xffff));
                dest.write(result);
                ccr.set(cpu);
            }
            Self::LEA { register, mode } => {
                let addr = cpu.memory_address(mode);
                cpu.ar(register).replace(addr);
            }
            Self::MULS { register, mode } => {
                let src = cpu.memory_handle(mode);
                let dest = cpu.memory_handle(DataDirect(register));
                let factor1 = src.read(Word).sign_extend();
                let factor2 = dest.read(Word).sign_extend();
                let res = factor1.overflowing_mul(factor2);
                let mut ccr = CCRFlags::new();
                ccr.n = Some(res.0 < 0);
                ccr.z = Some(res.0 == 0);
                ccr.v = Some(res.1);
                ccr.c = Some(false);
                dest.write(OpResult::Long(res.0 as u32));
                ccr.set(cpu);
            }
            Self::MULU { register, mode } => {
                let src = cpu.memory_handle(mode);
                let dest = cpu.memory_handle(DataDirect(register));
                let factor1 = src.read(Word).inner() as u32;
                let factor2 = dest.read(Word).inner() as u32;
                let res = factor1.overflowing_mul(factor2);
                let mut ccr = CCRFlags::new();
                ccr.n = Some((res.0 as i32) < 0);
                ccr.z = Some(res.0 == 0);
                ccr.v = Some(res.1);
                ccr.c = Some(false);
                dest.write(OpResult::Long(res.0 as u32));
                ccr.set(cpu);
            }
            Self::NBCD { mode } => {
                let mut ccr = CCRFlags::new();
                let dest = cpu.memory_handle(mode);
                let operand = PackedBCD::from(dest.read(Byte));
                let (result, carry) = PackedBCD(0).sub(operand, cpu.ccr(CCR::X));
                dest.write(result);
                ccr.c = Some(carry);
                if result.inner() != 0 {
                    ccr.z = Some(false)
                };
                ccr.set(cpu);
            }
            Self::MOVEP { dregister, opmode, aregister, displacement } => {
                let oplength = 1 << ((opmode % 2) + 1);
                let mut ram_handle = cpu.memory_handle(AddressDisplacement(aregister, displacement));
                let mut result: u32 = 0;
                let reg = cpu.memory_handle(DataDirect(dregister));
                if (opmode - 4) / 2 == 0 {
                    if oplength == 2 {
                        result = ram_handle.read(Byte).inner() << 8;
                        ram_handle.offset(2);
                        result += ram_handle.read(Byte).inner();
                        reg.write(OpResult::Word(result as u16));
                    } else {
                        for j in 0..4 {
                            result += ram_handle.read(Byte).inner() << (24 - 8 * j);
                            ram_handle.offset(2);
                        }
                        reg.write(OpResult::Long(result));
                    }
                } else {
                    result = *cpu.dr[dregister].as_ref().borrow();
                    ram_handle.offset(2 * oplength);
                    for _ in 0..oplength / 2 {
                        ram_handle.offset(-2);
                        ram_handle.write(OpResult::Byte((result & 0xff) as u8));
                        result = result >> 8;
                        ram_handle.offset(-2);
                        ram_handle.write(OpResult::Byte((result & 0xff) as u8));
                        result = result >> 8;
                    }
                    
                }
            }
            Self::SCC { condition, mode } => {
                let dest = cpu.memory_handle(mode);
                if condition.evaluate(cpu) {
                    dest.write(OpResult::Byte(0xff));
                } else {
                    dest.write(OpResult::Byte(0));
                }
            }
            Self::ASLRREG { register, count, size, dr, ir } => {
                let shift_count = shift_count(ir, count, cpu);
                let handle = cpu.memory_handle(DataDirect(register));
                aslr(handle, size, shift_count, dr, cpu);
            }
            Self::LSLRREG { register, count, size, dr, ir } => {
                let shift_count = shift_count(ir, count, cpu);
                let handle = cpu.memory_handle(DataDirect(register));
                lslr(handle, size, shift_count, dr, cpu);
            }
            Self::ROXLR { register, count, size, dr, ir } => {
                let shift_count = shift_count(ir, count, cpu);
                let handle = cpu.memory_handle(DataDirect(register));
                roxlr(handle, size, shift_count, dr, cpu);
            }
            Self::ROLR { register, count, size, dr, ir } => {
                let shift_count = shift_count(ir, count, cpu);
                let handle = cpu.memory_handle(DataDirect(register));
                rolr(handle, size, shift_count as u32, dr, cpu);
            }
            Self::ROXLRMEM { dr, mode } => {
                let handle = cpu.memory_handle(mode);
                roxlr(handle, Word, 1, dr, cpu);
            }
            Self::ROLRMEM { dr, mode } => {
                let handle = cpu.memory_handle(mode);
                rolr(handle, Word, 1, dr, cpu);
            }
            Self::MOVEQ { register, data } => {
                let result = data as i8;
                let mut ccr = CCRFlags::new();
                cpu.dr[register].as_ref().replace(result as u32);
                ccr.v = Some(false);
                ccr.c = Some(false);
                ccr.n = Some(result < 0);
                ccr.z = Some(result == 0);
                ccr.set(cpu);
            }
            Self::EXG { opmode, rx, ry } => {
                let (src, dest) = match opmode {
                    8 => (cpu.memory_handle(DataDirect(rx)), cpu.memory_handle(DataDirect(ry))),
                    9 => (cpu.memory_handle(AddressDirect(rx)), cpu.memory_handle(AddressDirect(ry))),
                    17 => (cpu.memory_handle(DataDirect(rx)), cpu.memory_handle(AddressDirect(ry))),
                    _ => panic!("Invalid opmode!"),
                };
                let srcval = src.read(Long);
                src.write(dest.read(Long));
                dest.write(srcval);
            }
            Self::CHK { register, size, mode } => {
                let upper_bound = cpu.memory_handle(mode).read(size).sign_extend() as i32;
                let operand = cpu.memory_handle(DataDirect(register)).read(size).sign_extend() as i32;
                let mut ccr = CCRFlags::new();
                let trap = Self::TRAP { vector: 6 };
                if operand < 0 {
                    ccr.n = Some(true);
                    ccr.set(cpu);
                    trap.execute(cpu);
                } else if operand > upper_bound {
                    ccr.n = Some(false);
                    ccr.set(cpu);
                    trap.execute(cpu);
                }
            }
            Self::MOVEA { register, size, mode } => match size {
                Long => {
                    let src = cpu.memory_handle(mode).read(Long).inner();
                    let _reg = cpu.ar(register);
                    let mut dest = _reg.as_ref().borrow_mut();
                    *dest = src;
                }
                Word => {
                    let src = cpu.memory_handle(mode).read(Word).inner() as i16;
                    let _reg = cpu.ar(register);
                    let mut dest = _reg.as_ref().borrow_mut();
                    *dest = src as u32;
                }
                _ => panic!("Invalid operand size!"),
            },
            Self::ADDQ { data, size, mode } => {
                let handle = cpu.memory_handle(mode);
                if !mode.is_address_register() {
                    let operand = handle.read(size);
                    let (res, ccr) = operand.add(size.from(data), false);
                    handle.write(res);
                    ccr.set(cpu);
                } else {
                    let operand = handle.read(Long);
                    let (res, _) = operand.add(OpResult::Long(data as u32), false);
                    handle.write(res);
                }
            }
            Self::SUBQ { data, size, mode } => {
                let handle = cpu.memory_handle(mode);
                if !mode.is_address_register() {
                    let operand = handle.read(size);
                    let (res, ccr) = operand.sub(size.from(data), false);
                    handle.write(res);
                    ccr.set(cpu);
                } else {
                    let operand = handle.read(Long);
                    let (res, _) = operand.sub(OpResult::Long(data as u32), false);
                    handle.write(res);
                }
            }
            Self::BCC { condition, displacement } => {
                if condition.evaluate(cpu) {
                    cpu.pc = (cpu.pc as i32 + displacement) as u32;
                }
            }
            Self::ADD { register, opmode, mode } => {
                let drhandle = cpu.memory_handle(DataDirect(register));
                let ophandle = cpu.memory_handle(mode);
                let dr = drhandle.read(opmode.size());
                let op = ophandle.read(opmode.size());
                let res = dr.add(op, false);
                let ccr = res.1;
                let result = res.0;
                opmode.write(drhandle, ophandle, result);
                ccr.set(cpu);
            }
            Self::AND { register, opmode, mode } => {
                let drhandle = cpu.memory_handle(DataDirect(register));
                let ophandle = cpu.memory_handle(mode);
                let dr = drhandle.read(opmode.size());
                let op = ophandle.read(opmode.size());
                let res = dr.and(op);
                let ccr = res.1;
                let result = res.0;
                opmode.write(drhandle, ophandle, result);
                ccr.set(cpu);
            }
            Self::CMP { register, opmode, mode } => {
                let drhandle = cpu.memory_handle(DataDirect(register));
                let ophandle = cpu.memory_handle(mode);
                let dr = drhandle.read(opmode.size());
                let op = ophandle.read(opmode.size());
                let res = dr.sub(op, false);
                let ccr = res.1;
                ccr.set(cpu);
            }
            Self::EOR { register, opmode, mode } => {
                let drhandle = cpu.memory_handle(DataDirect(register));
                let ophandle = cpu.memory_handle(mode);
                let dr = drhandle.read(opmode.size());
                let op = ophandle.read(opmode.size());
                let res = dr.xor(op);
                let ccr = res.1;
                let result = res.0;
                opmode.write(drhandle, ophandle, result);
                ccr.set(cpu);
            }
            Self::OR { register, opmode, mode } => {
                let drhandle = cpu.memory_handle(DataDirect(register));
                let ophandle = cpu.memory_handle(mode);
                let dr = drhandle.read(opmode.size());
                let op = ophandle.read(opmode.size());
                let res = dr.or(op);
                let ccr = res.1;
                let result = res.0;
                opmode.write(drhandle, ophandle, result);
                ccr.set(cpu);
            }
            Self::SUB { register, opmode, mode } => {
                let drhandle = cpu.memory_handle(DataDirect(register));
                let ophandle = cpu.memory_handle(mode);
                let dr = drhandle.read(opmode.size());
                let op = ophandle.read(opmode.size());
                let res = match opmode {
                    OpMode::MemoryToRegister(_) => dr.sub(op, false),
                    OpMode::RegisterToMemory(_) => op.sub(dr, false),
                };
                let ccr = res.1;
                let result = res.0;
                opmode.write(drhandle, ophandle, result);
                ccr.set(cpu);
            }
            Self::MOVE { size, destmode, srcmode } => {
                let src = cpu.memory_handle(srcmode);
                let dest = cpu.memory_handle(destmode);
                let result = src.read(size);
                dest.write(result);
                let ccr = CCRFlags {
                    c: Some(false),
                    v: Some(false),
                    z: Some(result.inner() == 0),
                    n: Some(result.sign_extend() < 0),
                    x: None,
                };
                ccr.set(cpu);
            }
        }
        Signal::Ok
    }
    pub fn as_asm(&self, cpu: &CPU) -> String {
        match *self {
            Self::ANDICCR { extword } => format!("andi #${:04x},ccr", extword),
            Self::ANDISR { extword } => format!("andi #${:04x},sr", extword),
            Self::EORICCR { extword } => format!("eori #${:04x},ccr", extword),
            Self::EORISR { extword } => format!("eori #${:04x},sr", extword),
            Self::ILLEGAL => String::from("illegal"),
            Self::NOP => String::from("nop"),
            Self::ORICCR { extword } => format!("ori #${:04x},ccr", extword),
            Self::ORISR { extword } => format!("ori #${:04x},sr", extword),
            Self::RESET => String::from("reset"),
            Self::RTE => String::from("rte"),
            Self::RTR => String::from("rtr"),
            Self::RTS => String::from("rts"),
            Self::STOP { extword: _ } => String::from("stop"),
            Self::TRAPV => String::from("trapv"),
            Self::LINK { register, displacement } => format!("link a{},#${:04x}", register, displacement),
            Self::SWAP { register } => format!("swap d{}", register),
            Self::UNLK { register } => format!("unlk a{}", register),
            Self::TRAP { vector } => format!("trap #{}", vector),
            Self::MOVEUSP { register, dr } => {
                if dr == 0 {
                    format!("move a{},usp", register)
                } else {
                    format!("move usp,a{}", register)
                }
            }
            Self::BCHGS { mode, extword } => format!("bchg #{},{}", extword, mode),
            Self::BCLRS { mode, extword } => format!("bclr #{},{}", extword, mode),
            Self::BSETS { mode, extword } => format!("bset #{},{}", extword, mode),
            Self::BTSTS { mode, extword } => format!("btst #{},{}", extword, mode),
            Self::JMP { mode } => format!("jmp {}", mode),
            Self::JSR { mode } => format!("jsr {}", mode),
            Self::MOVEFROMCCR { mode } => format!("move ccr,{}", mode),
            Self::MOVETOCCR { mode } => format!("move {},ccr", mode),
            Self::MOVEFROMSR { mode } => format!("move sr,{}", mode),
            Self::MOVETOSR { mode } => format!("move {},sr", mode),
            Self::PEA { mode } => format!("pea {}", mode),
            Self::TAS { mode } => format!("tas {}", mode),
            Self::EXT { opmode, register } => format!("ext.{} d{}", if opmode == 2 { "w" } else { "l" }, register),
            Self::ASLRMEM { dr, mode } => format!("as{} {}", if dr == 0 { "r" } else { "l" }, mode),
            Self::LSLRMEM { dr, mode } => format!("ls{} {}", if dr == 0 { "r" } else { "l" }, mode),
            Self::ROXLRMEM { dr, mode } => format!("rox{} {}", if dr == 0 { "r" } else { "l" }, mode),
            Self::ROLRMEM { dr, mode } => format!("ro{} {}", if dr == 0 { "r" } else { "l" }, mode),
            Self::DBCC { condition, register, displacement } => {
                format!("db{} d{},${:08x}", condition, register, (cpu.pc as i32 + displacement - 2) as u32)
            }
            Self::MOVEM { size, dr, mode, register_mask } => {
                let mut register_list = String::new();
                for j in 0..16 {
                    if mode == AddressPredecr(0, Size::Byte) {
                        if register_mask & (1 << j) != 0 {
                            register_list.push_str(&format!("{}{}/", if j < 8 { "a" } else { "d" }, (15 - j) % 8));
                        }
                    } else {
                        if register_mask & (1 << j) != 0 {
                            register_list.push_str(&format!("{}{}/", if j < 8 { "d" } else { "a" }, j % 8));
                        }
                    }
                }
                register_list.pop();
                if dr == 0 {
                    format!("movem.{} {},{}", size, &register_list, mode)
                } else {
                    format!("movem.{} {},{}", size, mode, &register_list)
                }
            }
            Self::ABCD { rx, ry, rm } => {
                if rm == 0 {
                    format!("abcd d{},d{}", ry, rx)
                } else {
                    format!("abcd -(A{}),-(A{})", ry, rx)
                }
            }
            Self::SBCD { rx, ry, rm } => {
                if rm == 0 {
                    format!("sbcd d{},d{}", ry, rx)
                } else {
                    format!("sbcd -(a{}),-(a{})", ry, rx)
                }
            }
            Self::ADDI { size, mode, operand } => format!("addi.{} #{},{}", size, operand, mode),
            Self::ANDI { size, mode, operand } => format!("andi.{} #{},{}", size, operand, mode),
            Self::CLR { size, mode } => format!("clr.{} {}", size, mode),
            Self::CMPI { size, mode, operand } => format!("cmpi.{} #{},{}", size, operand, mode),
            Self::EORI { size, mode, operand } => format!("eori.{} #{},{}", size, operand, mode),
            Self::NEG { size, mode } => format!("neg.{} {}", size, mode),
            Self::NEGX { size, mode } => format!("negx.{} {}", size, mode),
            Self::NOT { size, mode } => format!("not.{} {}", size, mode),
            Self::ORI { size, mode, operand } => format!("ori.{} #{},{}", size, operand, mode),
            Self::SUBI { size, mode, operand } => format!("subi.{} #{},{}", size, operand, mode),
            Self::TST { size, mode } => format!("tst.{} {}", size, mode),
            Self::BRA { displacement } => {
                let pc = (cpu.pc as i32 + displacement) as u32;
                format!("bra ${:08x}", pc)
            }
            Self::BSR { displacement } => {
                let pc = (cpu.pc as i32 + displacement) as u32;
                format!("bsr ${:08x}", pc)
            }
            Self::CMPM { ax, ay, size } => format!("cmpm.{} (a{})+,(a{})+", size, ay, ax),
            Self::ADDX { rx, ry, rm, size } => {
                if rm == 0 {
                    format!("addx.{} d{},d{}", size, ry, rx)
                } else {
                    format!("addx.{} -(a{}),-(a{})", size, ry, rx)
                }
            }
            Self::SUBX { rx, ry, rm, size } => {
                if rm == 0 {
                    format!("subx.{} d{},d{}", size, ry, rx)
                } else {
                    format!("subx.{} -(a{}),-(a{})", size, ry, rx)
                }
            }
            Self::ADDA { register, opmode, mode } => {
                let size = Size::from_opcode(opmode / 4 + 1);
                format!("adda.{} {},a{}", size, mode, register)
            }
            Self::SUBA { register, opmode, mode } => {
                let size = Size::from_opcode(opmode / 4 + 1);
                format!("suba.{} {},a{}", size, mode, register)
            }
            Self::CMPA { register, opmode, mode } => {
                let size = Size::from_opcode(opmode / 4 + 1);
                format!("cmpa.{} {},a{}", size, mode, register)
            }
            Self::BCHG { register, mode } => format!("bchg d{},{}", register, mode),
            Self::BCLR { register, mode } => format!("bclr d{},{}", register, mode),
            Self::BSET { register, mode } => format!("bset d{},{}", register, mode),
            Self::BTST { register, mode } => format!("btst d{},{}", register, mode),
            Self::DIVS { register, mode } => format!("divs.w {},d{}", mode, register),
            Self::DIVU { register, mode } => format!("divu.w {},d{}", mode, register),
            Self::LEA { register, mode } => format!("lea {},a{}", mode, register),
            Self::MULS { register, mode } => format!("muls.w {},d{}", mode, register),
            Self::MULU { register, mode } => format!("divs.w {},d{}", mode, register),
            Self::NBCD { mode } => format!("nbcd {}", mode),
            Self::MOVEP { dregister, opmode, aregister, displacement } => {
                let oplength = Size::from_opcode((opmode % 2) + 1);
                let mode = AddressDisplacement(aregister, displacement);
                if (opmode - 4) / 2 == 0 {
                    format!("movep.{} {},d{}", oplength, mode, dregister)
                } else {
                    format!("movep.{} d{},{}", oplength, dregister, mode)
                }
            }
            Self::SCC { condition, mode } => format!("s{} {}", condition, mode),
            Self::ASLRREG { register, count, size, dr, ir } => {
                let shift_mode = shift_mode_asm(ir, count, cpu);
                format!("as{}.{} {},d{}", if dr == 0 { "r" } else { "l" }, size, shift_mode, register)
            }
            Self::LSLRREG { register, count, size, dr, ir } => {
                let shift_mode = shift_mode_asm(ir, count, cpu);
                format!("ls{}.{} {},d{}", if dr == 0 { "r" } else { "l" }, size, shift_mode, register)
            }
            Self::ROXLR { register, count, size, dr, ir } => {
                let shift_mode = shift_mode_asm(ir, count, cpu);
                format!("rox{}.{} {},d{}", if dr == 0 { "r" } else { "l" }, size, shift_mode, register)
            }
            Self::ROLR { register, count, size, dr, ir } => {
                let shift_mode = shift_mode_asm(ir, count, cpu);
                format!("ro{}.{} {},d{}", if dr == 0 { "r" } else { "l" }, size, shift_mode, register)
            }
            Self::MOVEQ { register, data } => format!("moveq #${:02x},d{}", data, register),
            Self::EXG { opmode, rx, ry } => match opmode {
                8 => format!("exg d{},d{}", rx, ry),
                9 => format!("exg a{},a{}", rx, ry),
                10 => format!("exg d{},a{}", rx, ry),
                _ => panic!("Invalid opmode!"),
            },
            Self::CHK { register, size, mode } => format!("chk.{} {},d{}", size, mode, register),
            Self::MOVEA { register, size, mode } => format!("movea.{} {},a{}", size, mode, register),
            Self::ADDQ { data, size, mode } => format!("addq.{} #${:0x},{}", size, data, mode),
            Self::SUBQ { data, size, mode } => format!("subq.{} #${:0x},{}", size, data, mode),
            Self::BCC { condition, displacement } => {
                let pc = (cpu.pc as i32 + displacement) as u32;
                format!("b{} ${:08x}", condition, pc)
            }
            Self::ADD { register, opmode, mode } => match opmode {
                OpMode::MemoryToRegister(size) => format!("add.{} {},d{}", size, mode, register),
                OpMode::RegisterToMemory(size) => format!("add.{} d{},{}", size, register, mode),
            },
            Self::AND { register, opmode, mode } => match opmode {
                OpMode::MemoryToRegister(size) => format!("and.{} {},d{}", size, mode, register),
                OpMode::RegisterToMemory(size) => format!("and.{} d{},{}", size, register, mode),
            },
            Self::CMP { register, opmode, mode } => match opmode {
                OpMode::MemoryToRegister(size) => format!("cmp.{} {},d{}", size, mode, register),
                OpMode::RegisterToMemory(size) => format!("cmp.{} d{},{}", size, register, mode),
            },
            Self::EOR { register, opmode, mode } => match opmode {
                OpMode::MemoryToRegister(size) => format!("eor.{} {},d{}", size, mode, register),
                OpMode::RegisterToMemory(size) => format!("eor.{} d{},{}", size, register, mode),
            },
            Self::OR { register, opmode, mode } => match opmode {
                OpMode::MemoryToRegister(size) => format!("or.{} {},d{}", size, mode, register),
                OpMode::RegisterToMemory(size) => format!("or.{} d{},{}", size, register, mode),
            },
            Self::SUB { register, opmode, mode } => match opmode {
                OpMode::MemoryToRegister(size) => format!("sub.{} {},d{}", size, mode, register),
                OpMode::RegisterToMemory(size) => format!("sub.{} d{},{}", size, register, mode),
            },
            Self::MOVE { size, destmode, srcmode } => format!("move.{} {},{}", size, srcmode, destmode),
        }
    }
}

fn privilege_violation(cpu: &mut CPU) {
    cpu.supervisor_mode(true);
    let mut ssp = cpu.ssp.as_ref().borrow_mut();
    *ssp -= 4;
    let mut ram_handle = MemoryHandle::new(None, Some(*ssp as usize), None, cpu);
    ram_handle.write(OpResult::Long(cpu.pc));
    *ssp -= 2;
    ram_handle = MemoryHandle::new(None, Some(*ssp as usize), None, cpu);
    ram_handle.write(OpResult::Word((cpu.sr & 0xffff) as u16));
    cpu.pc = 0x20;
}

fn change_bit(mode: EAMode, register: Option<usize>, extword: Option<u16>, cpu: &mut CPU, opmode: BitMode) {
    let bitnumber_word =
        if register == None { extword.unwrap() as usize } else { *cpu.dr[register.unwrap()].borrow() as usize };
    let bitnumber;
    let size;
    let handle = cpu.memory_handle(mode);
    if !handle.in_memory() {
        bitnumber = bitnumber_word % 32;
        size = Long;
    } else {
        bitnumber = bitnumber_word % 8;
        size = Byte;
    }
    let mut bitfield = handle.read(size).inner() as usize;
    let mut value = get_bit(bitfield as usize, bitnumber);
    let mut ccr = CCRFlags::new();
    ccr.z = Some(!value);
    ccr.set(cpu);
    match opmode {
        BitMode::Clear => value = false,
        BitMode::Flip => value = !value,
        BitMode::Set => value = true,
        BitMode::None => {}
    }
    set_bit(&mut bitfield, bitnumber, value);
    if opmode != BitMode::None {
        if !handle.in_memory() {
            handle.write(OpResult::Long(bitfield as u32));
        } else {
            handle.write(OpResult::Byte((bitfield & 0xff) as u8));
        }
}
}

fn aslr(handle: MemoryHandle, size: Size, shift_count: usize, dr: usize, cpu: &mut CPU) {
    let bitsize = 8 * size as usize;
    let mut ccr = CCRFlags::new();
    let mut value = handle.read(size).sign_extend() as isize;
    let mut msb;
    let msb_changed;
    let xb;
    if dr == 1 {
        let sc_or_len = shift_count.min(bitsize - 1);
        let mask = (1 << (sc_or_len + 1)) - 1;
        let section = ((value as usize) >> (bitsize - sc_or_len - 1)) & mask;
        msb_changed = (section != 0) && (section != mask);
        xb = if bitsize >= shift_count {
            get_bit(value as usize, bitsize - shift_count)
        } else {
            false
        };
        value = value.overflowing_shl(shift_count as u32).0;
        msb = (value  & (1 << (bitsize - 1))) != 0;
    } else {
        msb = value < 0;
        xb = if bitsize >= shift_count && shift_count > 0 {
            get_bit(value as usize, shift_count - 1)
        } else {
            msb
        };
        value = value.overflowing_shr(shift_count as u32).0;
        msb = (value  & (1 << (bitsize - 1))) != 0;
        msb_changed = false;
    }
    handle.write(size.from(value as usize));
    ccr.z = Some((value & ((1 << bitsize) - 1)) == 0);
    ccr.n = Some(msb);
    if shift_count != 0 {
        ccr.x = Some(xb);
        ccr.c = Some(xb);
    } else {
        ccr.c = Some(false);
    }
    ccr.v = Some(msb_changed);
    ccr.set(cpu);
}

fn lslr(handle: MemoryHandle, size: Size, shift_count: usize, dr: usize, cpu: &mut CPU) {
    let bitsize = 8 * size as usize;
    let mut ccr = CCRFlags::new();
    let mut value = handle.read(size).inner() as usize;
    let msb;
    let xb;
    if dr == 1 {
        xb = if bitsize >= shift_count {
            get_bit(value, bitsize - shift_count)
        } else {
            false
        };
        value = value.overflowing_shl(shift_count as u32).0;
        msb = get_bit(value, bitsize - 1);
    } else {
        xb = if bitsize >= shift_count && shift_count > 0 {
            get_bit(value, shift_count - 1)
        } else {
            false
        };
        value = value.overflowing_shr(shift_count as u32).0;
        msb = get_bit(value, bitsize - 1);
    }
    handle.write(size.from(value));
    ccr.z = Some(value & ((1 << bitsize) - 1) == 0);
    ccr.n = Some(msb);
    if shift_count != 0 {
        ccr.x = Some(xb);
        ccr.c = Some(xb);
    } else {
        ccr.c = Some(false);
    }
    ccr.v = Some(false);
    ccr.set(cpu);
}

fn rolr(handle: MemoryHandle, size: Size, shift_count: u32, dr: usize, cpu: &mut CPU) {
    let mut ccr = CCRFlags::new();
    ccr.v = Some(false);
    match size {
        Size::Byte => {
            let mut value = handle.read(Byte).inner() as u8;
            if dr == 0 {
                value = value.rotate_right(shift_count);
            } else {
                value = value.rotate_left(shift_count);
            }
            handle.write(OpResult::Byte(value));
            ccr.z = Some(value == 0);
            ccr.n = Some((value as i8) < 0);
            ccr.c = if shift_count != 0 {
                if dr == 0 {
                    Some(get_bit(value as usize, 7))
                } else {
                    Some(get_bit(value as usize, 0))
                }
            } else {
                Some(false)
            };
            ccr.set(cpu);
        }
        Size::Word => {
            let mut value = handle.read(Word).inner() as u16;
            if dr == 0 {
                value = value.rotate_right(shift_count);
            } else {
                value = value.rotate_left(shift_count);
            }
            handle.write(OpResult::Word(value));
            ccr.z = Some(value == 0);
            ccr.n = Some((value as i16) < 0);
            ccr.c = if shift_count != 0 {
                if dr == 0 {
                    Some(get_bit(value as usize, 15))
                } else {
                    Some(get_bit(value as usize, 0))
                }
            } else {
                Some(false)
            };
            ccr.set(cpu);
        }
        Size::Long => {
            let mut value = handle.read(Long).inner() as u32;
            if dr == 0 {
                value = value.rotate_right(shift_count);
            } else {
                value = value.rotate_left(shift_count);
            }
            handle.write(OpResult::Long(value));
            ccr.z = Some(value == 0);
            ccr.n = Some((value as i32) < 0);
            ccr.c = if shift_count != 0 {
                if dr == 0 {
                    Some(get_bit(value as usize, 31))
                } else {
                    Some(get_bit(value as usize, 0))
                }
            } else {
                Some(false)
            };
            ccr.set(cpu);
        }
    }
}

fn roxlr(handle: MemoryHandle, size: Size, shift_count: usize, dr: usize, cpu: &mut CPU) {
    let mut value = handle.read(size).inner() as usize;
    let bitsize = 8 * (size as usize) + 1;
    set_bit(&mut value, bitsize - 1, cpu.ccr(CCR::X));
    let d: isize = if dr == 0 { 1 } else { -1 };
    let mut result = 0;
    for j in 0..bitsize {
        let r_j: isize = ((j as isize + d * (shift_count as isize)) % (bitsize as isize) + (bitsize as isize)) % (bitsize as isize);
        set_bit(&mut result, j, get_bit(value, r_j as usize));
    }
    let mut ccr = CCRFlags::new();
    if shift_count != 0 {
        ccr.x = Some(get_bit(result, bitsize - 1));
        ccr.c = Some(get_bit(result, bitsize - 1));
    } else {
        ccr.c = Some(cpu.ccr(CCR::X));
    }
    ccr.z = Some(result & ((1 << (bitsize - 1)) - 1) == 0);
    ccr.n = Some(get_bit(result, bitsize - 2));
    ccr.v = Some(false);
    handle.write(size.from(result));
    ccr.set(cpu);
}

fn shift_count(ir: usize, count: usize, cpu: &CPU) -> usize {
    if ir == 0 {
        if count != 0 {
            count
        } else {
            8
        }
    } else {
        (*cpu.dr[count].as_ref().borrow() % 64) as usize
    }
}

fn shift_mode_asm(ir: usize, count: usize, cpu: &CPU) -> String {
    if ir == 0 {
        format!("{}", shift_count(ir, count, cpu))
    } else {
        format!("d{}", count)
    }
}
