// Here reside the definitions of opcode fields; rather than have magic numbers everywhere,
// we opt for rich types wherever it is not too ridiculous.
// The parser then becomes the naturally messy code responsible for constructing said rich types.

use std::cmp::PartialEq;
use std::mem::discriminant;
use crate::parser::parse_extension_word;
use crate::memory::OpResult;
use crate::instructions::ExtensionWord;
use crate::processor::{CPU, CCR};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Size {
    Byte = 1,
    Word = 2,
    Long = 4,
}

impl Size {
    pub fn zero(&self) -> OpResult {
        match *self {
            Self::Byte => OpResult::Byte(0),
            Self::Word => OpResult::Word(0),
            Self::Long => OpResult::Long(0),
        }
    }
}

impl Size {
    pub fn from(size: usize) -> Self {
        match size {
            0 => Self::Byte,
            1 => Self::Word,
            2 => Self::Long,
            _ => panic!("Illegal operand size!") 
        }
    }
    pub fn as_asm(&self) -> String {
        match *self {
            Self::Byte => String::from("b"),
            Self::Word => String::from("w"),
            Self::Long => String::from("l"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EAMode {
    // Data register direct mode
    DataDirect(usize),
    // Address register direct mode
    AddressDirect(usize),
    // Address register indirect mode
    AddressIndirect(usize),
    // Address register indirect with postincrement mode
    AddressPostincr(usize, Size),
    // Address register indirect with predecrement mode    
    AddressPredecr(usize, Size),
    // Address register indirect with displacement mode
    AddressDisplacement(usize, i16),
    // Address Register Indirect with Index (8-Bit Displacement) Mode
    AddressIndex8Bit(usize, usize, i8, Size, usize, usize),
    // Address Register Indirect with Index (Base Displacement) Mode
    AddressIndexBase(usize, usize, i32, Size, usize, usize),
    // Absolute Short Addressing Mode
    AbsoluteShort(usize),
    // Absolute Long Addressing Mode
    AbsoluteLong(usize), 
    // Program Counter Indirect with Displacement Mode
    PCDisplacement(i32),
    // Program Counter Indirect with Index (8-Bit Displacement) Mode
    PCIndex8Bit,
    // Program Counter Indirect with Index (Base Displacement) Mode
    PCIndexBase,
    // Program Counter Memory Indirect Preindexed Mode
    PCPreindexed,
    // Immediate Data
    Immediate(OpResult),
}

impl EAMode {
    pub fn from(size: Size, mode: usize, earegister: usize, cpu: &mut CPU) -> Self {
        match mode {
            0 => Self::DataDirect(earegister),
            1 => Self::AddressDirect(earegister),
            2 => Self::AddressIndirect(earegister),
            3 => Self::AddressPostincr(earegister, size),
            4 => Self::AddressPredecr(earegister, size),
            5 => Self::AddressDisplacement(earegister, cpu.next_instruction() as i16),
            6 => {
                let opcode = cpu.next_instruction();
                if let Some(extword) = parse_extension_word(opcode) {
                    match extword {
                        ExtensionWord::BEW { da, register: iregister, wl: _, scale, displacement } => {
                            Self::AddressIndex8Bit(earegister, iregister, (displacement & 0xff) as i8, size, scale, da)
                        }
                        ExtensionWord::FEW { da, register: iregister, wl: _, scale, bs: _, is: _, bdsize: _, iis: _ } => {
                            let mut displacement: u32 = 0;
                            let (bdsize, _) = extword.remaining_length();
                            for j in 0..bdsize {
                                displacement += (cpu.next_instruction() * (1 << (8 * (bdsize - j - 1)))) as u32;
                            }
                            Self::AddressIndexBase(earegister, iregister, displacement as i32, size, scale, da)
                        }
                    }
                } else {
                    panic!("Invalid extension word!")
                }
            }
            7 => {
                let extword = cpu.next_instruction();
                match earegister {
                    // 0 => {
                    //     // Absolute Short Addressing Mode
                    // },
                    1 => {
                        // Absolute Long Addressing Mode
                        let extword2 = cpu.next_instruction();
                        let mut ptr = extword2 as usize;
                        ptr += (extword as usize) << 16;
                        Self::AbsoluteLong(ptr)
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
                        let data = match size {
                            Size::Byte => OpResult::Byte((extword &0xff) as u8),
                            Size::Word => OpResult::Word(extword),
                            Size::Long => {
                                let extword2 = cpu.next_instruction();
                                OpResult::Long(((extword as u32) << 16) + extword2 as u32) 
                                }
                            };
                        Self::Immediate(data)
                    }
                    _ => panic!("Invalid register!"),
                }
            }
            _ => panic!("Invalid addressing mode!"),
        }    
    }
    pub fn as_asm(&self) -> String {
        match *self {
            Self::DataDirect(earegister) => format!("d{:}", earegister),
            Self::AddressDirect(earegister) => format!("a{:}", earegister),
            Self::AddressIndirect(earegister) => format!("(a{:})", earegister),
            Self::AddressPostincr(earegister, _) => format!("(a{:})+", earegister),
            Self::AddressPredecr(earegister, _) => format!("-(a{:})", earegister),
            Self::AddressDisplacement(earegister, displacement) => format!("{:x}(a{:})", displacement, earegister),
            Self::AddressIndex8Bit(earegister, iregister, displacement, size, scale, da) => {
                let da_flag = if da == 0 { "d" } else { "a" };
                format!("({:x}a{:},{:}{:}.{:}*{:})", displacement, earegister, da_flag, iregister, size.as_asm(), scale)
            }
            Self::AddressIndexBase(earegister, iregister, displacement, size, scale, da) => {
                let da_flag = if da == 0 { "d" } else { "a" };
                format!("({:x}a{:},{:}{:}.{:}*{:})", displacement, earegister, da_flag, iregister, size.as_asm(), scale)
            }
            Self::AbsoluteShort(ptr) => format!("({:04x}).w", ptr),
            Self::AbsoluteLong(ptr) => format!("({:08x}).w", ptr),
            Self::PCDisplacement(ptr) => format!("({:04x},pc", ptr),
            Self::Immediate(data) => format!("#{:}", data),
            _ => panic!("Not implemented yet!")
        }    
    }
}

impl PartialEq for EAMode {
    fn eq(&self, other: &EAMode) -> bool {
        discriminant(&self) == discriminant(&other)
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

pub enum BitMode {
    Flip,
    Clear,
    Set,
    None
}
