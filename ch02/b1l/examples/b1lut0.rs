#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);

    loop {
        board.display_pins.col4.set_low().unwrap();
        board.display_pins.row4.set_high().unwrap();

        timer.delay_ms(500_u32);

        board.display_pins.row4.set_low().unwrap();

        timer.delay_ms(500_u32);
    }
}