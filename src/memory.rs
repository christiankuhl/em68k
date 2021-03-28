use crate::fields::{OpResult, Size};
use crate::processor::CPU;
use std::cell::RefCell;
use std::rc::Rc;

pub const RAM_SIZE: usize = 1 << 24;

pub type RamPtr = Rc<RefCell<Vec<u8>>>;
pub type RegPtr = Rc<RefCell<u32>>; 

pub struct MemoryHandle {
    reg: Option<RegPtr>,
    ptr: Option<usize>,
    mem: Option<RamPtr>,
    imm: Option<OpResult>,
}

impl MemoryHandle {
    pub fn new(reg: Option<RegPtr>, ptr: Option<usize>, imm: Option<OpResult>, cpu: &CPU) -> Self {
        MemoryHandle { reg, ptr, imm, mem: Some(Rc::clone(&cpu.ram)) }
    }
    pub fn read(&self, size: Size) -> OpResult {
        if let Some(ptr) = self.ptr {
            let ptr = ptr & (RAM_SIZE - 1);
            if let Some(mem) = &self.mem {
                let raw_mem = mem.as_ref().borrow();
                match size {
                    Size::Byte => OpResult::Byte(raw_mem[ptr]),
                    Size::Word => OpResult::Word(u16::from_be_bytes([raw_mem[ptr], raw_mem[ptr + 1]])),
                    Size::Long => {
                        OpResult::Long(u32::from_be_bytes([raw_mem[ptr], raw_mem[ptr + 1], raw_mem[ptr + 2], raw_mem[ptr + 3]]))
                    }
                }
            } else {
                panic!("Invalid memory handle!")
            }
        } else if let Some(reg) = &self.reg {
            let raw_mem = reg.as_ref().borrow();
            size.from(*raw_mem)
        } else if let Some(data) = &self.imm {
            *data
        } else {
            panic!("Invalid memory handle!")
        }
    }
    pub fn write(&self, res: OpResult) {
        if let Some(ptr) = self.ptr {
            if let Some(mem) = &self.mem {
                let ptr = ptr & (RAM_SIZE - 1);
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
    pub fn ptr(&self) -> Option<usize> {
        self.ptr
    }
}
