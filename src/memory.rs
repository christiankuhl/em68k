use crate::fields::{OpResult, Size};
use crate::devices::{DeviceList, Device, Signal};
use crate::processor::CPU;
use std::cell::RefCell;
use std::rc::Rc;

pub type BusPtr = Rc<RefCell<Bus>>;
pub type RegPtr = Rc<RefCell<u32>>; 
pub type MemoryRange = Vec<(usize, usize)>;

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
            self.bus.borrow_mut().read(ptr, size)
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

pub struct Bus {
    pub devices: DeviceList
}

impl Bus {
    pub fn new() -> Self {
        Bus { devices: DeviceList::new() }
    }
    pub fn attach(&mut self, device: Box<dyn Device>) {
        self.devices.push((device.memconfig(), device));
    }
    pub fn read(&mut self, address: usize, size: Size) -> OpResult {
        for (range, device) in &mut self.devices {
            for (fromaddr, toaddr) in range {
                if *fromaddr <= address && *toaddr > address {
                    return device.read(address, size)
                }
            }
        } 
        panic!(format!("Address {:08x} is not assigned!", address))
    }
    pub fn write(&mut self, address: usize, result: OpResult) {
        let mut written = false;
        for (range, device) in &mut self.devices {
            let mut remap = false;
            for (fromaddr, toaddr) in range.iter() {
                if *fromaddr <= address && *toaddr > address {
                    match device.write(address, result) {
                        Signal::Remap => {
                            remap = true;
                            written = true;
                            break;
                        }
                        _ => ()
                    }
                    written = true;
                }
            }
            if remap {
                *range = device.memconfig();
            }
        }
        if !written {
            panic!(format!("Address {:08x} is not assigned!", address))
        }
    }
}