use crate::memory::{Bus, MemoryRange};
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


pub trait Device: {
    fn memconfig(&self) -> MemoryRange;
    fn update(&mut self, cpu: &CPU) -> Signal;
    fn read(&mut self, address: usize, size: Size) -> OpResult;
    fn write(&mut self, address: usize, result: OpResult);
}

pub struct Ram {
    size: usize,
    mem: Vec<u8>
}

impl Ram {
    pub fn new(size: usize) -> Box<Self> {
        Box::new(Self { mem: vec![0; size], size: size })
    }
}

impl Device for Ram {
    fn memconfig(&self) -> MemoryRange {
        vec![(0x0, self.size)]
    }
    fn update(&mut self, _cpu: &CPU) -> Signal {
        Signal::Ok
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        match size {
            Size::Byte => OpResult::Byte(self.mem[address]),
            Size::Word => OpResult::Word(u16::from_be_bytes([self.mem[address], self.mem[address + 1]])),
            Size::Long => {
                OpResult::Long(u32::from_be_bytes([self.mem[address], self.mem[address + 1], self.mem[address + 2], self.mem[address + 3]]))
            }
        }
    }
    fn write(&mut self, address: usize, result: OpResult) {
        match result {
            OpResult::Byte(b) => self.mem[address] = b,
            OpResult::Word(w) => {
                self.mem[address + 1] = (w & 0xff) as u8;
                self.mem[address] = ((w & 0xff00) >> 8) as u8;
            }
            OpResult::Long(l) => {
                self.mem[address + 3] = (l & 0xff) as u8;
                self.mem[address + 2] = ((l & 0xff00) >> 8) as u8;
                self.mem[address + 1] = ((l & 0xff0000) >> 16) as u8;
                self.mem[address] = ((l & 0xff000000) >> 24) as u8;
            }
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
    data_address: usize,
    offset: usize,
    clockfreq: f64,
}

impl Timer {
    pub fn new(ctrl: usize, offset: usize, data: usize, clockfreq: f64) -> Box<Self> {
        Box::new(Self { now: Instant::now(), 
                        value: 0, 
                        data: 0, 
                        counter: 0, 
                        ctrl_address: ctrl, 
                        data_address: data,
                        ctrl: ControlMode::Stop(0), 
                        offset: offset,
                        clockfreq: clockfreq })
    }
}

impl Device for Timer {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.ctrl_address, self.ctrl_address + 1), (self.data_address, self.data_address + 1)]
    }
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
    vram_start: usize,
    ctrl_address: usize,
    ctrl_register: Vec<u8>,
}

// $FFFF8201  r/w  |xxxxxxxx|          Video base high
// $FFFF8203  r/w  |xxxxxxxx|          Video base medium
// $FFFF8205  r    |..xxxxxx|          Video address counter high (r/w on STe)
// $FFFF8207  r    |xxxxxxxx|          Video address counter med (r/w on STe)
// $FFFF8209  r    |xxxxxxx.|          Video address counter low (r/w on STe)
// $FFFF820A  r/w  |......xx|          Sync mode
//                        ||__________ External/Internal sync
//                        |___________ 50/60Hz

// $FFFF820D  r/w  |xxxxxxx.|          STe video base low
// $FFFF820F  r/w  |xxxxxxxx|          STe over-length line width
// $FFFF8240  r/w  |....xxxxxxxxxxxx|  Palette colour (1 word each, first of 16)
//                      ||  ||  ||____ Blue intensity (0-7)
//                      ||  ||  |_____ STe blue LSB
//                      ||  ||________ Green intensity (0-7)
//                      ||  |_________ STe green LSB
//                      ||____________ Red intensity (0-7)
//                      |_____________ STe red LSB

// $FFFF8260  r/w  |......xx|          Screen resolution
//                        |___________ 0 - 320x200x4
//                                     1 - 640x200x2
//                                     2 - 640x400x1

// $FFFF8264  r/w  |....xxxx|          Undocumented STE pixel hard scroll
// $FFFF8265  r/w  |....xxxx|          STE pixel hard scroll


impl Monitor {
    pub fn new(vram_start: usize, ctrl_address: usize) -> Box<Monitor> {
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
        Box::new(Monitor { window, buffer, vram_start, ctrl_address, ctrl_register: vec![0; 102] })
    }
}

impl Device for Monitor {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.vram_start, self.vram_start + 640 * 400 / 8), (self.ctrl_address, self.ctrl_address + 102)]
    }
    fn update(&mut self, _cpu: &CPU) -> Signal {
        Signal::Ok
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        if address >= self.ctrl_address {
            OpResult::Byte(0)
        } else {
            OpResult::Byte(0)
        }
    }
    fn write(&mut self, address: usize, result: OpResult) {
        if address < self.ctrl_address {
            for j in 0..8 {
                if result.inner() & (1 << (7 - j % 8)) > 0 {
                    self.buffer[8 * (address - self.vram_start) + j] = 0xffffff;
                } else {
                    self.buffer[8 * (address - self.vram_start) + j] = 0x0;
                }
            }
            self.window.update_with_buffer(&self.buffer, 640, 400).expect("Error updating screen!");
        } else {
            // FIXME
        }
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
    fn memconfig(&self) -> MemoryRange {
        vec![(0, 0)]
    }
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

pub struct MMU {
    address: usize,
    data: OpResult,
}

impl MMU {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address: address, data: OpResult::Byte(0) })
    }
}

impl Device for MMU {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 1)]
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        self.data
    }
    fn write(&mut self, _address: usize, result: OpResult) {
        self.data = result;
    }
    fn update(&mut self, _cpu: &CPU) -> Signal {
        Signal::Ok
    }
}

pub struct SoundGenerator {
    address: usize,
    raw_data: OpResult, // FIXME
}

impl SoundGenerator {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address: address, raw_data: OpResult::Long(0) })
    }
}

impl Device for SoundGenerator {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 4)]
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        self.raw_data
    }
    fn write(&mut self, _address: usize, result: OpResult) {
        self.raw_data = result;
    }
    fn update(&mut self, _cpu: &CPU) -> Signal {
        Signal::Ok
    }
}

pub struct MultiFunctionPeripheral {
    address: usize,
    bus: Bus,
}

impl MultiFunctionPeripheral {
    pub fn new(address: usize) -> Box<Self> {
        let mut bus = Bus::new();
        bus.attach(Timer::new(0x18, 0, 0x1e, 2457600.0));
        bus.attach(Timer::new(0x1a, 0, 0x20, 50.0));
        bus.attach(Timer::new(0x1c, 4, 0x22, 200.0));
        bus.attach(Timer::new(0x1c, 0, 0x24, 2457600.0));
        Box::new(Self { address, bus })
    }
}

impl Device for MultiFunctionPeripheral {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 64)]
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        self.bus.read(address - self.address, size)
    }
    fn write(&mut self, address: usize, result: OpResult) {
        self.bus.write(address - self.address, result);
    }
    fn update(&mut self, _cpu: &CPU) -> Signal {
        Signal::Ok
    }
}

// $FFFFFA01  r/w  |x.xx...x|          MFP GP I/O
//                  | ||   |__________ Parallel port status
//                  | ||______________ WD1772 active
//                  | |_______________ Interrupt
//                  |_________________ Mono monitor

// $FFFFFA03  r/w  |xxxxxxxx|          Active edge
//                  ||||||||__________ Centronics busy
//                  |||||||___________ RS-232 Data Carrier Detect (DCD)
//                  ||||||____________ RS-232 Clear To Send (CTS)
//                  |||||_____________ Reserved
//                  ||||______________ ACIA Interrupt
//                  |||_______________ FDC/HDC interrupt
//                  ||________________ RS-232 ring indicator
//                  |_________________ Mono monitor detect

// $FFFFFA05  r/w  |xxxxxxxx|          Data direction (all bits IN/OUT)

// $FFFFFA07  r/w  |xxxxxxxx|          Interrupt enable A
// $FFFFFA0B  r/w  |xxxxxxxx|          Interrupt pending A
// $FFFFFA0F  r/w  |xxxxxxxx|          Interrupt in-service A
// $FFFFFA13  r/w  |xxxxxxxx|          Interrupt mask A
//                  ||||||||__________ MFP timer B (ENABLE/DISABLE)
//                  |||||||___________ RS-232 transmit error
//                  ||||||____________ RS-232 transmit buffer empty
//                  |||||_____________ RS-232 receive error
//                  ||||______________ RS-232 receive buffer full
//                  |||_______________ MFP timer A
//                  ||________________ RS-232 ring indicator
//                  |_________________ Monochrome detect
 
// $FFFFFA09  r/w  |xxxxxxxx|          Interrupt enable B
// $FFFFFA0D  r/w  |xxxxxxxx|          Interrupt pending B
// $FFFFFA11  r/w  |xxxxxxxx|          Interrupt in-service B
// $FFFFFA15  r/w  |xxxxxxxx|          Interrupt mask B
//                  ||||||||__________ Centronics busy (ENABLE/DISABLE)
//                  |||||||___________ RS-232 Data Carrier Detect (DCD)
//                  ||||||____________ RS-232 Clear To Send (CTS)
//                  |||||_____________ Blitter done
//                  ||||______________ MFP Timer D (USART)
//                  |||_______________ MFP timer C (200Hz clock)
//                  ||________________ ACIA interrupt
//                  |_________________ FDC/HDC controller

// $FFFFFA17  r/w  |....x...|          Vector base
//                      |_____________ Manual/Auto end of interrupts

// $FFFFFA19  r/w  |....xxxx|          Timer A control
// $FFFFFA1B  r/w  |....xxxx|          Timer B control
//                      |_____________ Timer delay mode (see table)

//                                     Value  Delay (divider)
//                                     %0000  Timer stop
//                                     %0001  4
//                                     %0010  10
//                                     %0011  16
//                                     %0100  50
//                                     %0101  64
//                                     %0110  100
//                                     %0111  200
//                                     %1000  Event count mode
//                                     %1xxx  Pulse extension mode (delay as above)

// $FFFFFA1D  r/w  |.xxx.xxx|          Timers C&D control
//                   |   |____________ Timer D delay mode (see table)
//                   |________________ Timer C delay mode (see table)

//                                     Value  Delay (divider)
//                                     %000   Timer stop
//                                     %001   4
//                                     %010   10
//                                     %011   16
//                                     %100   50
//                                     %101   64
//                                     %110   100
//                                     %111   200

// $FFFFFA1F  r/w  |xxxxxxxx|          Timer A data
// $FFFFFA21  r/w  |xxxxxxxx|          Timer B data
// $FFFFFA23  r/w  |xxxxxxxx|          Timer C data
// $FFFFFA25  r/w  |xxxxxxxx|          Timer D data
// $FFFFFA27  r/w  |xxxxxxxx|          Sync character

// $FFFFFA29  r/w  |xxxxxxx.|          USART control register
//                  | | |||___________ Parity odd/even
//                  | | ||____________ Parity enable/disable
//                  | | |_____________ Protocol A (see table)
//                  | |_______________ Protocol B (see table)
//                  |_________________ Clock divide by 16 off/on

//                 Protocol A                        Protocol B
//                 Value  Stop  Start  Format        Value  Data
//                 %00    0     0      Synchronous	  %00    8
//                 %01    1     1      Asynchronous  %01    7
//                 %10    1     1.5    Asynchronous  %10    6
//                 %11    1     2      Asynchronous  %11    5

// $FFFFFA2B  r/w  |xxxxxxxx|          Receiver status
//                  ||||||||__________ Receiver enable bit
//                  |||||||___________ Synchronous strip enable
//                  ||||||____________ Match/Character in progress
//                  |||||_____________ Found, Search/Break detected
//                  ||||______________ Frame error
//                  |||_______________ Parity error
//                  ||________________ Frame error
//                  |_________________ Overrun error

// $FFFFFA2D  r/w  |xxxxxxxx|          Transmitter status
//                  ||||||||__________ Transmitter enable bit
//                  |||||||___________ Low bit
//                  ||||||____________ High bit
//                  |||||_____________ Break
//                  ||||______________ End of transmission
//                  |||_______________ Auto turnaround
//                  ||________________ Underrun error
//                  |_________________ Buffer empty

// $FFFFFA2F  r/w  |xxxxxxxx|          USART data
