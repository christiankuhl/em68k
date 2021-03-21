use std::cell::RefCell;
use std::rc::Rc;
use crate::processor::CCRFlags;

pub const RAM_SIZE: usize = 1 << 20;

pub type RamPtr = Rc<RefCell<[u8; RAM_SIZE]>>;
pub type RegPtr = Rc<RefCell<u32>>;

#[derive(Debug, Copy, Clone)]
pub enum OpResult {
    Byte(u8),
    Word(u16),
    Long(u32),
}

#[derive(Debug, Copy, Clone)]
pub enum Size {
    Byte = 1,
    Word = 2,
    Long = 4,
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

impl OpResult {
    pub fn inner(&self) -> u32 {
        match *self {
            OpResult::Byte(b) => b as u32,
            OpResult::Word(w) => w as u32,
            OpResult::Long(l) => l,
        }
    }
    pub fn sign_extend(&self) -> i32 {
        match *self {
            OpResult::Byte(b) => b as i8 as i32,
            OpResult::Word(w) => w as i16 as i32,
            OpResult::Long(l) => l as i32,
        }
    }
    pub fn sub(&self, other: OpResult) -> (OpResult, CCRFlags) {
        let mut ccr = CCRFlags::new();
        match *self {
            OpResult::Byte(dest_u) => {
                if let OpResult::Byte(src_u) = other {
                    let src = src_u as i8;
                    let dest = dest_u as i8;
                    let res = dest.overflowing_sub(src);
                    ccr.n = Some(res.0 < 0);
                    ccr.z = Some(res.0 == 0);
                    ccr.v = Some((src >= 0 && dest < 0 && res.0 >= 0) || (src < 0 && dest >= 0 && res.0 < 0));
                    ccr.c = Some((src < 0 && dest >= 0) || (res.0 < 0 && dest >= 0) || (src < 0 && res.0 < 0));
                    (OpResult::Byte(res.0 as u8), ccr)
                } else {
                    panic!("Unsupported operation!")
                }
            }
            OpResult::Word(dest_u) => {
                if let OpResult::Word(src_u) = other {
                    let src = src_u as i16;
                    let dest = dest_u as i16;
                    let res = dest.overflowing_sub(src);
                    ccr.n = Some(res.0 < 0);
                    ccr.z = Some(res.0 == 0);
                    ccr.v = Some((src >= 0 && dest < 0 && res.0 >= 0) || (src < 0 && dest >= 0 && res.0 < 0));
                    ccr.c = Some((src < 0 && dest >= 0) || (res.0 < 0 && dest >= 0) || (src < 0 && res.0 < 0));
                    (OpResult::Word(res.0 as u16), ccr)
                } else {
                    panic!("Unsupported operation!")
                }
            }
            OpResult::Long(dest_u) => {
                if let OpResult::Long(src_u) = other {
                    let src = src_u as i32;
                    let dest = dest_u as i32;
                    let res = dest.overflowing_sub(src);
                    ccr.n = Some(res.0 < 0);
                    ccr.z = Some(res.0 == 0);
                    ccr.v = Some((src >= 0 && dest < 0 && res.0 >= 0) || (src < 0 && dest >= 0 && res.0 < 0));
                    ccr.c = Some((src < 0 && dest >= 0) || (res.0 < 0 && dest >= 0) || (src < 0 && res.0 < 0));
                    (OpResult::Byte(res.0 as u8), ccr)
                } else {
                    panic!("Unsupported operation!")
                }
            }
        }
    }
    pub fn add(&self, other: OpResult) -> (OpResult, CCRFlags) {
        let mut ccr = CCRFlags::new();
        match *self {
            OpResult::Byte(dest_u) => {
                if let OpResult::Byte(src_u) = other {
                    let src = src_u as i8;
                    let dest = dest_u as i8;
                    let res = dest.overflowing_add(src);
                    ccr.n = Some(res.0 < 0);
                    ccr.z = Some(res.0 == 0);
                    ccr.v = Some((src < 0 && dest < 0 && res.0 >= 0) || (src >= 0 && dest >= 0 && res.0 < 0));
                    ccr.c = Some((src < 0 && dest < 0) || (res.0 >= 0 && dest < 0) || (src < 0 && res.0 >= 0));
                    (OpResult::Byte(res.0 as u8), ccr)
                } else {
                    panic!("Unsupported operation!")
                }
            }
            OpResult::Word(dest_u) => {
                if let OpResult::Word(src_u) = other {
                    let src = src_u as i16;
                    let dest = dest_u as i16;
                    let res = dest.overflowing_add(src);
                    ccr.n = Some(res.0 < 0);
                    ccr.z = Some(res.0 == 0);
                    ccr.v = Some((src < 0 && dest < 0 && res.0 >= 0) || (src >= 0 && dest >= 0 && res.0 < 0));
                    ccr.c = Some((src < 0 && dest < 0) || (res.0 >= 0 && dest < 0) || (src < 0 && res.0 >= 0));
                    (OpResult::Word(res.0 as u16), ccr)
                } else {
                    panic!("Unsupported operation!")
                }
            }
            OpResult::Long(dest_u) => {
                if let OpResult::Long(src_u) = other {
                    let src = src_u as i32;
                    let dest = dest_u as i32;
                    let res = dest.overflowing_add(src);
                    ccr.n = Some(res.0 < 0);
                    ccr.z = Some(res.0 == 0);
                    ccr.v = Some((src < 0 && dest < 0 && res.0 >= 0) || (src >= 0 && dest >= 0 && res.0 < 0));
                    ccr.c = Some((src < 0 && dest < 0) || (res.0 >= 0 && dest < 0) || (src < 0 && res.0 >= 0));
                    (OpResult::Byte(res.0 as u8), ccr)
                } else {
                    panic!("Unsupported operation!")
                }
            }
        }
    }
}

pub struct MemoryHandle {
    pub reg: Option<RegPtr>,
    pub ptr: Option<usize>,
    pub mem: Option<RamPtr>,
}

impl MemoryHandle {
    pub fn read(&self, size: usize) -> OpResult {
        if let Some(ptr) = self.ptr {
            if let Some(mem) = &self.mem {
                let raw_mem = mem.as_ref().borrow();
                match size {
                    1 => OpResult::Byte(raw_mem[ptr]),
                    2 => OpResult::Word(u16::from_be_bytes([raw_mem[ptr], raw_mem[ptr + 1]])),
                    4 => OpResult::Long(u32::from_be_bytes([
                        raw_mem[ptr],
                        raw_mem[ptr + 1],
                        raw_mem[ptr + 2],
                        raw_mem[ptr + 3],
                    ])),
                    _ => panic!("Invalid size!"),
                }
            } else {
                panic!("Invalid memory handle!")
            }
        } else {
            if let Some(reg) = &self.reg {
                let raw_mem = reg.as_ref().borrow();
                match size {
                    1 => OpResult::Byte((*raw_mem & 0xff) as u8),
                    2 => OpResult::Word((*raw_mem & 0xffff) as u16),
                    4 => OpResult::Long(*raw_mem & 0xffffffff),
                    _ => panic!("Invalid size!"),
                }
            } else {
                panic!("Invalid memory handle!")
            }
        }
    }
    pub fn write(&self, res: OpResult) {
        if let Some(ptr) = self.ptr {
            if let Some(mem) = &self.mem {
                let mut raw_mem = mem.as_ref().borrow_mut();
                match res {
                    OpResult::Byte(b) => raw_mem[ptr] = b,
                    OpResult::Word(w) => {
                        raw_mem[ptr] = (w & 0xff) as u8;
                        raw_mem[ptr + 1] = ((w & 0xff00) >> 8) as u8;
                    }
                    OpResult::Long(l) => {
                        raw_mem[ptr + 3] = (l & 0xff) as u8;
                        raw_mem[ptr + 2] = ((l & 0xff00) >> 8) as u8;
                        raw_mem[ptr + 1] = ((l & 0xff0000) >> 16) as u8;
                        raw_mem[ptr] = ((l & 0xff000000) >> 24) as u8;
                    }
                }
            } else {
                panic!("Invalid memory handle!")
            }
        } else {
            if let Some(reg) = &self.reg {
                let mut raw_mem = reg.as_ref().borrow_mut();
                match res {
                    OpResult::Byte(b) => {
                        *raw_mem &= 0xffffff00;
                        *raw_mem += b as u32;
                    }
                    OpResult::Word(w) => {
                        *raw_mem &= 0xffff0000;
                        *raw_mem += w as u32;
                    }
                    OpResult::Long(l) => {
                        *raw_mem = l;
                    }
                }
            } else {
                panic!("Invalid memory handle!")
            }
        }
    }
    pub fn offset(&mut self, offset: isize) {
        match self.ptr {
            Some(ptr) => self.ptr = Some((ptr as isize + offset) as usize),
            _ => {}
        }
    }
}
