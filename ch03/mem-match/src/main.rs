#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::Timer;
use panic_halt as _;

static SMILEY: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut display_buffer = [[0u8; 5]; 5];

    for row in 0..5 {
        for col in 0..5 {
            display_buffer[row][col] = if SMILEY[row][col] == 1 { 9 } else { 0 };
        }
    }

    loop {
        display.show(&mut timer, display_buffer, 100);
    }
}
