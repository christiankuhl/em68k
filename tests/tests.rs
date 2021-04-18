use em68k::{Emulator, Configuration};
use em68k::devices::{Device, Signal, Ram};
use em68k::memory::{Bus, MemoryRange};
use em68k::fields::{OpResult, Size};
use em68k::processor::IRQ;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::env;
use termion::{clear, color, cursor};

const BASE_ADDRESS: usize = 0xffffff00;
const TESTS: [&str; 61] = ["ORI_TO_CCR", "ORI_TO_SR", "EORI_TO_CCR", "EORI_TO_SR", "ANDI_TO_CCR", "ANDI_TO_SR", "BTST",
                            "BCHG", "BCLR", "BSET", "MOVEP", "BOOL_I", "BSR", "CMP_I", "ADD_I", "SUB_I", "MOVE", "MOVE_FLAGS",
                            "EXT", "SWAP", "LEA/PEA", "TAS", "TST", "LINK", "MOVE_USP", "CHK", "NEGS", "CLR", "MOVEM", "TRAPV", 
                            "RTR", "BCC", "DBCC", "SCC", "ADDQ", "SUBQ", "MOVEQ", "DIVU", "DIVS", "OR", "AND", "EOR", "CMP", 
                            "CMPA", "CMPM", "ADD", "SUB", "ADDA", "SUBA", "ADDX", "SUBX", "MULU", "MULS", "EXG", "RO<L/R>", 
                            "ROX<L/R>", "AS<L/R>", "LS<L/R>", "ABCD", "SBCD", "NBCD", ];

struct TestDevice {
    tests: HashMap<usize, (String, u32)>,
}

impl TestDevice {
    fn new() -> Box<TestDevice> {
        let mut tests = HashMap::new();
        for (j, t) in TESTS.iter().enumerate() {
            tests.insert(j, (t.to_string(), 0));
        }
        Box::new( Self { tests } )
    }
}

impl Device for TestDevice {
    fn memconfig(&self) -> MemoryRange {
        vec![(BASE_ADDRESS, BASE_ADDRESS + TESTS.len())]
    }
    fn read(&mut self, _address: usize, size: Size) -> OpResult {
        size.zero()
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        if let Some((_, test_result)) = self.tests.get_mut(&(address - BASE_ADDRESS)) {
            *test_result = result.inner();
        }
        println!("{}", self);
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
    fn poll(&self) -> Signal { Signal::Ok }
}

impl fmt::Display for TestDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("Running opcode tests ");
        let mut failed_tests = Vec::new();
        let mut current = 0;
        for j in 0..TESTS.len() {
            if let Some(status) = self.tests.get(&j) {
                match status {
                    (_, 0) => result.push_str(&format!("{n}.", n = color::Fg(color::Reset))),
                    (_, 1) => {
                        current = j;
                        result.push_str(&format!("{g}.", g = color::Fg(color::Green)))
                    }
                    (testname, 2) => { 
                        current = j;
                        result.push_str(&format!("{r}*", r = color::Fg(color::Red)));
                        failed_tests.push((j, testname, "failed"));
                    }
                    _ => ()
                }
            }
        }
        if current < TESTS.len() - 1 {
            result.push_str(&format!("{c}{n}Currently running: {t}\n", 
                t = TESTS[current + 1], 
                c = cursor::Goto(1, 3),
                n = color::Fg(color::Reset)
            ))
        } else {
            result.push_str(&format!("{c}{n}Opcode tests complete: {p}/{t} passed{s}\n", 
                p = TESTS.len() - failed_tests.len(), 
                t = TESTS.len(), 
                c = cursor::Goto(1, 3),
                n = color::Fg(color::Reset),
                s = if failed_tests.len() == 0 {" ;-)"} else {""},
            ))
        }
        for (j, testname, status) in failed_tests.drain(..) {
            result.push_str(&format!("\n{n}Test 0x{j:02x}: {t} {s}", 
                n = color::Fg(color::Reset),
                t = testname, 
                j = j, 
                s = status
            ));
        }
        write!(f, "{c}{tl}{res}\n", res=result, c=clear::All, tl=cursor::Goto(1, 1))
    }
}

fn test_configuration() -> Configuration {
    let mut bus = Bus::new();
    bus.attach(Ram::new(0xfff8000));
    bus.attach(TestDevice::new());
    
    Configuration {
        base_address: 0x0,
        start_address: 0x400,
        initial_ssp: 0x3f0,
        bus: bus,
        memory_layout: Vec::new(),
    }
}

fn main() {
    let args: HashSet<String> = env::args().collect();
    let mut em = Emulator::new(test_configuration());
    em.run("tests/opcode_tests.bin", args.contains(&String::from("--debug")));
}

// fail easy68k:
// 0x11
// 0x34

