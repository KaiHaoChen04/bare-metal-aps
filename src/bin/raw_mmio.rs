#![no_main]
#![no_std]

extern crate panic_halt as _;

mod interrupts;

use core::mem::size_of;
use cortex_m_rt::entry;

// universal address for GPI0 port 0
const GPI0_PO: usize = 0x5000_0000;

//GPI0 offsets
const PIN_CNF: usize = 0x700;
const OUTSET: usize = 0x508;
const OUTCLR: usize = 0x50c;

//PIN_CNF fields
const DIR_OUTPUT: u32 = 0x1;
const INPUT_DISCONNECT: u32 = 0x1 << 1;
const PULL_DISABLE: u32 = 0x0 << 2; // disable pull up&down resistors
const DRIVE_S0S1: u32 = 0x0 << 8; // drive strength
const SENSE_DISABLED: u32 = 0x0 << 16;

#[entry]
fn main() -> ! {
    // memory to raw pointer
    let pin_cnf_21 = (GPI0_PO + PIN_CNF + 21 * size_of::<u32>()) as *mut u32;
    let pin_cnf_28 = (GPI0_PO + PIN_CNF + 28 * size_of::<u32>()) as *mut u32;

    unsafe {
        //write volatile so force cpu to execute memory write
        pin_cnf_21.write_volatile(
            DIR_OUTPUT | INPUT_DISCONNECT | PULL_DISABLE | DRIVE_S0S1 | SENSE_DISABLED,
        );
        pin_cnf_28.write_volatile(
            DIR_OUTPUT | INPUT_DISCONNECT | PULL_DISABLE | DRIVE_S0S1 | SENSE_DISABLED,
        );
    }

    //compute raw pointer register
    let gpio0_outset = (GPI0_PO + OUTSET) as *mut u32;
    let gpio0_outclr = (GPI0_PO + OUTCLR) as *mut u32;

    unsafe {
        //shift one bit to each corresponding bit position
        gpio0_outclr.write_volatile(1 << 28);
        gpio0_outset.write_volatile(1 << 21);
    }
    loop {}
}
