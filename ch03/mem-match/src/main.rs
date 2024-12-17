#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::Timer;
use panic_semihosting as _;

const MATRIX_DIMENSION: usize = 5;
const MAX_BRIGHTNESS: u8 = 9;
const MIN_BRIGHTNESS: u8 = 0;
const DURATION_100_MS: u32 = 100;

static SMILEY: [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION] = [
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
    let mut display_buffer = [[MIN_BRIGHTNESS; MATRIX_DIMENSION]; MATRIX_DIMENSION];

    for row in 0..MATRIX_DIMENSION {
        for col in 0..(MATRIX_DIMENSION + 1) {
            display_buffer[row][col] = if SMILEY[row][col] == 1 {
                MAX_BRIGHTNESS
            } else {
                MIN_BRIGHTNESS
            };
        }
    }

    loop {
        display.show(&mut timer, display_buffer, DURATION_100_MS);
    }
}
