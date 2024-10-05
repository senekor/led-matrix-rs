#![no_std]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use panic_halt as _;

use led_matrix::{
    billboard::{vertical, Billboard},
    LedMatrix,
};

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
    let mut matrix = led_matrix::init();

    loop {
        for offset in 0..ZIG_ZAG.len() {
            matrix.draw_vertical_billboard_section(ZIG_ZAG, offset);
            matrix.apply();
            matrix.sleep_ms(100);
        }
    }
}
