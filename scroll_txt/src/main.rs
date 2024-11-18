#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

// 调整字母矩阵为顺时针旋转90度后的样式
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

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let display_text = b"HELLO";

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut scroll_pos: i32 = 0;
    let text_len = display_text.len();

    loop {
        let mut led_matrix = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];

        let current_char_idx = (scroll_pos.rem_euclid(6 * text_len as i32) / 6) as usize;
        let x_offset = (scroll_pos % 6) - 4 ;

        for i in 0..2 {
            let char_idx = (current_char_idx + i) % text_len;
            let char_x_offset = x_offset - (i as i32 * 6);

            if char_x_offset > -5 && char_x_offset < 5 {
                let current_char = display_text[char_idx];
                if current_char >= b'A' && current_char <= b'Z' {
                    let letter_pattern = LETTERS[(current_char - b'A') as usize];

                    for row in 0..5 {
                        for col in 0..5 {
                            let x = 4 - col as i32 - char_x_offset; 
                            let y = 4 - row;                        
                            if x >= 0 && x < 5 {
                                led_matrix[y as usize][x as usize] =
                                    if (letter_pattern[4 - row] & (1 << col)) != 0 { 1 } else { 0 };
                            }
                        }
                    }
                }
            }
        }

        display.show(&mut timer, led_matrix, 100);

        scroll_pos = (scroll_pos + 1) % (text_len as i32 * 6);

        timer.delay_ms(150_u32);
    }
}