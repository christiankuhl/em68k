use crate::CPU;

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
    FEW { da: usize, register: usize, wl: usize, scale: usize, bs: usize, is: usize, bdsize: usize, iis: usize }
}

impl ExtensionWord {
    pub fn remaining_length(&self) -> (usize, usize) {
        match *self {
            Self::FEW { da, register, wl, scale, bs, is, bdsize, iis } => {
                let mut bdsize_out;
                if bdsize ==  2 || bdsize == 3 {
                    bdsize_out = bdsize - 1;
                } else {
                    bdsize_out = 0;
                }
                match iis {
                    2 | 6 => (bdsize_out, 1),
                    3 | 7 => (bdsize_out, 2),
                    _  => (bdsize_out, 0),
                }
            },
            _ => (0, 0)
        }
    }
}

impl Instruction {
    pub fn execute(&self, cpu: &mut CPU) {
        match *self  {
            Self::ANDICCR => {},
            Self::ANDISR => {},
            Self::EORICCR => {},
            Self::EORISR => {},
            Self::ILLEGAL => {},
            Self::NOP => {},
            Self::ORICCR => {},
            Self::ORISR => {},
            Self::RESET => {},
            Self::RTE => {},
            Self::RTR => {},
            Self::RTS => {},
            Self::STOP => {},
            Self::TRAPV => {},
            Self::LINK { register } => {},
            Self::SWAP { register } => {},
            Self::UNLK { register } => {},
            Self::TRAP { vector } => {},
            Self::MOVEUSP { register, dr } => {},
            Self::BCHGS { mode, earegister } => {},
            Self::BCLRS { mode, earegister } => {},
            Self::BSETS { mode, earegister } => {},
            Self::BTSTS { mode, earegister } => {},
            Self::JMP { mode, earegister } => {},
            Self::JSR { mode, earegister } => {},
            Self::MOVECCR { mode, earegister } => {},
            Self::MOVEFROMSR { mode, earegister } => {},
            Self::MOVETOSR { mode, earegister } => {},
            Self::PEA { mode, earegister } => {},
            Self::TAS { mode, earegister } => {},
            Self::EXT { mode, earegister } => {},
            Self::ASLRMEM { mode, earegister } => {},
            Self::LSLRMEM { mode, earegister } => {},
            Self::DBCC { condition, register } => {},
            Self::MOVEM { size, dr, mode, earegister } => {},
            Self::ABCD { rx, ry, rm } => {},
            Self::SBCD { rx, ry, rm } => {},
            Self::ADDI { size, mode, earegister } => {},
            Self::ANDI { size, mode, earegister } => {},
            Self::CLR { size, mode, earegister } => {},
            Self::CMPI { size, mode, earegister } => {},
            Self::EORI { size, mode, earegister } => {},
            Self::NEG { size, mode, earegister } => {},
            Self::NEGX { size, mode, earegister } => {},
            Self::NOT { size, mode, earegister } => {},
            Self::ORI { size, mode, earegister } => {},
            Self::SUBI { size, mode, earegister } => {},
            Self::TST { size, mode, earegister } => {},
            Self::BRA { displacement } => {},
            Self::BSR { displacement } => {},
            Self::CMPM { ax, ay, size } => {},
            Self::ADDX { rx, ry, rm, size } => {},
            Self::SUBX { rx, ry, rm, size } => {},
            Self::BCHG { register, mode, earegister } => {},
            Self::BCLR { register, mode, earegister } => {},
            Self::BSET { register, mode, earegister } => {},
            Self::BTST { register, mode, earegister } => {},
            Self::DIVS { register, mode, earegister } => {},
            Self::DIVU { register, mode, earegister } => {},
            Self::LEA { register, mode, earegister } => {},
            Self::MULS { register, mode, earegister } => {},
            Self::MULU { register, mode, earegister } => {},
            Self::NBCD { register, mode, earegister } => {},
            Self::MOVEP { register, mode, earegister } => {},
            Self::SCC { condition, mode, earegister } => {},
            Self::ASLRREG { register, count, size, dr, lr } => {},
            Self::LSLRREG { register, count, size, dr, lr } => {},
            Self::ROXLR { register, count, size, dr, lr } => {},
            Self::ROLR { register, count, size, dr, lr } => {},
            Self::MOVEQ { register, data } => {},
            Self::EXG { mode, rx, ry } => {},
            Self::CHK { register, size, mode, earegister } => {},
            Self::MOVEA { register, size, mode, earegister } => {},
            Self::ADDQ { data, size, mode, earegister } => {},
            Self::SUBQ { data, size, mode, earegister } => {},
            Self::BCC { condition, displacement } => {},
            Self::ADD { register, opmode, mode, earegister } => {},
            Self::AND { register, opmode, mode, earegister } => {},
            Self::CMP { register, opmode, mode, earegister } => {},
            Self::EOR { register, opmode, mode, earegister } => {},
            Self::OR { register, opmode, mode, earegister } => {},
            Self::SUB { register, opmode, mode, earegister } => {},
            Self::MOVE { size, destreg, destmode, srcmode, srcreg }  => {},           
        }
    }
    pub fn length(&self) -> u32 {
        0
    }
}
