use em68k::{Emulator, devices::TestDevice, Configuration, memory::Bus};


fn test_configuration() -> Configuration {
    let mut bus = Bus::new();
    bus.attach(TestDevice::new(0x3f8000));
    
    Configuration {
        base_address: 0x0,
        start_address: 0x0,
        initial_ssp: 0x0,
        bus: bus,
        memory_layout: Vec::new(),
    }
}

#[test]
fn test_m68k_opcodes() {
    let mut em = Emulator::new(test_configuration());
    em.run("opcode_tests.bin", false);
}