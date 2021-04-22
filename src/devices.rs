use crate::memory::MemoryRange;
use crate::fields::{OpResult, Size};
use crate::processor::{set_bit, IRQ};
use std::mem::discriminant;
use minifb::{Window, WindowOptions};
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, atomic::{AtomicU8, Ordering, AtomicBool}, RwLock};

const CLKFREQ: f64 = 2457600.0;

pub type DeviceList = Vec<(MemoryRange, Box<dyn Device>)>;

pub enum Signal {
    Ok,
    Quit,
    NoOp,
    Remap,
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
    fn read(&mut self, address: usize, size: Size) -> OpResult;
    fn write(&mut self, address: usize, result: OpResult) -> Signal;
    fn interrupt_request(&mut self) -> Option<IRQ>;
    fn poll(&self) -> Signal;
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
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        size.from_be_bytes(&self.mem[address..])
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        for (j, &b) in result.to_be_bytes().iter().enumerate() {
            self.mem[address + j] = b;
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct Timer {
    data: Arc<AtomicU8>,
    value: Arc<AtomicU8>,
    interrupt: Arc<AtomicBool>,
    ctrl: ControlMode,
    ctrl_address: usize,
    data_address: usize,
    offset: usize,
    send_handle: mpsc::Sender<ControlMode>,
}

impl Timer {
    pub fn new(ctrl_addr: usize, offset: usize, data_addr: usize, clockfreq: f64) -> Box<Self> {
        let (tx, rx) = mpsc::channel();
        let data = Arc::new(AtomicU8::new(0));
        let value = Arc::new(AtomicU8::new(0));
        let interrupt = Arc::new(AtomicBool::new(false));
        let data_handle = Arc::clone(&data);
        let value_handle = Arc::clone(&value);
        let interrupt_handle = Arc::clone(&interrupt);
        thread::spawn(move || {
            let mut interval = 1e9 / CLKFREQ;
            let mut stopped = true;
            loop {
                let result = if stopped {
                    rx.recv()
                } else {
                    match rx.try_recv() {
                        Ok(mode) => Ok(mode),
                        _ => Err(mpsc::RecvError),
                    }
                };
                match result {
                    Ok(mode) => {
                        match mode {
                            ControlMode::Delay(delay, _) | ControlMode::PulseExtension(delay, _) => {
                                stopped = false;
                                interval = delay * 1e9 / CLKFREQ;
                            }
                            ControlMode::EventCount(_) => {
                                stopped = false;
                                interval = 1e9 / clockfreq;
                            }
                            ControlMode::Stop(_) => {
                                stopped = true;
                            }
                        }
                    }
                    Err(_) => ()
                }
                if !stopped {
                    thread::sleep(Duration::from_nanos(interval as u64));
                    value_handle.fetch_sub(1, Ordering::Relaxed);
                    if value_handle.load(Ordering::Relaxed) == 0 {
                        value_handle.store(data_handle.load(Ordering::Relaxed), Ordering::Relaxed);
                        interrupt_handle.store(true, Ordering::Relaxed);
                    }
                }
            }
        });
        Box::new(Self {
            value: value, 
            data: data,  
            interrupt: interrupt,
            ctrl_address: ctrl_addr, 
            data_address: data_addr,
            ctrl: ControlMode::Stop(0), 
            offset: offset,
            send_handle: tx,
         })
    }
}

impl Device for Timer {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.ctrl_address, self.ctrl_address + 1), (self.data_address, self.data_address + 1)]
    }
    fn read(&mut self, address: usize, _size: Size) -> OpResult {
        if address != self.ctrl_address {
            OpResult::Byte(self.value.load(Ordering::Relaxed))
        } else {
            OpResult::Byte(self.ctrl.as_u8())
        }
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal { 
        if address != self.ctrl_address {
            self.data.store(result.inner() as u8, Ordering::Relaxed);
            self.value.store(result.inner() as u8, Ordering::Relaxed);
        } else {
            self.ctrl = ControlMode::from(result.inner() as u8, self.offset);
            self.send_handle.send(self.ctrl).expect("Could not acquire timer lock!");
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct Monitor {
    buffer: Arc<RwLock<Vec<u32>>>,
    vram_start: usize,
    ctrl_address: usize,
    ctrl_register: Vec<u8>,
    resolution: Resolution,
    signal: mpsc::Receiver<Signal>,
}

impl Monitor {
    pub fn new(vram_start: usize, ctrl_address: usize) -> Box<Monitor> {
        let resolution = Resolution::High;
        let buffer: Arc<RwLock<Vec<u32>>> = Arc::new(RwLock::new(vec![0; 640 * 400]));
        let read_handle = Arc::clone(&buffer);
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut window = Window::new(
                "MyAtari ;-)",
                resolution.dimensions().0,
                resolution.dimensions().1,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
            while window.is_open() {
                {
                    let buffer = &read_handle.read().unwrap();
                    window.update_with_buffer(&buffer, resolution.dimensions().0, resolution.dimensions().1).expect("Error updating screen!");
                }
                thread::sleep(Duration::from_micros(166000));
            }
            tx.send(Signal::Quit).unwrap();
        });
        Box::new(Monitor { buffer, vram_start, ctrl_address, ctrl_register: vec![0; 102], resolution: Resolution::High, signal: rx })
    }
}

impl Device for Monitor {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.vram_start, self.vram_start + 640 * 400 / 8), (self.ctrl_address, self.ctrl_address + 102)]
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        if address >= self.ctrl_address {
            let rel_addr = address - self.ctrl_address; 
            if rel_addr == 0x5f {
                return size.from(self.resolution as u8)
            }
            size.from_be_bytes(&self.ctrl_register[address - self.ctrl_address..])
        } else {
            let mut result = Vec::new();
            let buffer = self.buffer.read().unwrap();
            for j in 0..size as usize {
                let mut b: usize = 0;
                for i in 0..8 {
                    set_bit(&mut b, i, buffer[8 * (address - self.vram_start) + 8 * (size as usize - j - 1) + i] > 0)
                }
                result.push(b as u8)
            }
            size.from_be_bytes(&result[..])
        }
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        if address < self.ctrl_address {
            let mut buffer = self.buffer.write().unwrap();
            for j in 0..8 {
                if result.inner() & (1 << (7 - j % 8)) > 0 {
                    buffer[8 * (address - self.vram_start) + j] = 0xffffff;
                } else {
                    buffer[8 * (address - self.vram_start) + j] = 0x0;
                }
            }
        } else {
            let rel_addr = address - self.ctrl_address;
            for (j, &b) in result.to_be_bytes().iter().enumerate() {
                self.ctrl_register[rel_addr + j] = b;
            }
            if rel_addr <= 2 {
                self.vram_start = u32::from_be_bytes([0, self.ctrl_register[0], self.ctrl_register[2], 0]) as usize;
                return Signal::Remap
            }
            if rel_addr == 0x5f {
                self.resolution = Resolution::from(self.ctrl_register[rel_addr]);
                println!("Resolution changed to {:?}", self.resolution);
            }

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
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { 
        match self.signal.try_recv() {
            Ok(signal) => signal,
            _ => Signal::Ok,
        }    
    }
}

#[derive(Debug, Clone, Copy)]
enum Resolution {
    Low = 0,
    Medium = 1,
    High = 2,
}

impl Resolution {
    fn from(b: u8) -> Self {
        match b {
            0 => Self::Low,
            1 => Self::Medium,
            2 => Self::High,
            _ => panic!("Invalid screen resolution!")
        }
    }
    fn dimensions(&self) -> (usize, usize) {
        match self {
            Self::Low => (320, 200),
            Self::Medium => (640, 200),
            Self::High => (640, 400),
        }
    }
}

pub struct Floppy {
    address: usize,
    _content: Vec<u8>,
}

impl Floppy {
    pub fn new(address: usize, image: &str) -> Box<Self> {
        let _content = fs::read(image).expect("Disk image does not exist!");
        Box::new(Self { address, _content })
    }
}

impl Device for Floppy {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 14)]
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) -> Signal { 
        Signal::Ok 
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

#[derive(Copy, Clone)]
enum ControlMode {
    Stop(u8),
    Delay(f64, u8),
    EventCount(u8),
    PulseExtension(f64, u8),
}

impl ControlMode {
    fn from(ctrl: u8, offset: usize) -> Self {
        match ctrl >> offset {
            0 => Self::Stop(ctrl),
            1 => Self::Delay(4., ctrl),
            2 => Self::Delay(10., ctrl),
            3 => Self::Delay(16., ctrl),
            4 => Self::Delay(50., ctrl),
            5 => Self::Delay(64., ctrl),
            6 => Self::Delay(100., ctrl),
            7 => Self::Delay(200., ctrl),
            8 => Self::EventCount(ctrl),
            9 => Self::PulseExtension(4., ctrl),
            10 => Self::PulseExtension(10., ctrl),
            11 => Self::PulseExtension(16., ctrl),
            12 => Self::PulseExtension(50., ctrl),
            13 => Self::PulseExtension(64., ctrl),
            14 => Self::PulseExtension(100., ctrl),
            15 => Self::PulseExtension(200., ctrl),
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
        vec![(self.address, self.address + 8)]
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        self.data
    }
    fn write(&mut self, _address: usize, result: OpResult) -> Signal {
        self.data = result;
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
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
    fn write(&mut self, _address: usize, result: OpResult) -> Signal {
        self.raw_data = result;
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct MultiFunctionPeripheral {
    address: usize,
    active_edge: u32,
    timer_a: Box<Timer>,
    timer_b: Box<Timer>,
    timer_c: Box<Timer>,
    timer_d: Box<Timer>,
    interrupt_handler: Box<InterruptHandler>,
}

impl MultiFunctionPeripheral {
    pub fn new(address: usize) -> Box<Self> {
        let result = Self { 
                        address: address, 
                        active_edge: 0,
                        interrupt_handler: InterruptHandler::new(0x6),
                        timer_a: Timer::new(0x18, 0, 0x1e, 2457600.0),
                        timer_b: Timer::new(0x1a, 0, 0x20, 50.0),
                        timer_c: Timer::new(0x1c, 4, 0x22, 200.0),
                        timer_d: Timer::new(0x1c, 0, 0x24, 2457600.0),
                    };
        Box::new(result)
    }
}

impl Device for MultiFunctionPeripheral {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 64)]
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        let rel_addr = address - self.address;
        if rel_addr == 0 {
            return size.from(0xa1)
        }
        if rel_addr == 2 {
            return size.from(self.active_edge)
        }
        for (fromaddr, toaddr) in self.timer_a.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                return self.timer_a.read(rel_addr, size)
            }
        }
        for (fromaddr, toaddr) in self.timer_b.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                return self.timer_b.read(rel_addr, size)
            }
        }
        for (fromaddr, toaddr) in self.timer_c.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                return self.timer_c.read(rel_addr, size)
            }
        }
        for (fromaddr, toaddr) in self.timer_d.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                return self.timer_d.read(rel_addr, size)
            }
        }
        for (fromaddr, toaddr) in self.interrupt_handler.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                return self.interrupt_handler.read(rel_addr, size)
            }
        }
        panic!("Unmapped address {:x}!", rel_addr)
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        let rel_addr = address - self.address;
        if rel_addr == 2 {
            self.active_edge = result.inner();
        }
        for (fromaddr, toaddr) in self.timer_a.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                self.timer_a.write(rel_addr, result);
            }
        }
        for (fromaddr, toaddr) in self.timer_b.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                self.timer_b.write(rel_addr, result);
            }
        }
        for (fromaddr, toaddr) in self.timer_c.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                self.timer_c.write(rel_addr, result);
            }
        }
        for (fromaddr, toaddr) in self.timer_d.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                self.timer_d.write(rel_addr, result);
            }
        }
        for (fromaddr, toaddr) in self.interrupt_handler.memconfig() {
            if rel_addr >= fromaddr && rel_addr < toaddr {
                self.interrupt_handler.write(rel_addr, result);
            }
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { 
        if self.timer_a.interrupt.load(Ordering::Relaxed) {
            self.timer_a.interrupt.store(false, Ordering::Relaxed);
            return Some(IRQ { level: 6 })
        }
        if self.timer_b.interrupt.load(Ordering::Relaxed) {
            self.timer_b.interrupt.store(false, Ordering::Relaxed);
            return Some(IRQ { level: 6 })
        }
        if self.timer_c.interrupt.load(Ordering::Relaxed) {
            self.timer_c.interrupt.store(false, Ordering::Relaxed);
            return Some(IRQ { level: 6 })
        }
        if self.timer_d.interrupt.load(Ordering::Relaxed) {
            self.timer_d.interrupt.store(false, Ordering::Relaxed);
            return Some(IRQ { level: 6 })
        }
        None
    }
    fn poll(&self) -> Signal { Signal::Ok }
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

struct InterruptHandler {
    ctrl_register: usize
}

impl InterruptHandler {
    pub fn new(ctrl_register: usize) -> Box<Self> {
        Box::new(Self { ctrl_register })
    }
}

impl Device for InterruptHandler {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.ctrl_register, self.ctrl_register + 22)]
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, result: OpResult) -> Signal {
        println!("Interrupt handler received {}", result);
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct MIDIAdapter {
    address: usize
}

impl MIDIAdapter {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address })
    }
}

impl Device for MIDIAdapter {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 4)]
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        OpResult::Byte(0)
    }
    fn write(&mut self, _address: usize, _result: OpResult) -> Signal {
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct Microwire {
    address: usize,
    data: Vec<u8>,
}

impl Microwire {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address, data: vec![0; 4] })
    }
}

impl Device for Microwire {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 4)]
    }
    fn read(&mut self, _address: usize, size: Size) -> OpResult {
        size.zero()
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        for (j, &b) in result.to_be_bytes().iter().enumerate() {
            self.data[address - self.address + j] = b;
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct DMASoundSystem {
    address: usize,
    data: Vec<u8>,
}

impl DMASoundSystem {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address, data: vec![0; 0x1a] })
    }
}

impl Device for DMASoundSystem {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 0x1a)]
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        size.from_be_bytes(&self.data[address - self.address..])
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        for (j, &b) in result.to_be_bytes().iter().enumerate() {
            self.data[address - self.address + j] = b;
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct SystemControlUnit {
    address: usize,
    data: Vec<u8>,
}

impl SystemControlUnit {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address, data: vec![0; 0x20] })
    }
}

impl Device for SystemControlUnit {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 0x20)]
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        size.from_be_bytes(&self.data[address - self.address..])
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        for (j, &b) in result.to_be_bytes().iter().enumerate() {
            self.data[address - self.address + j] = b;
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct JoystickPort {
    address: usize,
    data: Vec<u8>,
}

impl JoystickPort {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address, data: vec![0; 0x600] })
    }
}

impl Device for JoystickPort {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 0x600)]
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        size.from_be_bytes(&self.data[address - self.address..])
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        for (j, &b) in result.to_be_bytes().iter().enumerate() {
            self.data[address - self.address + j] = b;
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}


pub struct Keyboard {
    address: usize
}

impl Keyboard {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address })
    }
}

impl Device for Keyboard {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 4)]
    }
    fn read(&mut self, _address: usize, _size: Size) -> OpResult {
        OpResult::Byte(2)
    }
    fn write(&mut self, _address: usize, _result: OpResult) -> Signal {
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}


pub struct Blitter {
    address: usize,
    raw_data: Vec<u8>,
}

impl Blitter {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address: address, raw_data: vec![0; 0x3d] })
    }
}

impl Device for Blitter {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 0x3d)]
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        size.from_be_bytes(&self.raw_data[address - self.address..])
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        for (j, &b) in result.to_be_bytes().iter().enumerate() {
            self.raw_data[address - self.address + j] = b;
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

pub struct RealTimeClock {
    address: usize,
    raw_data: Vec<u8>,
}

impl RealTimeClock {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address: address, raw_data: vec![0; 0x20] })
    }
}

impl Device for RealTimeClock {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 0x20)]
    }
    fn read(&mut self, address: usize, size: Size) -> OpResult {
        size.from_be_bytes(&self.raw_data[address - self.address..])
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        for (j, &b) in result.to_be_bytes().iter().enumerate() {
            self.raw_data[address - self.address + j] = b;
        }
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}


pub struct CartridgeROM {
    address: usize,
}

impl CartridgeROM {
    pub fn new(address: usize) -> Box<Self> {
        Box::new(Self { address })
    }
}

impl Device for CartridgeROM {
    fn memconfig(&self) -> MemoryRange {
        vec![(self.address, self.address + 0x10000)]
    }
    fn read(&mut self, _address: usize, size: Size) -> OpResult {
        size.from(0xffffffff as u32)
    }
    fn write(&mut self, _address: usize, _result: OpResult) -> Signal {
        panic!("Memory not writable!")
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

