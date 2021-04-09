use crate::memory::MemoryRange;
use crate::fields::{OpResult, Size};
use crate::processor::CPU;
use std::mem::discriminant;
use minifb::{Window, WindowOptions};
use std::time::Instant;
use std::fs;

const CLKFREQ: f64 = 2457600.0;

pub type DeviceList = Vec<(MemoryRange, Box<dyn Device>)>;

pub enum Signal {
    Ok,
    Quit,
    Attach(Box<dyn Device>),
    Detach,
    NoOp,
}

pub trait Device: {
    // fn init(&mut self, bus: BusPtr);
    fn update(&mut self, cpu: &CPU) -> Signal;
    fn read(&mut self, address: usize, size: Size) -> OpResult;
    fn write(&mut self, address: usize, result: OpResult);
}


pub struct ASMStream;

impl Device for ASMStream {
    // fn init(&mut self, _ram: RamPtr) {}
    fn update(&mut self, cpu: &CPU) -> Signal {
        let mut dis = cpu.disassemble(1);
        for mut instr in dis.drain(..) {
            let mut hex = String::new();
            for opcode in instr.1.drain(..) {
                hex.push_str(&format!(" {:04x}", opcode))
            }
            println!("{:08x}:{:<30} {}", instr.0, hex, instr.2);
        }
        Signal::Ok
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) { }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Signal) -> bool {
        discriminant(&self) == discriminant(&other)
    }
}

impl Signal {
    pub fn add(&mut self, rhs: &Self) {
        match rhs {
            Self::Quit => *self = Self::Quit,
            Self::NoOp => match *self {
                Self::Quit => *self = Self::Quit,
                _ => *self = Self::NoOp,
            }
            _ => {}
        }
    }
}

pub struct Timer {
    data: u8,
    value: u8,
    counter: u8,
    now: Instant,
    ctrl: ControlMode,
    ctrl_address: usize,
    offset: usize,
    clockfreq: f64,
}

impl Timer {
    pub fn new(ctrl: usize, offset: usize, clockfreq: f64) -> Box<Self> {
        Box::new(Self { now: Instant::now(), 
                        value: 0, 
                        data: 0, 
                        counter: 0, 
                        ctrl_address: ctrl, 
                        ctrl: ControlMode::Stop(0), 
                        offset: offset,
                        clockfreq: clockfreq })
    }
}

impl Device for Timer {
    fn update(&mut self, _cpu: &CPU) -> Signal {
        Signal::Ok
    }
    fn read(&mut self, address: usize, _size: Size) -> OpResult {
        if address != self.ctrl_address {
            match self.ctrl {
                ControlMode::Delay(delay, _) | ControlMode::PulseExtension(delay, _) => {
                    let elapsed = self.now.elapsed().as_nanos();
                    let pulses = (elapsed as f64 * CLKFREQ / 1e9) as u8;
                    self.value = self.value.wrapping_sub(pulses / delay);
                    if self.value == 0 {
                        self.value = self.data;
                    }
                    if pulses / delay > 0 {
                        self.now = Instant::now();
                    }
                }
                ControlMode::EventCount(_) => {
                    let elapsed = self.now.elapsed().as_nanos();
                    let pulses = (elapsed as f64 * self.clockfreq / 1e9) as u8;
                    self.value = self.value.wrapping_sub(pulses);
                    if self.value == 0 {
                        self.value = self.data;
                    }
                    if pulses > 0 {
                        self.now = Instant::now();
                    }
                }
                ControlMode::Stop(_) => {}
            }
            OpResult::Byte(self.value)
        } else {
            OpResult::Byte(self.ctrl.as_u8())
        }
    }
    fn write(&mut self, address: usize, result: OpResult) { 
        if address != self.ctrl_address {
            self.data = result.inner() as u8;
            self.counter = 0;
            self.value = self.data;
        } else {
            self.ctrl = ControlMode::from(result.inner() as u8, self.offset);
        }
    }
}

pub struct Monitor {
    window: Window,
    buffer: Vec<u32>,
}

impl Monitor {
    pub fn new() -> Box<Monitor> {
        let window = Window::new(
            "Test - ESC to exit",
            640,
            400,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        let buffer: Vec<u32> = vec![0; 640 * 400];
        Box::new(Monitor { window, buffer })
    }
}

impl Device for Monitor {
    fn update(&mut self, _cpu: &CPU) -> Signal {
        Signal::Ok
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, address: usize, result: OpResult) {
        for j in 0..8 {
            if result.inner() & (1 << (7 - j % 8)) > 0 {
                self.buffer[8 * (address - 0x38000) + j] = 0xffffff;
            } else {
                self.buffer[8 * (address - 0x38000) + j] = 0x0;
            }
        }
        self.window.update_with_buffer(&self.buffer, 640, 400).expect("Error updating screen!");
    }
}


pub struct Floppy {
    content: Vec<u8>,
}

impl Floppy {
    pub fn new(image: &str) -> Box<Self> {
        let content = fs::read(image).expect("Disk image does not exist!");
        Box::new(Self { content })
    }
}

impl Device for Floppy {
    // fn init(&mut self, ram: RamPtr) {
    //     let mut raw_mem = ram.as_ref().borrow_mut();
    //     println!("{:08x}", 0xfa0000 + self.content.len());
    //     for (j, &b) in self.content.iter().enumerate() {
    //         raw_mem[j + 0xfa0000 as usize] = b;
    //     }
    // }
    fn update(&mut self, _cpu: &CPU) -> Signal { 
        Signal::Ok
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) { }
}

enum ControlMode {
    Stop(u8),
    Delay(u8, u8),
    EventCount(u8),
    PulseExtension(u8, u8),
}

impl ControlMode {
    fn from(ctrl: u8, offset: usize) -> Self {
        match ctrl >> offset {
            0 => Self::Stop(ctrl),
            1 => Self::Delay(4, ctrl),
            2 => Self::Delay(10, ctrl),
            3 => Self::Delay(16, ctrl),
            4 => Self::Delay(50, ctrl),
            5 => Self::Delay(64, ctrl),
            6 => Self::Delay(100, ctrl),
            7 => Self::Delay(200, ctrl),
            8 => Self::EventCount(ctrl),
            9 => Self::PulseExtension(4, ctrl),
            10 => Self::PulseExtension(10, ctrl),
            11 => Self::PulseExtension(16, ctrl),
            12 => Self::PulseExtension(50, ctrl),
            13 => Self::PulseExtension(64, ctrl),
            14 => Self::PulseExtension(100, ctrl),
            15 => Self::PulseExtension(200, ctrl),
            _ => Self::Stop(ctrl),
        }
    }
    fn as_u8(&self) -> u8 {
        match *self {
            Self::Stop(ctrl) | Self::Delay(_, ctrl) | Self::EventCount(ctrl) | Self::PulseExtension(_, ctrl) => ctrl,
        }
    }
}
