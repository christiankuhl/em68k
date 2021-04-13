use em68k::{Emulator, atari::st1040};

fn main() {
    let mut em = Emulator::new(st1040());
    em.run("tos/TOS104GE.IMG", true);
}
