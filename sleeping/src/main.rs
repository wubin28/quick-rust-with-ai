#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

// 修正Z字母的LED点阵大小为7
const Z_PATTERN: [(usize, usize); 7] = [
    (0, 0), (0, 1), (0, 2),  // 顶部横线
    (1, 1),                   // 中间斜线
    (2, 0), (2, 1), (2, 2)   // 底部横线
];

const Z_POSITIONS: [(i32, i32); 3] = [
    (2, 0),    // 第一个Z的起始位置
    (1, 2),    // 第二个Z的起始位置
    (0, 4)     // 第三个Z的起始位置
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    
    let mut frame = 0;
    
    loop {
        let mut led_matrix = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
        
        for (z_idx, &(base_row, base_col)) in Z_POSITIONS.iter().enumerate() {
            let offset = (frame + z_idx * 3) % 15;
            let current_row = base_row - offset as i32;
            let current_col = base_col + offset as i32;
            
            if current_row >= 0 && current_row <= 2 && current_col >= 0 && current_col <= 2 {
                for &(z_row, z_col) in &Z_PATTERN {
                    let pixel_row = (current_row + z_row as i32) as usize;
                    let pixel_col = (current_col + z_col as i32) as usize;
                    
                    if pixel_row < 5 && pixel_col < 5 {
                        led_matrix[pixel_row][pixel_col] = 1;
                    }
                }
            }
        }
        
        display.show(&mut timer, led_matrix, 100);
        frame = (frame + 1) % 15;
        timer.delay_ms(200_u32);
    }
}