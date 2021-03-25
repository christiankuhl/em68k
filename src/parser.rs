// This is where the dirty grunt work of making sense of our binary input happens.

use crate::fields::{Condition, EAMode, Size, OpMode};
use crate::instructions::ExtensionWord::*;
use crate::instructions::Instruction::*;
use crate::instructions::{ExtensionWord, Instruction};
use crate::processor::CPU;

// Specificity 16 - full word opcodes
const _ANDICCR: u16 = 0x23c;
const _ANDISR: u16 = 0x27c;
const _EORICCR: u16 = 0xa3c;
const _EORISR: u16 = 0xa7c;
const _ILLEGAL: u16 = 0x4afc;
const _NOP: u16 = 0x4e71;
const _ORICCR: u16 = 0x3c;
const _ORISR: u16 = 0x7c;
const _RESET: u16 = 0x4e70;
const _RTE: u16 = 0x4e73;
const _RTR: u16 = 0x4e77;
const _RTS: u16 = 0x4e75;
const _STOP: u16 = 0x4e72;
const _TRAPV: u16 = 0x4e76;

// Specificity 13
// - Signature 13, 3
const _LINK: usize = 0x9ca;
const _SWAP: usize = 0x908;
const _UNLK: usize = 0x9cb;

// Specificity 12
// - Signature 12, 4
const _TRAP: usize = 0x4e4;
// - Signature 12, 1, 3
const _MOVEUSP: usize = 0x4e6;

// Specificity 10
// - Signature 10, 3, 3
const _BCHGS: usize = 0x21;
const _BCLRS: usize = 0x22;
const _BSETS: usize = 0x23;
const _BTSTS: usize = 0x20;
const _JMP: usize = 0x13b;
const _JSR: usize = 0x13a;
const _MOVECCR: usize = 0x113;
const _MOVEFROMSR: usize = 0x103;
const _MOVETOSR: usize = 0x11b;
const _NBCD: usize = 0x120;
const _PEA: usize = 0x121;
const _TAS: usize = 0x12b;
// - Signature 7, 3, 3, 3
const _EXT: usize = 0x24;

// Specificity 9
// - Signature 7, 1, 2, 3, 3
const _ASLRMEM: usize = 0x70;
const _LSLRMEM: usize = 0x71;
const _ROXLRMEM: usize = 0x72;
const _ROLRMEM: usize = 0x73;
// - Signature 4, 4, 5, 3
const _DBCC: usize = 0x19;
// - Signature 7, 1, 2, 3, 3
const _MOVEM: usize = 0x9;
// - Signature 4, 3, 5, 1, 3
const _ABCD: usize = 0xc;
const _SBCD: usize = 0x8;

// Specificity 8
// - Signature 8, 8
const _BRA: usize = 0x60;
const _BSR: usize = 0x61;
// - Signature 8, 2, 3, 3
const _ADDI: usize = 0x6;
const _ANDI: usize = 0x2;
const _CLR: usize = 0x42;
const _CMPI: usize = 0xc;
const _EORI: usize = 0xa;
const _NEG: usize = 0x44;
const _NEGX: usize = 0x40;
const _NOT: usize = 0x46;
const _ORI: usize = 0x0;
const _SUBI: usize = 0x4;
const _TST: usize = 0x4a;
// - Signature 4, 3, 1, 2, 3, 3
const _CMPM: usize = 0xb;
// - Signature 4, 3, 1, 2, 2, 1, 3
const _ADDX: usize = 0xd;
const _SUBX: usize = 0x9;

// Specificity 7
// - Signature 4, 3, 3, 3, 3
const _BCHG: usize = 0x5;
const _BCLR: usize = 0x6;
const _BSET: usize = 0x7;
const _BTST: usize = 0x4;
const _DIVS: usize = 0x7;
const _DIVU: usize = 0x5;
const _LEA: usize = 0x7;
const _MOVEP: usize = 0x1;
const _MULS: usize = 0x7;
const _MULU: usize = 0x3;

// Specificity 6
// - Signature 4, 4, 2, 3, 3
const _SCC: usize = 0x5;
// - Signature 4, 3, 1, 2, 1, 2, 3
const _ASLRREG: usize = 0x0;
const _LSLRREG: usize = 0x1;
const _ROXLR: usize = 0x2;
const _ROLR: usize = 0x3;

// Specificity 5
// - Signature 4, 3, 1, 8
const _MOVEQ: usize = 0x7;
// - Signature 4, 3, 1, 5, 3
const _EXG: usize = 0xc;
// - Signature 4, 3, 2, 1, 3, 3
const _CHK: usize = 0x4;
// - Signature 4, 3, 1, 2, 3, 3
const _ADDQ: usize = 0x0;
const _SUBQ: usize = 0x1;
// - Signature 2, 2, 3, 3, 3, 3
const _MOVEA: usize = 0x0;

// Specificity 4
// - Signature 4, 3, 3, 3, 3
const _ADD: usize = 0xd;
const _AND: usize = 0xc;
const _CMP: usize = 0xb;
const _EOR: usize = 0xb;
const _OR: usize = 0x8;
const _SUB: usize = 0x9;
// - Signature 4, 4, 8
const _BCC: usize = 0x6;

// Specificity 2
// - Signature 2, 2,3, 3, 3, 3
const _MOVE: usize = 0x0;

// Extension word formats
const _BEW: usize = 0x0;
const _FEW: usize = 0x1;

pub fn split_instruction(word: u16, lengths: Vec<usize>) -> Vec<usize> {
    let mut result = vec![0; lengths.len()];
    let mut bits = [0; 16];
    for j in 0..16 {
        bits[15 - j] = ((word & (1 << j)) >> j).into();
    }
    let mut current = &bits[..];
    for (j, &length) in lengths.iter().enumerate() {
        let (part, rest) = current.split_at(length);
        current = rest;
        for (i, bit) in part.iter().enumerate() {
            result[j] += bit << (length - i - 1);
        }
    }
    result
}

pub fn parse_extension_word(opcode: u16) -> Option<ExtensionWord> {
    match split_instruction(opcode, vec![1, 3, 1, 2, 1, 8]).as_slice() {
        [da, register, wl, scale, _BEW, displacement] => {
            return Some(BEW { da: *da, register: *register, wl: *wl, scale: *scale, displacement: *displacement })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![1, 3, 1, 2, 1, 1, 1, 2, 1, 3]).as_slice() {
        [da, register, wl, scale, _FEW, bs, is, bdsize, 0, iis] => {
            return Some(FEW {
                da: *da,
                register: *register,
                wl: *wl,
                scale: *scale,
                bs: *bs,
                is: *is,
                bdsize: *bdsize,
                iis: *iis,
            })
        }
        _ => {}
    }
    None
}

pub fn parse_instruction(opcode: u16, cpu: &mut CPU) -> Option<Instruction> {
    match opcode {
        _ANDICCR => return Some(ANDICCR { extword: cpu.next_instruction() }),
        _ANDISR => return Some(ANDISR { extword: cpu.next_instruction() }),
        _EORICCR => return Some(EORICCR { extword: cpu.next_instruction() }),
        _EORISR => return Some(EORISR { extword: cpu.next_instruction() }),
        _ILLEGAL => return Some(ILLEGAL),
        _NOP => return Some(NOP),
        _ORICCR => return Some(ORICCR { extword: cpu.next_instruction() }),
        _ORISR => return Some(ORISR { extword: cpu.next_instruction() }),
        _RESET => return Some(RESET),
        _RTE => return Some(RTE),
        _RTR => return Some(RTR),
        _RTS => return Some(RTS),
        _STOP => return Some(STOP),
        _TRAPV => return Some(TRAPV),
        _ => {}
    }
    // Specificity 13
    match split_instruction(opcode, vec![13, 3]).as_slice() {
        [_LINK, register] => return Some(LINK { register: *register, displacement: cpu.next_instruction() as i16 }),
        [_SWAP, register] => return Some(SWAP { register: *register }),
        [_UNLK, register] => return Some(UNLK { register: *register }),
        _ => {}
    }
    // Specificity 12
    match split_instruction(opcode, vec![12, 4]).as_slice() {
        [_TRAP, vector] => return Some(TRAP { vector: *vector }),
        _ => {}
    }
    match split_instruction(opcode, vec![12, 1, 3]).as_slice() {
        [_MOVEUSP, dr, register] => return Some(MOVEUSP { register: *register, dr: *dr }),
        _ => {}
    }
    // Specificity 10
    match split_instruction(opcode, vec![10, 3, 3]).as_slice() {
        [_BCHGS, mode, earegister] => return Some(BCHGS { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        [_BCLRS, mode, earegister] => return Some(BCLRS { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        [_BSETS, mode, earegister] => return Some(BSETS { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        [_BTSTS, mode, earegister] => return Some(BTSTS { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        [_JMP, mode, earegister] => return Some(JMP { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        [_JSR, mode, earegister] => return Some(JSR { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        [_MOVECCR, mode, earegister] => {
            return Some(MOVECCR { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [_MOVEFROMSR, mode, earegister] => {
            return Some(MOVEFROMSR { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [_MOVETOSR, mode, earegister] => {
            return Some(MOVETOSR { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [_PEA, mode, earegister] => return Some(PEA { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        [_TAS, mode, earegister] => return Some(TAS { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        [_NBCD, mode, earegister] => return Some(NBCD { mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) }),
        _ => {}
    }
    match split_instruction(opcode, vec![7, 3, 3, 3]).as_slice() {
        [_EXT, opmode, 0, register] if opmode == &2 || opmode == &3 => {
            return Some(EXT { opmode: *opmode, register: *register })
        }
        _ => {}
    }
    // Specificity 9
    match split_instruction(opcode, vec![7, 1, 2, 3, 3]).as_slice() {
        [_ASLRMEM, dr, 3, mode, earegister] => {
            return Some(ASLRMEM { dr: *dr, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [_LSLRMEM, dr, 3, mode, earegister] => {
            return Some(LSLRMEM { dr: *dr, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [_ROXLRMEM, dr, 3, mode, earegister] => {
            return Some(ROXLRMEM { dr: *dr, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [_ROLRMEM, dr, 3, mode, earegister] => {
            return Some(ROLRMEM { dr: *dr, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![4, 4, 5, 3]).as_slice() {
        [5, condition, _DBCC, register] if condition > &1 => {
            return Some(DBCC { condition: Condition::from(*condition), register: *register })
        }
        _ => {}
    }
    // FIXME: sort this elsewhere
    match split_instruction(opcode, vec![5, 1, 3, 1, 3, 3]).as_slice() {
        [_MOVEM, dr, 1, size, mode, earegister] => {
            return Some(MOVEM {
                size: Size::from_opcode(1 << (*size + 1)),
                dr: *dr,
                mode: EAMode::from(Size::Byte, *mode, *earegister, cpu),
            })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 5, 1, 3]).as_slice() {
        [_ABCD, rx, 0x10, rm, ry] => return Some(ABCD { rx: *rx, ry: *ry, rm: *rm }),
        [_SBCD, rx, 0x10, rm, ry] => return Some(SBCD { rx: *rx, ry: *ry, rm: *rm }),
        _ => {}
    }
    // Specificity 8
    match split_instruction(opcode, vec![8, 2, 3, 3]).as_slice() {
        [_ADDI, size, mode, earegister] if size < &3 => { 
            let instr_size = Size::from_opcode(*size);
            return Some(ADDI {
                size: instr_size,
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
                operand: cpu.immediate_operand(instr_size),
            })
        }
        [_ANDI, size, mode, earegister] => {
            let instr_size = Size::from_opcode(*size);
            return Some(ANDI {
                size: instr_size,
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
                operand: cpu.immediate_operand(instr_size),
            })
        }
        [_CLR, size, mode, earegister] => {
            return Some(CLR {
                size: Size::from_opcode(*size),
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
            })
        }
        [_CMPI, size, mode, earegister] => {
            let instr_size = Size::from_opcode(*size);
            return Some(CMPI {
                size: instr_size,
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
                operand: cpu.immediate_operand(instr_size),
            })
        }
        [_EORI, size, mode, earegister] => {
            let instr_size = Size::from_opcode(*size);
            return Some(EORI {
                size: instr_size,
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
                operand: cpu.immediate_operand(instr_size),
            })
        }
        [_NEG, size, mode, earegister] => {
            return Some(NEG {
                size: Size::from_opcode(*size),
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
            })
        }
        [_NEGX, size, mode, earegister] => {
            return Some(NEGX {
                size: Size::from_opcode(*size),
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
            })
        }
        [_NOT, size, mode, earegister] => {
            return Some(NOT {
                size: Size::from_opcode(*size),
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
            })
        }
        [_ORI, size, mode, earegister] => {
            let instr_size = Size::from_opcode(*size);
            return Some(ORI {
                size: instr_size,
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
                operand: cpu.immediate_operand(instr_size),
            })
        }
        [_SUBI, size, mode, earegister] => {
            let instr_size = Size::from_opcode(*size);
            return Some(SUBI {
                size: instr_size,
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
                operand: cpu.immediate_operand(instr_size),
            })
        }
        [_TST, size, mode, earegister] => {
            return Some(TST {
                size: Size::from_opcode(*size),
                mode: EAMode::from(Size::from_opcode(*size), *mode, *earegister, cpu),
            })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![8, 8]).as_slice() {
        [_BRA, displacement] => return Some(BRA { displacement: *displacement }),
        [_BSR, displacement] => return Some(BSR { displacement: *displacement }),
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 2, 3, 3]).as_slice() {
        [_CMPM, ax, 1, size, 1, ay] => return Some(CMPM { ax: *ax, ay: *ay, size: Size::from_opcode(*size) }),
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 2, 2, 1, 3]).as_slice() {
        [_ADDX, rx, 1, size, 0, rm, ry] => {
            return Some(ADDX { rx: *rx, ry: *ry, rm: *rm, size: Size::from_opcode(*size) })
        }
        [_SUBX, rx, 1, size, 0, rm, ry] => {
            return Some(SUBX { rx: *rx, ry: *ry, rm: *rm, size: Size::from_opcode(*size) })
        }
        _ => {}
    }
    // Specificity 7
    match split_instruction(opcode, vec![4, 3, 3, 3, 3]).as_slice() {
        [0x0, register, _BCHG, mode, earegister] => {
            return Some(BCHG { register: *register, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [0x0, register, _BCLR, mode, earegister] => {
            return Some(BCLR { register: *register, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [0x0, register, _BSET, mode, earegister] => {
            return Some(BSET { register: *register, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [0x0, register, _BTST, mode, earegister] => {
            return Some(BTST { register: *register, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [0x8, register, _DIVS, mode, earegister] => {
            return Some(DIVS { register: *register, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [0x8, register, _DIVU, mode, earegister] => {
            return Some(DIVU { register: *register, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [0x4, register, _LEA, mode, earegister] => {
            return Some(LEA { register: *register, mode: EAMode::from(Size::Long, *mode, *earegister, cpu) })
        }
        [0xc, register, _MULS, mode, earegister] => {
            return Some(MULS { register: *register, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [0xc, register, _MULU, mode, earegister] => {
            return Some(MULU { register: *register, mode: EAMode::from(Size::Byte, *mode, *earegister, cpu) })
        }
        [0x0, dregister, opmode, _MOVEP, aregister] if opmode > &4 => {
            return Some(MOVEP { dregister: *dregister, opmode: *opmode, aregister: *aregister })
        }
        _ => {}
    }
    // Specificity 6
    match split_instruction(opcode, vec![4, 4, 2, 3, 3]).as_slice() {
        [_SCC, condition, 3, mode, earegister] if condition > &1 => {
            return Some(SCC {
                condition: Condition::from(*condition),
                mode: EAMode::from(Size::Byte, *mode, *earegister, cpu),
            })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 2, 1, 2, 3]).as_slice() {
        [0xe, count, dr, size, ir, _ASLRREG, register] => {
            return Some(ASLRREG {
                register: *register,
                count: *count,
                size: Size::from_opcode(*size),
                dr: *dr,
                ir: *ir,
            })
        }
        [0xe, count, dr, size, ir, _LSLRREG, register] => {
            return Some(LSLRREG {
                register: *register,
                count: *count,
                size: Size::from_opcode(*size),
                dr: *dr,
                ir: *ir,
            })
        }
        [0xe, count, dr, size, ir, _ROXLR, register] => {
            return Some(ROXLR { register: *register, count: *count, size: Size::from_opcode(*size), dr: *dr, ir: *ir })
        }
        [0xe, count, dr, size, ir, _ROLR, register] => {
            return Some(ROLR { register: *register, count: *count, size: Size::from_opcode(*size), dr: *dr, ir: *ir })
        }
        _ => {}
    }
    // Specificity 5
    match split_instruction(opcode, vec![4, 3, 1, 8]).as_slice() {
        [_MOVEQ, register, 0, data] => return Some(MOVEQ { register: *register, data: *data }),
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 5, 3]).as_slice() {
        [_EXG, rx, 1, opmode, ry] if opmode == &8 || opmode == &9 || opmode == &17 => {
            return Some(EXG { opmode: *opmode, rx: *rx, ry: *ry })
        }
        _ => {}
    }
    // Specificity 5
    match split_instruction(opcode, vec![4, 3, 2, 1, 3, 3]).as_slice() {
        [_CHK, register, size, 0, mode, earegister] if size == &2 || size == &3 => {
            let opsize = Size::from_opcode(4 - *size);
            return Some(CHK { register: *register, size: opsize, mode: EAMode::from(opsize, *mode, *earegister, cpu) });
        }
        _ => {}
    }
    match split_instruction(opcode, vec![2, 2, 3, 3, 3, 3]).as_slice() {
        [_MOVEA, size, register, 1, mode, earegister] if size == &2 || size == &3 => {
            let opsize = Size::from_opcode(4 - *size);
            return Some(MOVEA {
                register: *register,
                size: opsize,
                mode: EAMode::from(opsize, *mode, *earegister, cpu),
            });
        }
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 2, 3, 3]).as_slice() {
        [0x5, data, _ADDQ, size, mode, earegister] => {
            let opsize = Size::from_opcode(1 << (4 - *size));
            return Some(ADDQ { data: *data, size: opsize, mode: EAMode::from(opsize, *mode, *earegister, cpu) });
        }
        [0x5, data, _SUBQ, size, mode, earegister] => {
            let opsize = Size::from_opcode(1 << (4 - *size));
            return Some(SUBQ { data: *data, size: opsize, mode: EAMode::from(opsize, *mode, *earegister, cpu) });
        }
        _ => {}
    }
    // Specificity 4
    match split_instruction(opcode, vec![4, 4, 8]).as_slice() {
        [_BCC, condition, displacement] if condition < &13 => {
            return Some(BCC { condition: Condition::from(*condition), displacement: *displacement })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 3, 3, 3]).as_slice() {
        [_ADD, register, opmode, mode, earegister] if opmode < &6 && opmode != &3 => {
            return Some(ADD {
                register: *register,
                opmode: OpMode::from_opcode(*opmode),
                mode: EAMode::from(Size::Byte, *mode, *earegister, cpu),
            })
        }
        [_AND, register, opmode, mode, earegister] => {
            return Some(AND {
                register: *register,
                opmode: OpMode::from_opcode(*opmode),
                mode: EAMode::from(Size::Byte, *mode, *earegister, cpu),
            })
        }
        [_CMP, register, opmode, mode, earegister] if opmode < &3 => {
            return Some(CMP {
                register: *register,
                opmode: OpMode::from_opcode(*opmode),
                mode: EAMode::from(Size::Byte, *mode, *earegister, cpu),
            })
        }
        [_EOR, register, opmode, mode, earegister] if opmode > &3 => {
            return Some(EOR {
                register: *register,
                opmode: OpMode::from_opcode(*opmode),
                mode: EAMode::from(Size::Byte, *mode, *earegister, cpu),
            })
        }
        [_OR, register, opmode, mode, earegister] => {
            return Some(OR {
                register: *register,
                opmode: OpMode::from_opcode(*opmode),
                mode: EAMode::from(Size::Byte, *mode, *earegister, cpu),
            })
        }
        [_SUB, register, opmode, mode, earegister] => {
            return Some(SUB {
                register: *register,
                opmode: OpMode::from_opcode(*opmode),
                mode: EAMode::from(Size::Byte, *mode, *earegister, cpu),
            })
        }
        _ => {}
    }
    // Specificity 2
    match split_instruction(opcode, vec![2, 2, 3, 3, 3, 3]).as_slice() {
        [_MOVE, size, destreg, destmode, srcmode, srcreg] if size <= &3 && size > &0 => {
            let opsize = Size::from_opcode((4 - *size) % 3);
            return Some(MOVE {
                size: opsize,
                destmode: EAMode::from(opsize, *destmode, *destreg, cpu),
                srcmode: EAMode::from(opsize, *srcmode, *srcreg, cpu),
            });
        }
        _ => {}
    }
    None
}
