#![no_main]
#![no_std]

extern crate panic_halt as _;

use core::fmt::Write;
use cortex_m_rt::entry;
use microbit::{
    hal::{
        uarte::{Baudrate, Parity, Uarte},
        Delay,
    },
    Board,
};

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    loop {}
}
