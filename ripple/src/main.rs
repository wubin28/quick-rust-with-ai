#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

// 定义4种大小的圆形涟漪
const CIRCLE_1: [(usize, usize); 1] = [
    (2, 2),  // 中心点
];

const CIRCLE_2: [(usize, usize); 4] = [
    (1, 2), (2, 1),
    (2, 3), (3, 2),
];

const CIRCLE_3: [(usize, usize); 8] = [
    (1, 1), (1, 3),
    (2, 0), (2, 4),
    (3, 1), (3, 3),
    (0, 2), (4, 2),
];

const CIRCLE_4: [(usize, usize); 12] = [
    (0, 1), (0, 3),
    (1, 0), (1, 4),
    (3, 0), (3, 4),
    (4, 1), (4, 3),
    (0, 2), (4, 2),
    (2, 0), (2, 4),
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    
    // 涟漪状态：0=最小, 1=小, 2=中, 3=大
    let mut ripple_state = 0;
    
    loop {
        let mut led_matrix = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
        
        // 根据状态选择显示的圆形大小
        let ripple_points = match ripple_state {
            0 => &CIRCLE_1[..],
            1 => &CIRCLE_2[..],
            2 => &CIRCLE_3[..],
            3 => &CIRCLE_4[..],
            _ => &CIRCLE_1[..],
        };
        
        // 点亮对应的LED
        for &(row, col) in ripple_points {
            led_matrix[row][col] = 1;
        }
        
        // 显示当前LED矩阵
        display.show(&mut timer, led_matrix, 100);
        
        // 更新涟漪状态
        ripple_state = (ripple_state + 1) % 4;
        
        // 控制涟漪扩散速度
        timer.delay_ms(150_u32);
    }
}