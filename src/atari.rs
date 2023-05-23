use crate::fields::{OpResult, OpResult::*};
use crate::memory::Bus;
use crate::devices::*;
use crate::Configuration;

const RAM_SIZE: u32 = 0x400000;
const BASE_ADDRESS: u32 = 0xfc0000;
const START_ADDRESS: u32 = 0xfc0030;
const INITIAL_SSP: u32 = 0x0104; 

// Initial Memory Layout Atari ST
const MEMORY_LAYOUT: [(usize, OpResult); 14] = [
    //   $000.L      Reset initial SSP value
    (0x0, Long(0x0104)),
    //   $004.L      Reset initial PC address
    (0x4, Long(START_ADDRESS)),
    //   $028.L      Line 1010 (Line A routine)
    // Line A emulator $EB9A
    (0x28, Long(0xeb9a)),
    //   $068.L      Interrupt level 2 (Hblank sync)
    // Level 2 interrupt $543C
    (0x68, Long(0x543c)),
    //   $070.L      Interrupt level 4 (Vblank sync)
    // Level 4 interrupt $5452
    (0x80, Long(0x5452)),
    //   $420.L      Memvalid (Cold start OK if #$752019F3)
    (0x420, Long(0x752019f3)),
    //   $424.B      Memcntlr (Memory controller low nibble)
    (0x424, Byte(0x0)),
    //   $426.L      Resvalid (#$31415926 to jump through 'resvector')
    (0x426, Long(0x0)),
    //   $42A.L      Resvector (System reset bailout vector)
    (0x42a, Long(START_ADDRESS)),
    //   $42E.L      Phystop (Physical RAM top)
    (0x42e, Long(RAM_SIZE)),
    //   $43A.L      Memval2 (#$237698AA)
    (0x43a, Long(0x237698aa)),
    //   $51A.L      Memval3 (#$5555AAAA)
    (0x51a, Long(0x5555aaaa)),
    //   $4A6.W      _Nflops (Number of floppies attached)
    (0x4a6, Word(0x1)),
    //   $44E.L      _V_bas_ad (Screen memory base pointer)
    (0x44e, Long(0x78000)),
    ];

    // Set trap vectors:
    // TRAP #1 GEMDOS $965E
    // TRAP #2 GEM $2A338
    // TRAP #13 BIOS $556C
    // TRAP #14 XBIOS $5566
    
    
//   $000.L      Reset initial SSP value
//   $004.L      Reset initial PC address
//   $008.L      Bus error
//   $00C.L      Address error
//   $010.L      Illegal instruction
//   $014.L      Division by zero
//   $018.L      Chk instruction
//   $01C.L      Trapv instruction
//   $020.L      Privilege violation
//   $024.L      Trace mode
//   $02C.L      Line 1111 (Used by AES)
//   $030.L      Unassigned
//   $034.L      Coprocessor protocol violation (for MC68020)
//   $038.L      Format error (for MC68020)
//   $03C.L      Uninitialised interrupt vector
//   $040.L      Unassigned
//   $044.L      Unassigned
//   $048.L      Unassigned
//   $04C.L      Unassigned
//   $050.L      Unassigned
//   $054.L      Unassigned
//   $058.L      Unassigned
//   $05C.L      Unassigned
//   $060.L      Spurious interrupt (Hacked to level 3)
//   $064.L      Interrupt level 1 (Used when user wants Hblanks)
//   $06C.L      Interrupt level 3 (Normal processor interrupt
//                                  level)
//   $074.L      Interrupt level 5
//   $078.L      Interrupt level 6 (MK68901 MFP Interrupts)
//   $07C.L      Interrupt level 7 (NMI)
//   $080.L      Trap #0
//   $084.L      Trap #1 (GEMDOS interface calls)
//   $088.L      Trap #2 (Extended DOS calls)
//   $08C.L      Trap #3
//   $090.L      Trap #4
//   $094.L      Trap #5
//   $098.L      Trap #6
//   $09C.L      Trap #7
//   $0A0.L      Trap #8
//   $0A4.L      Trap #9
//   $0A8.L      Trap #10
//   $0AC.L      Trap #11
//   $0B0.L      Trap #12
//   $0B4.L      Trap #13 (BIOS interface calls)
//   $0B8.L      Trap #14 (XBIOS interface calls)
//   $0BC.L      Trap #15
//   $0C0-$0FF   Unassigned

// MFP hardware bound interrupt vectors

//   $100.L      Parallel port interrupt_0 (Centronics Busy)
//   $104.L      RS232 carrier detect (dcd) interrupt_1
//   $108.L      RS232 clear to send (cts) interrupt_2
//   $10C.L      Graphics blt done interrupt_3
//   $110.L      RS232 baud rate generator (Timer D)
//   $114.L      200 Hz system clock (Timer C)
//   $118.L      Keyboard/MIDI (6850) interrupt_4
//   $11C.L      Polled fdc/_hdc interrupt_5
//   $120.L      Horizontal blank counter (Timer C)
//   $124.L      RS232 transmit error interrupt
//   $128.L      RS232 transmit buffer empty interrupt
//   $12C.L      RS232 receive error interrupt
//   $130.L      RS232 receive buffer full interrupt
//   $134.L      User/application (Timer A)
//   $138.L      RS232 ring indicator interrupt_6
//   $13C.L      Polled monochrome detect interrupt_7
//   $140-$1FF   Unassigned

// Application interrupts

//   $200-$37F   Reserved for Other Equipment Manufacturers (OEMs)

// Processor state (Post mortem dump area)

//   $380.L      Proc_lives
//   $384.L      Proc_regs (saved D0)
//   $388.L      Proc_regs (saved D1)
//   $38C.L      Proc_regs (saved D2)
//   $390.L      Proc_regs (saved D3)
//   $394.L      Proc_regs (saved D4)
//   $398.L      Proc_regs (saved D5)
//   $39C.L      Proc_regs (saved D6)
//   $3A0.L      Proc_regs (saved D7)
//   $3A4.L      Proc_regs (saved A0)
//   $3A8.L      Proc_regs (saved A1)
//   $3AC.L      Proc_regs (saved A2)
//   $3B0.L      Proc_regs (saved A3)
//   $3B4.L      Proc_regs (saved A4)
//   $3B8.L      Proc_regs (saved A5)
//   $3BC.L      Proc_regs (saved A6)
//   $3C0.L      Proc_regs (saved A7_ssp)
//   $3C4.L      Proc_pc
//   $3C8.L      Proc_usp
//   $3CC.W      Proc_stk (total of 16 words)
//   $3CE.W      Proc_stk
//   $3D0.W      Proc_stk
//   $3D2.W      Proc_stk
//   $3D4.W      Proc_stk
//   $3D6.W      Proc_stk
//   $3D8.W      Proc_stk
//   $3DA.W      Proc_stk
//   $3DC.W      Proc_stk
//   $3DE.W      Proc_stk
//   $3E0.W      Proc_stk
//   $3E2.W      Proc_stk
//   $3E4.W      Proc_stk
//   $3E6.W      Proc_stk
//   $3E8.W      Proc_stk
//   $3EA.W      Proc_stk
//   $3EC-$3FF   Unassigned

// System Variables

//   $400.L      Etv_timer (Timer handoff)
//   $404.L      Etv_critic (Critical error handoff vector)
//   $408.L      Etv_term (Process terminate handoff vector)
//   $40C.L      Etv_xtra (Space for additional GEM vectors)
//   $410.L      Etv_xtra
//   $414.L      Etv_xtra
//   $418.L      Etv_xtra
//   $41C.L      Etv_xtra

//   $424.B      Memcntlr (Memory controller low nibble)
//   $425.B      Unassigned


//   $432.L      _Membot (Available memory bottom)
//   $436.L      _Memtop (Available memory top)

//   $43E.W      Flock (Floppy FIFO lock variable)
//   $440.W      Seekrate (Floppy seekrate)
//   $442.W      _Timr_ms (System timer calibration)
//   $446.W      _Fverify (Floppy verify flag)
//   $448.W      Palmode
//   $44A.B      Desfshftmd (Default video res if monitor changed)
//   $44B.B      Unassigned
//   $44C.B      Sshiftmd (Shadow shiftmode hardware register)
//   $44D.B      Unassigned
//   $452.W      Vblsem (Vertical blank mutual exclusionm semaphore)
//   $454.W      Nvbls (No. of longwords 'vblqueue' points to)
//   $456.L      _Vblqueue (Vblank handler pointer to pointers)
//   $45A.L      Colorptr
//   $45E.L      Screenpt (Screen base next vbl pointer)
//   $462.L      _vbclock (Vertical blank interrupt count)
//   $466.L      _Frclock (Count vblank interrupts not vblsem'd)
//   $46A.L      Hdv_init (Hard disk intitialise vector)
//   $46E.L      Swv_vec (Monitor changed vector)
//   $472.L      Hdv_bpb (Hard disk vector to return BPB)
//   $476.L      Hdv_rw (Hard disk vector to read/write)
//   $47A.L      Hdv_boot (Hard disk boot routine vector)
//   $47E.L      Hdv_mediach (Disk media change routine vector)
//   $482.W      _Cmdload
//   $484.B      Conterm (Console sys)
//   $485.B      Unassigned
//   $486.L      Trp14ret (Saved Trap #14 return address)
//   $48A.L      Criticret (Saved return address for Etv_critic)
//   $48E.L      Themd (GEMDOS memory descriptors) (M_link)
//   $492.L      Themd (M_start)
//   $496.L      Themd (M_length)
//   $49A.L      Themd (M_own)
//   $49E.L      _Md
//   $4A2.L      Savptr (BIOS register save area pointer)
//   $4A8.L      Con_state (State of Conout() parser)
//   $4AC.W      Save_row (Save row# for x-y addressing)
//   $4AE.L      Save_contxt (Pointer to saved processor context)
//   $4B2.L      _Bufl (Data sector buffer)
//   $4B6.L      _Bufl (FAT and DIR sectors buffer)
//   $4BA.L      _Hz_200 (Raw 200 Hz timer tick)
//   $4BE.L      The_env (Default environment string)
//   $4C2.L      _Drvbits (32 bit vector of live block devices)
//   $4C6.L      Dskbufp (Pointer to common disk buffer - 1 Kb)
//   $4CA.L      _Autopath (Pointer to autoexec path)

//   $4CE.L      _Vbl_list (A total of 8 longwords that are executed
//                          at every vertical blank)
//   $4D2.L      _Vbl_list
//   $4D6.L      _Vbl_list
//   $4DA.L      _Vbl_list
//   $4DE.L      _Vbl_list
//   $4E2.L      _Vbl_list
//   $4E6.L      _Vbl_list
//   $4EA.L      _Vbl_list
//   $4EE.W      _Prt_cnt (Print counter, intially -1, ALT-HELP inc)
//   $4F0.W      _Prtabt (Printer abort flag)
//   $4F2.L      _Sysbase (Base of OS pointer)
//   $4F6.L      _Shell_p (Global shell info pointer)
//   $4FA.L      End_os (Pointer to end of OS memory usage)
//   $4FE.L      Exec_os (Pointer to shell addr. to exec on startup)
//   $502.L*     Hardcopy (Hardcopy routine vector)
//   $506.L*     Listin (Parallel port status routine)
//   $50A.L*     Lstout (Output character to parallel port routine)
//   $50E.L*     Auxostat (RS232 output status routine)
//   $512.L*     Auxout (RS232 output routine)
//   $516-$83F   Unassigned

// Other more or less useful addresses

//   $840  *     Pathname buffer (e.g. PATH= A:\)

//   $93A.L*     Return address for Auto files
//   $93E.L*     Pathname address (\AUTO\*.PRG)
//   $942.L*     Filename address (\*.PRG)
//   $946.L*     DMA address

//   $964  *     Filename in autofolder buffer (e.g. NAME.PRG)
//   $972  *     Whole name buffer (e.g. \AUTO\NAME.PRG)

// Memory address that are used by the OS hardcopy routine

//   $992.L*     Buffer for '_V_bas_ad' when hardcopy
//   $996.W*     Offset
//   $998.W*     Screen width
//   $99A.W*     Screen height
//   $99C.W*     Left
//   $99E.W*     Right
//   $9A0.W*     Buffer for 'sshiftmd'
//   $9A2.W*     Quality mode
//   $9A4.L*     Color palette address
//   $9A8.W*     Printer table
//   $9AA.W*     Parallel/serial flag
//   $9AC.W*     Mask pointer

// Memory addresses that are used by the floppies

//   $9B0.W*     Retrycnt (Retry count)
//   $9B2.W*     Write Protect status
//   $9B4.W*     Cdev ('wplatch'?)

//   $9C6  *     Sector number
//   $9CC.B*     CDMA (DMA buffer for bad sector list)
//   $9CD.B*     DMA high
//   $9CE.B*     DMA mid
//   $9CF.B*     DMA low

//   $A06  *     DSB Drive A

// Buffers

//   $A0E  *     RS232 Input buffer
//   $B0E  *     RS232 Output buffer
//   $C0E  *     Keyboard buffer
//   $D0E  *     MIDI buffer
//   $DCC  *     Keyboard/MIDI table
//   $E28  *     Mouse buffer

// Addresses that are used by XBIOS function 32, 'Dosound'

//   $E44.L*     Music data pointer
//   $E48.L*     Temporary storage register

// Miscellaneous other addresses

//  $167A-$1879* Sector buffer (Boot sector)

//  $29B4.L*     Max access time *20 ms

//  $5220  *     Directory buffer

pub fn st1040() -> Configuration {
    let mut bus = Bus::new();
    bus.attach(CartridgeROM::new(0xfffa0000));
    bus.attach(Ram::new(0xff8000));
    bus.attach(Monitor::new(0xff3f8000, 0xffff8201));
    bus.attach(Blitter::new(0xffff8a00));
    bus.attach(MMU::new(0xffff8000));
    bus.attach(Floppy::new(0xffff8600, "examples/ST0001 Mono Demos.st"));
    bus.attach(SoundGenerator::new(0xffff8800));
    bus.attach(MultiFunctionPeripheral::new(0xfffffa01));
    bus.attach(Keyboard::new(0xfffffc00));
    bus.attach(MIDIAdapter::new(0xfffffc04));
    bus.attach(Microwire::new(0xffff8922));
    bus.attach(DMASoundSystem::new(0xffff8900));
    bus.attach(SystemControlUnit::new(0xffff8e00));
    bus.attach(JoystickPort::new(0xffff9200));
    bus.attach(RealTimeClock::new(0xfffffc20));

    Configuration {
        base_address: BASE_ADDRESS,
        start_address: START_ADDRESS,
        initial_ssp: INITIAL_SSP,
        bus: bus,
        memory_layout: Vec::from(MEMORY_LAYOUT),
    }
}

