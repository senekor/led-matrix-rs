#![no_std]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use panic_halt as _;

use led_matrix::{bitmap, character::convert_str};

#[cfg_attr(target_os = "none", rp_pico::entry)]
fn main() -> ! {
    led_matrix::run(app);
}

fn app(matrix: &mut dyn led_matrix::LedMatrix) {
    let (billboard, length) = &convert_str(*b" REWRITE IT IN RUST ");

    loop {
        for offset in 0..*length {
            matrix.draw_text_billboard_frame(billboard, offset);
            matrix.apply();
            matrix.sleep_ms(50);
        }

        for _ in 0..3 {
            matrix.draw_bitmap(bitmap::CRAB);
            matrix.apply();
            matrix.sleep_ms(1_000);
            matrix.clear();
            matrix.apply();
            matrix.sleep_ms(500);
        }
    }
}
