#![no_main]
#![no_std]

use cortex_m_rt::entry;
use core::fmt::Write;
use rtt_target::rtt_init_print;
use heapless::Vec;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer, uarte},
    hal::uarte::{Baudrate, Parity},
};

mod letters;
use crate::letters::char_to_led;
mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            let byte = nb::block!(serial.read()).unwrap();

            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full \r\n").unwrap();
                break;
            }

            if byte == 13 {
                break;
            }
        }

        for byte in buffer.iter() {
            display.show(&mut timer, char_to_led(&(*byte as char)), 500);
        }
        
        nb::block!(serial.flush()).unwrap();
    }
}
