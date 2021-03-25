use crate::processor::CPU;
use crate::memory::{RamPtr, RAM_SIZE};
use std::cell::RefCell;
use std::rc::Rc;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};
// use tui::layout::{Layout, Constraint, Direction};
// use termion::event::{Key, Event};
// use termion::input::TermRead;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};


pub type DeviceList = Vec<Box<dyn Device>>;

pub trait Device {
    fn init(&mut self, ram: RamPtr);
    fn update(&mut self, cpu: &mut CPU);
}

pub struct Debugger {
    ram: RamPtr,
    // stdout: termion::raw::RawTerminal<std::io::Stdout>,
    // backend: tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>,
    // terminal: tui::Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
}

impl Debugger {
    pub fn new() -> Box<Self> {
        // let stdout = stdout().into_raw_mode().unwrap();
        // let backend = TermionBackend::new(stdout);
        // let mut terminal = Terminal::new(backend).unwrap();
        // terminal.clear().expect("Terminal clear failed.");
        Box::new(Debugger { 
            ram: Rc::new(RefCell::new([0; RAM_SIZE])),
            // terminal: terminal,
        })
    }
}

impl Device for Debugger {
    fn init(&mut self, ram: RamPtr) {
        self.ram = ram;
    }
    fn update(&mut self, cpu: &mut CPU) {
        // self.terminal.draw(|f| {
        //     let size = f.size();
        //     let block = Block::default()
        //         .title("Block")
        //         .borders(Borders::ALL);
        //     f.render_widget(block, size);
        // }).expect("UI draw failed!");

        write!(stdout(), "{:?}\n\n", cpu);
        write!(stdout(), "{:}\n\n", cpu.nxt.as_asm(cpu));

        for line in cpu.disassemble() {
            let mut out = format!("{:08x} ", line.0);
            for word in line.1 {
                out.push_str(&format!("{:04x} ", word));
            }
            write!(stdout(), "{:<30}{}\n", out, line.2);
        }
        stdout().flush().unwrap();
        pause();
    }
}

fn pause() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char(' ')) => break,
            Event::Key(Key::Char('q')) => panic!(),
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                    },
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::clear::All);
}
