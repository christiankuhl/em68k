use crate::{RamPtr, RegPtr};

#[derive(Debug)]
pub enum OpResult {
    Byte(u8),
    Word(u16),
    Long(u32)
}

pub struct MemoryHandle {
    pub reg: Option<RegPtr>,
    pub ptr: Option<usize>,
    pub mem: Option<RamPtr>
}

impl MemoryHandle {
    pub fn read(&self, size: usize) -> OpResult {
        if let Some(ptr) = self.ptr {
            if let Some(mem) = &self.mem {
                let raw_mem = mem.as_ref().borrow();
                match size {
                    1 => OpResult::Byte(raw_mem[ptr]),
                    2 => OpResult::Word(u16::from_be_bytes([raw_mem[ptr], raw_mem[ptr+1]])),
                    4 => OpResult::Long(u32::from_be_bytes([raw_mem[ptr], raw_mem[ptr+1], raw_mem[ptr+2], raw_mem[ptr+3]])),
                    _ => panic!("Invalid size!")
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
                    _ => panic!("Invalid size!")
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
                    OpResult::Byte(b) => { raw_mem[ptr] = b },
                    OpResult::Word(w) => {
                        raw_mem[ptr] = (w & 0xff) as u8;
                        raw_mem[ptr+1] = ((w & 0xff00) >> 8) as u8;
                    },
                    OpResult::Long(l) => {
                        raw_mem[ptr+3] = (l & 0xff) as u8;
                        raw_mem[ptr+2] = ((l & 0xff00) >> 8) as u8;
                        raw_mem[ptr+1] = ((l & 0xff0000) >> 16) as u8;
                        raw_mem[ptr] = ((l & 0xff000000) >> 24) as u8;
                    },
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
                     },
                    OpResult::Word(w) => { 
                        *raw_mem &= 0xffff0000;
                        *raw_mem += w as u32;
                     },
                    OpResult::Long(l) => { *raw_mem = l; },
                }
            } else {
                panic!("Invalid memory handle!")
            }
        }
    }
}
