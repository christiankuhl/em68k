use em68k::{Emulator, atari::st1040};
use std::collections::HashSet;
use std::env;

fn main() {
    let args: HashSet<String> = env::args().collect();
    println!("{:?}", args);
    let mut em = Emulator::new(st1040());
    em.run("tos/TOS104GE.IMG", args.contains(&String::from("--debug")));
}
