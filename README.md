# em68k
This is a naïve attempt to build an emulator for the [Motorola 68000](https://en.wikipedia.org/wiki/Motorola_68000) processor in Rust.

The long term goal is to emulate the [Atari ST](https://en.wikipedia.org/wiki/Atari_ST) I learned to program on as a kid and maybe revive some of the old games I used to play.

In its current state, the emulator supports the processor's instruction set with the exception of the instructions for cache control and the floating point coprocessor, which I consider out of scope.
