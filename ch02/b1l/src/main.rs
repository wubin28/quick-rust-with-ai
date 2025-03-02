#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use microbit::board::Board;
use panic_halt as _;

use cortex_m::asm::delay;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    loop {
        board.display_pins.col4.set_low().unwrap();
        board.display_pins.row4.set_high().unwrap();

        delay(1_000_000);

        board.display_pins.row4.set_low().unwrap();

        delay(1_000_000);
    }
}
