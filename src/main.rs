use std::rc::Rc;
use std::cell::RefCell;
mod parser;
mod instructions;
use parser::{parse_instruction, parse_extension_word};
use instructions::ExtensionWord::{*};

const RAM_SIZE: usize = 1 << 20;

type RamPtr = Rc<RefCell<[u8; RAM_SIZE]>>;

enum Result {
    Byte(u8),
    Word(u16),
    Long(u32)
}

struct MemoryHandle {
    reg: Option<RefCell<u32>>,
    ptr: Option<usize>,
    mem: Option<RamPtr>
}

impl MemoryHandle {
    fn read(&self, size: Result) -> Result {
        if let Some(ptr) = self.ptr {
            if let Some(mem) = &self.mem {
                let raw_mem = *mem.borrow();
                match size {
                    Result::Byte(_) => Result::Byte(raw_mem[ptr]),
                    Result::Word(_) => Result::Word(u16::from_be_bytes([raw_mem[ptr], raw_mem[ptr+1]])),
                    Result::Long(_) => Result::Long(u32::from_be_bytes([raw_mem[ptr], raw_mem[ptr+1], raw_mem[ptr+2], raw_mem[ptr+3]])),
                }
            } else {
                panic!("Invalid memory handle!")
            }
        } else {
            if let Some(reg) = &self.reg {
                let raw_mem = *reg.borrow();
                match size {
                    Result::Byte(_) => Result::Byte((raw_mem & 0xff) as u8),
                    Result::Word(_) => Result::Word((raw_mem & 0xffff) as u16),
                    Result::Long(_) => Result::Long(raw_mem & 0xffffffff),
                }
            } else {
                panic!("Invalid memory handle!")
            }
        }
    }
    fn write(&self, res: Result) {
        if let Some(ptr) = self.ptr {
            if let Some(mem) = &self.mem {
                let mut raw_mem = *mem.borrow_mut();
                match res {
                    Result::Byte(b) => { raw_mem[ptr] = b },
                    Result::Word(w) => {
                        raw_mem[ptr] = (w & 0xff) as u8;
                        raw_mem[ptr+1] = ((w & 0xff00) >> 8) as u8;
                    },
                    Result::Long(l) => {
                        raw_mem[ptr] = (l & 0xff) as u8;
                        raw_mem[ptr+1] = ((l & 0xff00) >> 8) as u8;
                        raw_mem[ptr+2] = ((l & 0xff0000) >> 16) as u8;
                        raw_mem[ptr+3] = ((l & 0xff000000) >> 8) as u8;
                    },
                }
            } else {
                panic!("Invalid memory handle!")
            }
        } else {
            if let Some(reg) = &self.reg {
                let mut raw_mem = reg.borrow_mut();
                match res {
                    Result::Byte(b) => { 
                        *raw_mem &= 0xffffff00;
                        *raw_mem += b as u32;
                     },
                    Result::Word(w) => { 
                        *raw_mem &= 0xffff0000;
                        *raw_mem += w as u32;
                     },
                    Result::Long(l) => { *raw_mem = l; },
                }
            } else {
                panic!("Invalid memory handle!")
            }
        }
    }
}

pub struct CPU {
    pc: u32,                    // Program counter
    ccr: u8,                    // Condition code register
    dr: [u32; 8],               // Data registers
    ar: [u32; 8],               // Address registers
    ram: RamPtr                 // Pointer to RAM
}

pub struct Emulator {
    cpu: CPU,                 // CPU
    ram: RamPtr               // Pointer to RAM
}

impl Emulator {
    pub fn run(&mut self, program: &str) {
        self.initialize(program);
        loop {
            self.cpu.clock_cycle();
            self.hardware_update();
        }
    }
    fn initialize(&mut self, program: &str) {}
    fn hardware_update(&mut self) {}
    pub fn new() -> Emulator {
        let ram = Rc::new(RefCell::new([0u8; RAM_SIZE]));
        let cpu = CPU { pc: 0, ccr: 0, dr: [0u32; 8], ar: [0u32; 8], ram: Rc::clone(&ram) };
        Emulator { cpu: cpu, ram: Rc::clone(&ram) }
    }
}

impl CPU {
    pub fn clock_cycle(&mut self) {
        let opcode = self.next_instruction();
        if let Some(instruction) = parser::parse_instruction(opcode) {
            instruction.execute(self);
        } else {
            panic!("Illegal instruction!");
        }
    }
    fn next_instruction(&mut self) -> u16 {
        let raw_mem = *self.ram.borrow();
        let ptr = self.pc as usize;
        let instr = u16::from_be_bytes([raw_mem[ptr], raw_mem[ptr+1]]);
        self.pc += 2;
        instr
    }
    fn memory_handle(&mut self, mode: usize, register: usize, size: usize) -> MemoryHandle {
        match mode {
            // Data register direct mode
            0 => MemoryHandle { reg: Some(RefCell::new(self.dr[register])), ptr: None, mem: None },
            // Address register direct mode
            1 => MemoryHandle { reg: Some(RefCell::new(self.ar[register])), ptr: None, mem: None },
            // Address register indirect mode
            2 => {
                let ptr = self.ar[register] as usize;
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            },
            // Address register indirect with postincrement mode
            3 => {
                let ptr = self.ar[register] as usize;
                if register == 7 && size == 1 {
                    self.ar[register] += 2;    
                } else {
                    self.ar[register] += size as u32; 
                }
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            },
            // Address register indirect with predecrement mode
            4 => {
                if register == 7 && size == 1 {
                    self.ar[register] -= 2;    
                } else {
                    self.ar[register] -= size as u32; 
                }
                let ptr = self.ar[register] as usize;
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            },           
            // Address register indirect with displacement mode
            5 => {
                let displacement = self.next_instruction() as i16;
                let ptr = (self.ar[register] + displacement as u32) as usize;
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            }
            6 => {
                let opcode = self.next_instruction();
                if let Some(extword) = parse_extension_word(opcode) {
                    let mut ptr;
                    match extword {
                        // Address Register Indirect with Index (8-Bit Displacement) Mode
                        BEW {da, register: iregister, wl: _, scale, displacement } => {
                            if da == 0 {
                                ptr = self.dr[iregister];
                            } else {
                                ptr = self.ar[iregister];
                            }
                            ptr *= 1 << scale;
                            ptr += (displacement & 0xff) as i8 as u32;
                            MemoryHandle { reg: None, ptr: Some(ptr as usize), mem: Some(Rc::clone(&self.ram)) }
                        },
                        // Address Register Indirect with Index (Base Displacement) Mode
                        FEW { da, register: iregister, wl: _, scale, bs: _, is: _, bdsize: _, iis: _ } => {
                            if da == 0 {
                                ptr = self.dr[iregister];
                            } else {
                                ptr = self.ar[iregister];
                            }
                            ptr *= 1 << scale;
                            let mut displacement: u32 = 0;
                            let (bdsize, _) = extword.remaining_length();
                            for j in 0..bdsize {
                                displacement += (self.next_instruction() * (1 << (8 * (bdsize - j - 1)))) as u32;
                            }
                            ptr += displacement;
                            MemoryHandle { reg: None, ptr: Some(ptr as usize), mem: Some(Rc::clone(&self.ram)) }
                        }
                    } 
                } else { 
                    panic!("Invalid extension word!") 
                }
            },
            _ => panic!("Invalid addressing mode!")
        }
    }
}

fn main() {
    let a = RefCell::new(1);

    {
        let mut b = a.borrow_mut();
        *b = 2;
    }

    println!("{:?}", a.into_inner());    
}


