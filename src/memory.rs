use crate::fields::{OpResult, Size};
use crate::devices::{DeviceList, Device, Signal};
use crate::processor::{CPU, IRQ};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub type BusPtr = Rc<RefCell<Bus>>;
pub type RegPtr = Rc<RefCell<u32>>; 
pub type MemoryRange = Vec<(usize, usize)>;

pub struct MemoryHandle {
    pub reg: Option<RegPtr>,
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
        let trunc_address = address & 0xffffff;
        for (range, device) in &mut self.devices {
            for (fromaddr, toaddr) in range {
                if *fromaddr <= trunc_address && *toaddr > trunc_address {
                    return device.read(trunc_address, size)
                }
            }
        } 
        panic!(format!("Address {:08x} is not assigned!", trunc_address))
    }
    pub fn write(&mut self, address: usize, result: OpResult) {
        let mut written = false;
        let trunc_address = address & 0xffffff;
        for (range, device) in &mut self.devices {
            let mut remap = false;
            for (fromaddr, toaddr) in range.iter() {
                if *fromaddr <= trunc_address && *toaddr > trunc_address {
                    match device.write(trunc_address, result) {
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
            panic!(format!("Address {:08x} is not assigned!", trunc_address))
        }
    }
    pub fn interrupt_requests(&mut self) -> VecDeque<IRQ> {
        let mut irqs = VecDeque::new();
        for (_, device) in &mut self.devices {
            if let Some(irq) = device.interrupt_request() {
                irqs.push_back(irq);
            }
        }
        irqs
    }
    pub fn poll_devices(&self) -> Signal {
        let mut signal = Signal::Ok;
        for (_, device) in &self.devices {
            signal.add(&device.poll());
        }
        signal
    }
}