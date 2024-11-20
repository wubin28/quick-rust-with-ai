#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

const LETTERS: [[u8; 5]; 26] = [
    // A
    [0b11111,
     0b10001,
     0b11111,
     0b10001,
     0b10001],
    // B
    [0b11110,
     0b10001,
     0b11110,
     0b10001,
     0b11110],
    // C
    [0b11111,
     0b10000,
     0b10000,
     0b10000,
     0b11111],
    // D
    [0b11110,
     0b10001,
     0b10001,
     0b10001,
     0b11110],
    // E
    [0b11111,
     0b10000,
     0b11110,
     0b10000,
     0b11111],
    // F
    [0b11111,
     0b10000,
     0b11110,
     0b10000,
     0b10000],
    // G
    [0b11111,
     0b10000,
     0b10111,
     0b10001,
     0b11111],
    // H
    [0b10001,
     0b10001,
     0b11111,
     0b10001,
     0b10001],
    // I
    [0b11111,
     0b00100,
     0b00100,
     0b00100,
     0b11111],
    // J
    [0b11111,
     0b00100,
     0b00100,
     0b10100,
     0b11100],
    // K
    [0b10001,
     0b10010,
     0b11100,
     0b10010,
     0b10001],
    // L
    [0b10000,
     0b10000,
     0b10000,
     0b10000,
     0b11111],
    // M
    [0b10001,
     0b11011,
     0b10101,
     0b10001,
     0b10001],
    // N
    [0b10001,
     0b11001,
     0b10101,
     0b10011,
     0b10001],
    // O
    [0b11111,
     0b10001,
     0b10001,
     0b10001,
     0b11111],
    // P
    [0b11111,
     0b10001,
     0b11111,
     0b10000,
     0b10000],
    // Q
    [0b01110,
     0b10001,
     0b10101,
     0b10010,
     0b01101],
    // R
    [0b11111,
     0b10001,
     0b11110,
     0b10010,
     0b10001],
    // S
    [0b11111,
     0b10000,
     0b11111,
     0b00001,
     0b11111],
    // T
    [0b11111,
     0b00100,
     0b00100,
     0b00100,
     0b00100],
    // U
    [0b10001,
     0b10001,
     0b10001,
     0b10001,
     0b11111],
    // V
    [0b10001,
     0b10001,
     0b10001,
     0b01010,
     0b00100],
    // W
    [0b10001,
     0b10001,
     0b10101,
     0b11011,
     0b10001],
    // X
    [0b10001,
     0b01010,
     0b00100,
     0b01010,
     0b10001],
    // Y
    [0b10001,
     0b01010,
     0b00100,
     0b00100,
     0b00100],
    // Z
    [0b11111,
     0b00010,
     0b00100,
     0b01000,
     0b11111],
];

struct DisplayState {
    led_matrix: [[u8; 5]; 5],
    scroll_pos: i32,
    text: &'static [u8],
}

impl DisplayState {
    fn new(text: &'static [u8]) -> Self {
        Self {
            led_matrix: [[0; 5]; 5],
            scroll_pos: 0,
            text,
        }
    }

    fn clear_matrix(&mut self) {
        self.led_matrix = [[0; 5]; 5];
    }

    fn get_current_char_info(&self) -> (usize, i32) {
        let text_len = self.text.len();
        let current_char_idx = (self.scroll_pos.rem_euclid(6 * text_len as i32) / 6) as usize;
        let x_offset = (self.scroll_pos % 6) - 4;
        (current_char_idx, x_offset)
    }

    fn render_letter(&mut self, char_idx: usize, char_x_offset: i32) {
        if char_x_offset <= -5 || char_x_offset >= 5 {
            return;
        }

        let current_char = self.text[char_idx];
        if !is_valid_letter(current_char) {
            return;
        }

        let letter_pattern = LETTERS[(current_char - b'A') as usize];
        self.render_letter_pattern(letter_pattern, char_x_offset);
    }

    fn render_letter_pattern(&mut self, letter_pattern: [u8; 5], x_offset: i32) {
        for row in 0..5 {
            for col in 0..5 {
                let x = 4 - col as i32 - x_offset;
                let y = 4 - row;
                
                if x >= 0 && x < 5 {
                    self.led_matrix[y as usize][x as usize] = 
                        if (letter_pattern[4 - row] & (1 << col)) != 0 { 1 } else { 0 };
                }
            }
        }
    }

    fn update_scroll_position(&mut self) {
        self.scroll_pos = (self.scroll_pos + 1) % (self.text.len() as i32 * 6);
    }
}

fn is_valid_letter(c: u8) -> bool {
    c >= b'A' && c <= b'Z'
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let display_text = b"RUST";
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut state = DisplayState::new(display_text);

    loop {
        state.clear_matrix();
        let (current_char_idx, x_offset) = state.get_current_char_info();

        for i in 0..2 {
            let char_idx = (current_char_idx + i) % state.text.len();
            let char_x_offset = x_offset - (i as i32 * 6);
            state.render_letter(char_idx, char_x_offset);
        }

        display.show(&mut timer, state.led_matrix, 100);
        state.update_scroll_position();
        timer.delay_ms(150_u32);
    }
}