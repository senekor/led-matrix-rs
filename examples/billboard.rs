#![no_std]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use panic_halt as _;

use led_matrix::billboard::{vertical, Billboard};

static ZIG_ZAG: Billboard = &vertical([
    *b"#       ",
    *b" #      ",
    *b"  #     ",
    *b"   #    ",
    *b"    #   ",
    *b"     #  ",
    *b"      # ",
    *b"       #",
    *b"      # ",
    *b"     #  ",
    *b"    #   ",
    *b"   #    ",
    *b"  #     ",
    *b" #      ",
    *b"#       ",
    *b" #      ",
    *b"  #     ",
    *b"   #    ",
    *b"    #   ",
    *b"     #  ",
    *b"      # ",
    *b"       #",
]);

#[cfg_attr(target_os = "none", rp_pico::entry)]
fn main() -> ! {
    led_matrix::run(app);
}

fn app(matrix: &mut dyn led_matrix::LedMatrix) {
    loop {
        for offset in 0..ZIG_ZAG.len() {
            matrix.draw_vertical_billboard_frame(ZIG_ZAG, offset);
            matrix.apply();
            matrix.sleep_ms(100);
        }
    }
}
