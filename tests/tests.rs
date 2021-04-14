use em68k::{Emulator, Configuration};
use em68k::devices::{Device, Signal, Ram};
use em68k::memory::{Bus, MemoryRange};
use em68k::fields::{OpResult, Size};
use em68k::processor::IRQ;
use std::collections::HashMap;
use std::fmt;

const BASE_ADDRESS: usize = 0xffffff00;
const TESTS: [&str; 60] = ["ORI_TO_CCR", "ORI_TO_SR", "EORI_TO_CCR", "EORI_TO_SR", "ANDI_TO_CCR", "ANDI_TO_SR", "BTST",
                            "BCHG", "BCLR", "BSET", "MOVEP", "BOOL_I", "BSR", "CMP_I", "ADD_I", "SUB_I", "MOVE", "MOVE_FLAGS",
                            "EXT", "SWAP", "LEA/PEA", "TAS", "TST", "LINK", "MOVE_USP", "CHK", "CLR", "MOVEM", "ABCD",
                            "SBCD", "NBCD", "TRAPV", "RTR", "BCC", "DBCC", "SCC", "ADDQ", "SUBQ", "MOVEQ", "DIVU", "DIVS",      
                            "OR", "AND", "EOR", "CMP", "CMPA", "CMPM", "ADD", "SUB", "ADDA", "SUBA", "ADDX", "SUBX", "MULU",
                            "MULS", "EXG", "RO<L/R>", "ROX<L/R>", "AS<L/R>", "LS<L/R>",];

struct TestDevice {
    tests: HashMap<usize, (String, u32)>,
}

impl TestDevice {
    fn new() -> Box<TestDevice> {
        let mut tests = HashMap::new();
        for (j, t) in TESTS.iter().enumerate() {
            tests.insert(j + BASE_ADDRESS, (t.to_string(), 0));
        }
        Box::new( Self { tests } )
    }
}

impl Device for TestDevice {
    fn memconfig(&self) -> MemoryRange {
        vec![(BASE_ADDRESS, BASE_ADDRESS + 60)]
    }
    fn read(&mut self, _address: usize, size: Size) -> OpResult {
        size.zero()
    }
    fn write(&mut self, address: usize, result: OpResult) -> Signal {
        if let Some((_, test_result)) = self.tests.get_mut(&address) {
            *test_result = result.inner();
        }
        println!("{}", self);
        Signal::Ok
    }
    fn interrupt_request(&mut self) -> Option<IRQ> { None }
}

impl fmt::Display for TestDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str("Foo");
        write!(f, "{}", result)
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

#[test]
fn test_m68k_opcodes() {
    let mut em = Emulator::new(test_configuration());
    em.run("tests/opcode_tests.bin", false);
}