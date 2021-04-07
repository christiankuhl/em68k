use crate::fields::{OpResult, Size};
use crate::devices::{DeviceList, Device, Signal};
use crate::processor::CPU;
use std::cell::RefCell;
use std::rc::Rc;

pub const RAM_SIZE: usize = 1 << 24;

pub type BusPtr = Rc<RefCell<Bus>>;
pub type RegPtr = Rc<RefCell<u32>>; 

pub struct MemoryHandle {
    reg: Option<RegPtr>,
    ptr: Option<usize>,
    bus: BusPtr,
    imm: Option<OpResult>,
}

impl MemoryHandle {
    pub fn new(reg: Option<RegPtr>, ptr: Option<usize>, imm: Option<OpResult>, cpu: &CPU) -> Self {
        MemoryHandle { reg, ptr, imm, bus: Rc::clone(&cpu.bus) }
    }
    pub fn read(&self, size: Size) -> OpResult {
        if let Some(ptr) = self.ptr {
            self.bus.borrow().read(ptr, size)
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
            self.bus.borrow_mut().write(ptr, res)
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
    pub fn in_memory(&self) -> bool {
        match self.ptr {
            Some(_) => true,
            None => false,
        }
    }
}

pub struct RAM {
    mem: Vec<u8>
}

impl RAM {
    pub fn new() -> Box<Self> {
        Box::new(Self { mem: vec![0; RAM_SIZE] })
    }
}

impl Device for RAM {
    // fn init(&mut self, _ram: RamPtr) {}
    fn update(&mut self, _cpu: &CPU) -> Signal {
        Signal::Ok
    }
    fn read(&self, address: usize, size: Size) -> OpResult {
        let ptr = address & (RAM_SIZE - 1);
        match size {
            Size::Byte => OpResult::Byte(self.mem[ptr]),
            Size::Word => OpResult::Word(u16::from_be_bytes([self.mem[ptr], self.mem[ptr + 1]])),
            Size::Long => {
                OpResult::Long(u32::from_be_bytes([self.mem[ptr], self.mem[ptr + 1], self.mem[ptr + 2], self.mem[ptr + 3]]))
            }
        }
    }
    fn write(&mut self, address: usize, result: OpResult) {
        let ptr = address & (RAM_SIZE - 1);
            match result {
                OpResult::Byte(b) => self.mem[ptr] = b,
                OpResult::Word(w) => {
                    self.mem[ptr + 1] = (w & 0xff) as u8;
                    self.mem[ptr] = ((w & 0xff00) >> 8) as u8;
                }
                OpResult::Long(l) => {
                    self.mem[ptr + 3] = (l & 0xff) as u8;
                    self.mem[ptr + 2] = ((l & 0xff00) >> 8) as u8;
                    self.mem[ptr + 1] = ((l & 0xff0000) >> 16) as u8;
                    self.mem[ptr] = ((l & 0xff000000) >> 24) as u8;
                }
            }
    }
}

pub struct Bus {
    pub devices: DeviceList
}

impl Bus {
    pub fn new() -> Self {
        Bus { devices: DeviceList::new() }
    }
    pub fn attach(&mut self, device: Box<dyn Device>, from: usize, to: usize) {
        self.devices.push((from, to, device));
    }
    pub fn read(&self, address: usize, size: Size) -> OpResult {
        for (fromaddr, toaddr, device) in &self.devices {
            if *fromaddr <= address && *toaddr >= address {
                return device.read(address, size)
            }
        } 
        OpResult::Byte(0)
    }
    pub fn write(&mut self, address: usize, result: OpResult) {
        for (fromaddr, toaddr, device) in &mut self.devices {
            if *fromaddr <= address && *toaddr >= address {
                device.write(address, result);
                return
            }
        } 
    }
    pub fn update(&mut self, cpu: &CPU) {
        for (fromaddr, toaddr, device) in &mut self.devices {
            device.update(cpu);
        }
    }
}