#![no_std]
#![no_main]

extern crate panic_halt as _;

use cortex_m_rt::entry;
use nfr52833_pac::Peripherals;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio0 = p.P0;

    // Configure GPIO 0 pins 21 and 28 as push-pull outputs.
    gpio0.pin_cnf[21].write(|w| {
       w.dir().output();
       w.input().disconnect();
       w.pull().disabled();
       w.drive().s0s1();
       w.sense().disabled();
    });
    
}
