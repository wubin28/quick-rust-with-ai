#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::InputPin;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::Timer;
use panic_semihosting as _;

struct XorShiftRng {
    state: u32,
}

impl XorShiftRng {
    fn new(seed: u32) -> Self {
        XorShiftRng {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn next_range(&mut self, range: usize) -> usize {
        (self.next() as usize) % range
    }
}

const MATRIX_DIMENSION: usize = 5;
const MAX_BRIGHTNESS: u8 = 9;
const MIN_BRIGHTNESS: u8 = 0;
const DURATION_100_MS: u32 = 100;
const DURATION_1000_MS: u32 = 1000;
const PATTERN_NUM: usize = 10;

static SMILEY: [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION] = [
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 1],
    [0, 1, 1, 1, 0],
];

static PATTERNS: [[[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION]; PATTERN_NUM] = [
    // 0 Heart shape
    [
        [0, 1, 0, 1, 0],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
    ],
    // 1 Up arrow
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 0, 1, 0, 1],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    // 2 Solid square
    [
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 1, 1, 1, 1],
        [0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
    ],
    // 3 Hollow square
    [
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
    ],
    // 4 X shape
    [
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
    ],
    // 5 Hollow pointed roof
    [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0],
    ],
    // 6 Circle
    [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ],
    // 7 Plus sign
    [
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [1, 1, 1, 1, 1],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
    ],
    // 8 Lightning bolt
    [
        [1, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 1, 0],
    ],
    // 9 Wave
    [
        [0, 0, 0, 0, 0],
        [1, 0, 1, 0, 1],
        [1, 1, 1, 1, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0],
    ],
];

#[derive(PartialEq)]
enum GameState {
    ShowingSmiley,
    ShowingRandomPattern,
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);
    let mut current_state = GameState::ShowingSmiley;
    let button_a = board.buttons.button_a;

    let seed = timer.read();
    let mut rng = XorShiftRng::new(seed);

    let mut display_buffer = [[MIN_BRIGHTNESS; MATRIX_DIMENSION]; MATRIX_DIMENSION];
    copy_pattern_to_buffer(&SMILEY, &mut display_buffer);

    loop {
        match current_state {
            GameState::ShowingSmiley => {
                display.show(&mut timer, display_buffer, DURATION_100_MS);
                let is_button_a_pressed = button_a.is_low().unwrap();
                if is_button_a_pressed {
                    clear_buffer(&mut display_buffer);
                    display.show(&mut timer, display_buffer, DURATION_100_MS);
                    timer.delay_ms(DURATION_1000_MS);
                    current_state = GameState::ShowingRandomPattern;
                }
            }

            GameState::ShowingRandomPattern => {
                let pattern_index = rng.next_range(PATTERN_NUM);

                copy_pattern_to_buffer(&PATTERNS[pattern_index], &mut display_buffer);
                display.show(&mut timer, display_buffer, DURATION_1000_MS);
                timer.delay_ms(DURATION_1000_MS);
            }
        }
    }
}

fn copy_pattern_to_buffer(
    pattern: &[[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION],
    buffer: &mut [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION],
) {
    for (row, pattern_row) in pattern.iter().enumerate() {
        for (col, &value) in pattern_row.iter().enumerate() {
            buffer[row][col] = if value == 1 {
                MAX_BRIGHTNESS
            } else {
                MIN_BRIGHTNESS
            };
        }
    }
}

fn clear_buffer(buffer: &mut [[u8; MATRIX_DIMENSION]; MATRIX_DIMENSION]) {
    for (row, buffer_row) in buffer.iter_mut().enumerate() {
        for (col, cell) in buffer_row.iter_mut().enumerate() {
            *cell = MIN_BRIGHTNESS;
        }
    }
}
