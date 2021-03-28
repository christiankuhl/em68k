// Here reside the definitions of opcode fields; rather than have magic numbers everywhere,
// we opt for rich types wherever it is not too ridiculous.
// The parser then becomes the naturally messy code responsible for constructing said rich types.

use crate::conversions::Truncate;
use crate::instructions::ExtensionWord;
use crate::memory::MemoryHandle;
use crate::parser::parse_extension_word;
use crate::processor::{CCRFlags, CCR, CPU};
use std::cmp::PartialEq;
use std::fmt;
use std::mem::discriminant;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Size {
    Byte = 1,
    Word = 2,
    Long = 4,
}

impl Size {
    pub fn from<T>(&self, res: T) -> OpResult
    where
        T: Truncate<u8> + Truncate<u16> + Truncate<u32>,
    {
        match *self {
            Self::Byte => OpResult::Byte(res.truncate()),
            Self::Word => OpResult::Word(res.truncate()),
            Self::Long => OpResult::Long(res.truncate()),
        }
    }
    pub fn zero(&self) -> OpResult {
        self.from(0u8)
    }
    pub fn from_opcode(size: usize) -> Self {
        match size {
            0 => Self::Byte,
            1 => Self::Word,
            2 => Self::Long,
            _ => panic!("Illegal operand size!"),
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
pub enum OpResult {
    Byte(u8),
    Word(u16),
    Long(u32),
}

impl OpResult {
    pub fn inner(&self) -> u32 {
        match *self {
            Self::Byte(b) => b as u32,
            Self::Word(w) => w as u32,
            Self::Long(l) => l,
        }
    }
    pub fn sign_extend(&self) -> i32 {
        match *self {
            Self::Byte(b) => b as i8 as i32,
            Self::Word(w) => w as i16 as i32,
            Self::Long(l) => l as i32,
        }
    }
    pub fn sub(&self, other: Self) -> (Self, CCRFlags) {
        let mut ccr = CCRFlags::new();
        let src = other.sign_extend();
        let dest = self.sign_extend();
        let res = dest.overflowing_sub(src);
        ccr.n = Some(res.0 < 0);
        ccr.z = Some(res.0 == 0);
        ccr.v = Some((src >= 0 && dest < 0 && res.0 >= 0) || (src < 0 && dest >= 0 && res.0 < 0));
        ccr.c = Some((src < 0 && dest >= 0) || (res.0 < 0 && dest >= 0) || (src < 0 && res.0 < 0));
        (self.size().from(res.0), ccr)
    }
    pub fn add(&self, other: Self) -> (Self, CCRFlags) {
        let mut ccr = CCRFlags::new();
        let src = self.sign_extend();
        let dest = other.sign_extend();
        let res = dest.overflowing_add(src);
        ccr.n = Some(res.0 < 0);
        ccr.z = Some(res.0 == 0);
        ccr.v = Some((src < 0 && dest < 0 && res.0 >= 0) || (src >= 0 && dest >= 0 && res.0 < 0));
        ccr.c = Some((src < 0 && dest < 0) || (res.0 >= 0 && dest < 0) || (src < 0 && res.0 >= 0));
        (self.size().from(res.0), ccr)
    }
    pub fn and(&self, other: Self) -> (Self, CCRFlags) {
        self.bitwise_op(other, |a: u32, b: u32| a & b)
    }
    pub fn or(&self, other: Self) -> (Self, CCRFlags) {
        self.bitwise_op(other, |a: u32, b: u32| a | b)
    }
    pub fn xor(&self, other: Self) -> (Self, CCRFlags) {
        self.bitwise_op(other, |a: u32, b: u32| a ^ b)
    }
    pub fn clear(&self) -> (Self, CCRFlags) {
        self.bitwise_op(*self, |a: u32, b: u32| a ^ b)
    }
    pub fn not(&self) -> (Self, CCRFlags) {
        self.bitwise_op(*self, |a: u32, _: u32| !a)
    }
    fn bitwise_op<T>(&self, other: Self, fun: T) -> (Self, CCRFlags)
    where
        T: Fn(u32, u32) -> u32,
    {
        let mut ccr = CCRFlags::new();
        let src = self.inner();
        let dest = other.inner();
        let res = fun(src, dest);
        ccr.n = Some((res as i32) < 0);
        ccr.z = Some(res == 0);
        ccr.v = Some(false);
        ccr.c = Some(false);
        (self.size().from(res), ccr)
    }
    pub fn size(&self) -> Size {
        match self {
            Self::Byte(_) => Size::Byte,
            Self::Word(_) => Size::Word,
            Self::Long(_) => Size::Long,
        }
    }
}

impl fmt::Display for OpResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OpResult::Byte(b) => write!(f, "${:02x}", b),
            OpResult::Word(w) => write!(f, "${:04x}", w),
            OpResult::Long(l) => write!(f, "${:08x}", l),
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
    // Memory Indirect Postindexed Mode
    MemoryIndirectPostindexed, // ([bd,An],Xn.SIZE*SCALE,od)
    // Memory Indirect Postindexed Mode
    MemoryIndirectPreindexed,
    // Absolute Short Addressing Mode
    AbsoluteShort(usize),
    // Absolute Long Addressing Mode
    AbsoluteLong(usize),
    // Program Counter Indirect with Displacement Mode
    PCDisplacement(i32),
    // Program Counter Indirect with Index (8-Bit Displacement) Mode
    PCIndex8Bit(usize, i8, Size, usize, usize),
    // Program Counter Indirect with Index (Base Displacement) Mode
    PCIndexBase(usize, i32, Size, usize, usize),
    // Program Counter Memory Indirect Postindexed Mode
    PCIndirectPostindexed, 
    // Program Counter Memory Indirect Preindexed Mode
    PCIndirectPreindexed,
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
                        ExtensionWord::BEW { da, register: iregister, wl: wl, scale, displacement } => {
                            Self::AddressIndex8Bit(earegister, iregister, (displacement & 0xff) as i8, size, scale, da)
                        }
                        ExtensionWord::FEW { da, register: iregister, wl: wl, scale, bs: bs, is: is, bdsize: bdsize, iis: iis } => {
                            let mut displacement: u32 = 0;
                            let (bdsize, _) = extword.remaining_length();
                            for j in 0..bdsize {
                                displacement += ((cpu.next_instruction() as u32) * (1 << (8 * (bdsize - j - 1)))) as u32;
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
                    0 => Self::AbsoluteShort(extword as i16 as usize),
                    1 => {
                        let extword2 = cpu.next_instruction();
                        let mut ptr = extword2 as usize;
                        ptr += (extword as usize) << 16;
                        Self::AbsoluteLong(ptr)
                    }
                    2 => Self::PCDisplacement(extword as i16 as i32),
                    3 => {
                        if let Some(extword) = parse_extension_word(extword) {
                            match extword {
                                ExtensionWord::BEW { da, register, wl: _, scale, displacement } => {
                                    Self::PCIndex8Bit(register, (displacement & 0xff) as i8, size, scale, da)
                                }
                                ExtensionWord::FEW { da, register, wl: _, scale, bs: _, is: _, bdsize: _, iis: _ } => {
                                    let mut displacement: u32 = 0;
                                    let (bdsize, _) = extword.remaining_length();
                                    for j in 0..bdsize {
                                        displacement += (cpu.next_instruction() * (1 << (8 * (bdsize - j - 1)))) as u32;
                                    }
                                    Self::PCIndexBase(register, displacement as i32, size, scale, da)
                                }
                            }
                        } else {
                            panic!("Invalid extension word!")
                        }
                    }
                    4 => {
                        let data = match size {
                            Size::Byte => OpResult::Byte((extword & 0xff) as u8),
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
            Self::AddressDisplacement(earegister, displacement) => format!("{:-x}(a{:})", SignedForDisplay(displacement), earegister),
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
            Self::PCDisplacement(displ) => format!("({:04x},pc)", SignedForDisplay(displ)),
            Self::Immediate(data) => format!("#{:}", data),
            _ => panic!("Not implemented yet!"),
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
    LE = 15,
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
            _ => panic!("Invalid condition code!"),
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
            Self::GT => {
                (cpu.ccr(CCR::N) && cpu.ccr(CCR::V) && !cpu.ccr(CCR::Z))
                    || (!cpu.ccr(CCR::N) && !cpu.ccr(CCR::V) && !cpu.ccr(CCR::Z))
            }
            Self::LE => cpu.ccr(CCR::Z) || (cpu.ccr(CCR::N) && !cpu.ccr(CCR::V)) || (!cpu.ccr(CCR::N) && cpu.ccr(CCR::V)),
        }
    }
}

pub enum BitMode {
    Flip,
    Clear,
    Set,
    None,
}

#[derive(Copy, Clone)]
pub enum OpMode {
    MemoryToRegister(Size),
    RegisterToMemory(Size),
}

impl OpMode {
    pub fn from_opcode(opmode: usize) -> Self {
        let size = Size::from_opcode(opmode % 4);
        match opmode >> 2 {
            0 => Self::MemoryToRegister(size),
            1 => Self::RegisterToMemory(size),
            _ => panic!("Invalid opmode!"),
        }
    }
    pub fn size(&self) -> Size {
        match *self {
            Self::MemoryToRegister(size) | Self::RegisterToMemory(size) => size,
        }
    }
    pub fn write(&self, reghandle: MemoryHandle, memhandle: MemoryHandle, result: OpResult) {
        match self {
            Self::MemoryToRegister(_) => reghandle.write(result),
            Self::RegisterToMemory(_) => memhandle.write(result),
        }
    }
}

pub struct PackedBCD(pub u8);

impl PackedBCD {
    pub fn from(res: OpResult) -> Self {
        match res {
            OpResult::Byte(b) => {
                let value = (b & 0xf) + 10 * (b & 0xf0 >> 4);
                if value > 9 {
                    panic!("Invalid BCD encoding!")
                };
                Self(value)
            }
            _ => panic!("Unsupported operation!"),
        }
    }
    pub fn pack(&self) -> OpResult {
        let low_digit = self.0 % 10;
        let high_digit = self.0 / 10;
        OpResult::Byte(low_digit + (high_digit << 4))
    }
    pub fn add(&self, other: Self, extend: bool) -> (Self, bool) {
        let result = self.0 + other.0 + extend as u8;
        let carry = result > 99;
        (Self(result % 100), carry)
    }
    pub fn sub(&self, other: Self, extend: bool) -> (Self, bool) {
        let result = self.0 as i8 - other.0 as i8 - extend as i8;
        let carry = result > 99 || result < 0;
        (Self((result.abs() % 100) as u8), carry)
    }
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for EAMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_asm())
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_asm())
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_asm())
    }
}

struct SignedForDisplay<T>(T);

impl fmt::LowerHex for SignedForDisplay<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = if f.alternate() { "0x" } else { "" };
        let bare_hex = format!("{:x}", self.0.abs());
        f.pad_integral(self.0 >= 0, prefix, &bare_hex)
    }
}

impl fmt::LowerHex for SignedForDisplay<i16> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = if f.alternate() { "0x" } else { "" };
        let bare_hex = format!("{:x}", self.0.abs());
        f.pad_integral(self.0 >= 0, prefix, &bare_hex)
    }
}