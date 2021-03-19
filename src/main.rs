use std::fs;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
mod parser;
use parser::{parse_instruction, parse_extension_word};
mod instructions;
use instructions::ExtensionWord::{*};
mod memory;
use memory::MemoryHandle;

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

const RAM_SIZE: usize = 1 << 20;

type RamPtr = Rc<RefCell<[u8; RAM_SIZE]>>;
type RegPtr = Rc<RefCell<u32>>;

pub struct CPU {
    pc: u32,                        // Program counter
    ccr: u8,                        // Condition code register
    dr: [RegPtr; 8],                // Data registers
    ar: [RegPtr; 8],                // Address registers
    ram: RamPtr                     // Pointer to RAM
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
    fn initialize(&mut self, progname: &str) {
        let program = fs::read(progname).expect("Program does not exist!");
        let mut raw_mem = self.ram.as_ref().borrow_mut();
        for (j, &b) in program.iter().enumerate() {
            raw_mem[j + 0x0400] = b;
        } 
        self.cpu.pc = 0x0400
    }
    fn hardware_update(&mut self) {}
    pub fn new() -> Emulator {
        let ram = RamPtr::new(RefCell::new([0u8; RAM_SIZE]));
        let ar = [Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0))];
        let dr = [Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)),
            Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0))];
        let cpu = CPU { pc: 0, ccr: 0, dr: dr, ar: ar, ram: Rc::clone(&ram) };
        Emulator { cpu: cpu, ram: Rc::clone(&ram) }
    }
}

impl CPU {
    pub fn clock_cycle(&mut self) {
        let opcode = self.next_instruction();
        if let Some(instruction) = parse_instruction(opcode) {
            println!("{:?}", self);
            println!("Instruction: {:?}", instruction);
            println!("{:02x}", self.ram.borrow()[0xf0001]);
            pause();
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
            0 => MemoryHandle { reg: Some(Rc::clone(&self.dr[register])), ptr: None, mem: None },
            // Address register direct mode
            1 => MemoryHandle { reg: Some(Rc::clone(&self.ar[register].clone())), ptr: None, mem: None },
            // Address register indirect mode
            2 => {
                let ptr = *self.ar[register].borrow() as usize;
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            },
            // Address register indirect with postincrement mode
            3 => {
                let ptr = (*self.ar[register]).borrow().clone() as usize;
                if register == 7 && size == 1 {
                    *self.ar[register].borrow_mut() += 2;    
                } else {
                    *self.ar[register].borrow_mut() += size as u32; 
                }
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            },
            // Address register indirect with predecrement mode
            4 => {
                if register == 7 && size == 1 {
                    *self.ar[register].borrow_mut() -= 2;    
                } else {
                    *self.ar[register].borrow_mut() -= size as u32; 
                }
                let ptr = *self.ar[register].borrow() as usize;
                MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
            },           
            // Address register indirect with displacement mode
            5 => {
                let displacement = self.next_instruction() as i16;
                let ptr = (*self.ar[register].borrow() + displacement as u32) as usize;
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
                                ptr = *self.dr[iregister].borrow_mut();
                            } else {
                                ptr = *self.ar[iregister].borrow_mut();
                            }
                            ptr *= 1 << scale;
                            ptr += (displacement & 0xff) as i8 as u32;
                            MemoryHandle { reg: None, ptr: Some(ptr as usize), mem: Some(Rc::clone(&self.ram)) }
                        },
                        // Address Register Indirect with Index (Base Displacement) Mode
                        FEW { da, register: iregister, wl: _, scale, bs: _, is: _, bdsize: _, iis: _ } => {
                            if da == 0 {
                                ptr = *self.dr[iregister].borrow_mut();
                            } else {
                                ptr = *self.ar[iregister].borrow_mut();
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
            7 => {
                let extword = self.next_instruction();
                match register {
                    // 0 => {
                    //     // Absolute Short Addressing Mode
                    // },
                    1 => {
                        // Absolute Long Addressing Mode
                        let extword2 = self.next_instruction();
                        let mut ptr = extword2 as usize;
                        ptr += (extword as usize) << 16;
                        MemoryHandle { reg: None, ptr: Some(ptr), mem: Some(Rc::clone(&self.ram)) }
                    },
                    // 2 => {
                    //     // Program Counter Indirect with Displacement Mode
                    // },
                    // 3 => {
                    //     // Program Counter Indirect with Index (8-Bit Displacement) Mode
                    //     // Program Counter Indirect with Index (Base Displacement) Mode
                    //     // Program Counter Memory Indirect Preindexed Mode
                    // },
                    4 => {
                        // Immediate Data  
                        match size {
                            1 => {
                                MemoryHandle { reg: None, ptr: Some(self.pc as usize - 1), mem: Some(Rc::clone(&self.ram)) }
                            },
                            2 => {
                                MemoryHandle { reg: None, ptr: Some(self.pc as usize - 2), mem: Some(Rc::clone(&self.ram)) }
                            },
                            4 => {
                                self.pc += 2;
                                MemoryHandle { reg: None, ptr: Some(self.pc as usize - 4), mem: Some(Rc::clone(&self.ram)) }
                            }
                            _ => panic!("Unexpected operand size!")
                        }
                    },
                    _ => panic!("Invalid register!")
                }
            },
            _ => panic!("Invalid addressing mode!")
        }
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("Registers:\n\n");
        for j in 0..7 {
            s.push_str(&format!("A{j}: {a:08x}     D{j}: {d:08x}\n", j=j, a=*self.ar[j].borrow(), d=*self.dr[j].borrow()));
        }
        s.push_str(&format!("\n\nProgram Counter: {:04x}", self.pc));
        write!(f, "{}", s)
    }
}

fn main() {
    let mut em = Emulator::new();
    em.run("test.bin");
}


