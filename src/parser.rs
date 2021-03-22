use crate::instructions::ExtensionWord::*;
use crate::instructions::Instruction::*;
use crate::instructions::{ExtensionWord, Instruction, Condition};
use crate::memory::Size;

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
const _PEA: usize = 0x121;
const _TAS: usize = 0x12b;
// - Signature 7, 3, 3, 3
const _EXT: usize = 0x24;

// Specificity 9
// - Signature 7, 1, 2, 3, 3
const _ASLRMEM: usize = 0x70;
const _LSLRMEM: usize = 0x71;
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
const _NBCD: usize = 0x5;

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

pub fn parse_instruction(opcode: u16) -> Option<Instruction> {
    match opcode {
        _ANDICCR => return Some(ANDICCR),
        _ANDISR => return Some(ANDISR),
        _EORICCR => return Some(EORICCR),
        _EORISR => return Some(EORISR),
        _ILLEGAL => return Some(ILLEGAL),
        _NOP => return Some(NOP),
        _ORICCR => return Some(ORICCR),
        _ORISR => return Some(ORISR),
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
        [_LINK, register] => return Some(LINK { register: *register }),
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
        [_BCHGS, mode, earegister] => return Some(BCHGS { mode: *mode, earegister: *earegister }),
        [_BCLRS, mode, earegister] => return Some(BCLRS { mode: *mode, earegister: *earegister }),
        [_BSETS, mode, earegister] => return Some(BSETS { mode: *mode, earegister: *earegister }),
        [_BTSTS, mode, earegister] => return Some(BTSTS { mode: *mode, earegister: *earegister }),
        [_JMP, mode, earegister] => return Some(JMP { mode: *mode, earegister: *earegister }),
        [_JSR, mode, earegister] => return Some(JSR { mode: *mode, earegister: *earegister }),
        [_MOVECCR, mode, earegister] => return Some(MOVECCR { mode: *mode, earegister: *earegister }),
        [_MOVEFROMSR, mode, earegister] => return Some(MOVEFROMSR { mode: *mode, earegister: *earegister }),
        [_MOVETOSR, mode, earegister] => return Some(MOVETOSR { mode: *mode, earegister: *earegister }),
        [_PEA, mode, earegister] => return Some(PEA { mode: *mode, earegister: *earegister }),
        [_TAS, mode, earegister] => return Some(TAS { mode: *mode, earegister: *earegister }),
        _ => {}
    }
    match split_instruction(opcode, vec![7, 3, 3, 3]).as_slice() {
        [_EXT, opmode, 0, register] if opmode == &2 || opmode == &3 => return Some(EXT { opmode: *opmode, register: *register }),
        _ => {}
    }
    // Specificity 9
    match split_instruction(opcode, vec![7, 1, 2, 3, 3]).as_slice() {
        [_ASLRMEM, dr, 3, mode, earegister] => return Some(ASLRMEM { dr: *dr, mode: *mode, earegister: *earegister }),
        [_LSLRMEM, dr, 3, mode, earegister] => return Some(LSLRMEM { dr: *dr, mode: *mode, earegister: *earegister }),
        _ => {}
    }
    match split_instruction(opcode, vec![4, 4, 5, 3]).as_slice() {
        [5, condition, _DBCC, register] if condition > &1 => return Some(DBCC { condition: Condition::from(*condition), register: *register }),
        _ => {}
    }
    // FIXME: sort this elsewhere
    match split_instruction(opcode, vec![5, 1, 3, 1, 3, 3]).as_slice() {
        [_MOVEM, dr, 1, size, mode, earegister] => {
            return Some(MOVEM { size: Size::from(1 << (*size + 1)), dr: *dr, mode: *mode, earegister: *earegister })
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
        [_ADDI, size, mode, earegister] if size < &3 => return Some(ADDI { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_ANDI, size, mode, earegister] => return Some(ANDI { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_CLR, size, mode, earegister] => return Some(CLR { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_CMPI, size, mode, earegister] => return Some(CMPI { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_EORI, size, mode, earegister] => return Some(EORI { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_NEG, size, mode, earegister] => return Some(NEG { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_NEGX, size, mode, earegister] => return Some(NEGX { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_NOT, size, mode, earegister] => return Some(NOT { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_ORI, size, mode, earegister] => return Some(ORI { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_SUBI, size, mode, earegister] => return Some(SUBI { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        [_TST, size, mode, earegister] => return Some(TST { size: Size::from(*size), mode: *mode, earegister: *earegister }),
        _ => {}
    }
    match split_instruction(opcode, vec![8, 8]).as_slice() {
        [_BRA, displacement] => return Some(BRA { displacement: *displacement }),
        [_BSR, displacement] => return Some(BSR { displacement: *displacement }),
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 2, 3, 3]).as_slice() {
        [_CMPM, ax, 1, size, 1, ay] => return Some(CMPM { ax: *ax, ay: *ay, size: Size::from(*size) }),
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 2, 2, 1, 3]).as_slice() {
        [_ADDX, rx, 1, size, 0, rm, ry] => return Some(ADDX { rx: *rx, ry: *ry, rm: *rm, size: Size::from(*size) }),
        [_SUBX, rx, 1, size, 0, rm, ry] => return Some(SUBX { rx: *rx, ry: *ry, rm: *rm, size: Size::from(*size) }),
        _ => {}
    }
    // Specificity 7
    match split_instruction(opcode, vec![4, 3, 3, 3, 3]).as_slice() {
        [0x0, register, _BCHG, mode, earegister] => {
            return Some(BCHG { register: *register, mode: *mode, earegister: *earegister })
        }
        [0x0, register, _BCLR, mode, earegister] => {
            return Some(BCLR { register: *register, mode: *mode, earegister: *earegister })
        }
        [0x0, register, _BSET, mode, earegister] => {
            return Some(BSET { register: *register, mode: *mode, earegister: *earegister })
        }
        [0x0, register, _BTST, mode, earegister] => {
            return Some(BTST { register: *register, mode: *mode, earegister: *earegister })
        }
        [0x8, register, _DIVS, mode, earegister] => {
            return Some(DIVS { register: *register, mode: *mode, earegister: *earegister })
        }
        [0x8, register, _DIVU, mode, earegister] => {
            return Some(DIVU { register: *register, mode: *mode, earegister: *earegister })
        }
        [0x4, register, _LEA, mode, earegister] => {
            return Some(LEA { register: *register, mode: *mode, earegister: *earegister })
        }
        [0xc, register, _MULS, mode, earegister] => {
            return Some(MULS { register: *register, mode: *mode, earegister: *earegister })
        }
        [0xc, register, _MULU, mode, earegister] => {
            return Some(MULU { register: *register, mode: *mode, earegister: *earegister })
        }
        [0xc, register, _NBCD, mode, earegister] => {
            return Some(NBCD { register: *register, mode: *mode, earegister: *earegister })
        }
        [0x0, dregister, opmode, _MOVEP, aregister] if opmode > &4 => {
            return Some(MOVEP { dregister: *dregister, opmode: *opmode, aregister: *aregister })
        }
        _ => {}
    }
    // Specificity 6
    match split_instruction(opcode, vec![4, 4, 2, 3, 3]).as_slice() {
        [_SCC, condition, 3, mode, earegister] if condition > &1 => {
            return Some(SCC { condition: Condition::from(*condition), mode: *mode, earegister: *earegister })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 2, 1, 2, 3]).as_slice() {
        [0xe, count, dr, size, ir, _ASLRREG, register] => {
            return Some(ASLRREG { register: *register, count: *count, size: Size::from(*size), dr: *dr, ir: *ir })
        }
        [0xe, count, dr, size, ir, _LSLRREG, register] => {
            return Some(LSLRREG { register: *register, count: *count, size: Size::from(*size), dr: *dr, ir: *ir })
        }
        [0xe, count, dr, size, ir, _ROXLR, register] => {
            return Some(ROXLR { register: *register, count: *count, size: Size::from(*size), dr: *dr, ir: *ir })
        }
        [0xe, count, dr, size, ir, _ROLR, register] => {
            return Some(ROLR { register: *register, count: *count, size: Size::from(*size), dr: *dr, ir: *ir })
        }
        _ => {}
    }
    // Specificity 5
    match split_instruction(opcode, vec![4, 3, 1, 8]).as_slice() {
        [_MOVEQ, register, 0, data] => return Some(MOVEQ { register: *register, data: *data }),
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 5, 3]).as_slice() {
        [_EXG, rx, 1, mode, ry] => return Some(EXG { mode: *mode, rx: *rx, ry: *ry }),
        _ => {}
    }
    // Specificity 5
    match split_instruction(opcode, vec![4, 3, 2, 1, 3, 3]).as_slice() {
        [_CHK, register, size, 0, mode, earegister] if size == &2 || size == &3 => {
            return Some(CHK { register: *register, size: Size::from(4 - *size), mode: *mode, earegister: *earegister })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![2, 2, 3, 3, 3, 3]).as_slice() {
        [_MOVEA, size, register, 1, mode, earegister] if size == &2 || size == &3 => {
            return Some(MOVEA { register: *register, size: Size::from(4 - *size), mode: *mode, earegister: *earegister })
        }
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 1, 2, 3, 3]).as_slice() {
        [0x5, data, _ADDQ, size, mode, earegister] => {
            return Some(ADDQ { data: *data, size: Size::from(1 << (4 - *size)), mode: *mode, earegister: *earegister })
        }
        [0x5, data, _SUBQ, size, mode, earegister] => {
            return Some(SUBQ { data: *data, size: Size::from(1 << (4 - *size)), mode: *mode, earegister: *earegister })
        }
        _ => {}
    }
    // Specificity 4
    match split_instruction(opcode, vec![4, 4, 8]).as_slice() {
        [_BCC, condition, displacement] if condition < &13 => return Some(BCC { condition: Condition::from(*condition), displacement: *displacement }),
        _ => {}
    }
    match split_instruction(opcode, vec![4, 3, 3, 3, 3]).as_slice() {
        [_ADD, register, opmode, mode, earegister] if opmode < &6 && opmode != &3 => {
            return Some(ADD { register: *register, opmode: *opmode, mode: *mode, earegister: *earegister })
        }
        [_AND, register, opmode, mode, earegister] => {
            return Some(AND { register: *register, opmode: *opmode, mode: *mode, earegister: *earegister })
        }
        [_CMP, register, opmode, mode, earegister] if opmode < &3 => {
            return Some(CMP { register: *register, opmode: *opmode, mode: *mode, earegister: *earegister })
        }
        [_EOR, register, opmode, mode, earegister] if opmode > &3 => {
            return Some(EOR { register: *register, opmode: *opmode, mode: *mode, earegister: *earegister })
        }
        [_OR, register, opmode, mode, earegister] => {
            return Some(OR { register: *register, opmode: *opmode, mode: *mode, earegister: *earegister })
        }
        [_SUB, register, opmode, mode, earegister] => {
            return Some(SUB { register: *register, opmode: *opmode, mode: *mode, earegister: *earegister })
        }
        _ => {}
    }
    // Specificity 2
    match split_instruction(opcode, vec![2, 2, 3, 3, 3, 3]).as_slice() {
        [_MOVE, size, destreg, destmode, srcmode, srcreg] if size <= &3 && size > &0 => {
            return Some(MOVE {
                size: Size::from((4 - *size) % 3),
                destreg: *destreg,
                destmode: *destmode,
                srcmode: *srcmode,
                srcreg: *srcreg,
            })
        }
        _ => {}
    }
    None
}




