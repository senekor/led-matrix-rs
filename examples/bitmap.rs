#![no_std]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use panic_halt as _;

static SOME_BITMAPS: &[&[u8]] = &[
    led_matrix::bitmap::APPLE,
    led_matrix::bitmap::BAT,
    led_matrix::bitmap::BIRD,
    led_matrix::bitmap::CHICKEN,
    led_matrix::bitmap::CRAB,
    led_matrix::bitmap::DINO,
    led_matrix::bitmap::DUCK,
    led_matrix::bitmap::FOX,
    led_matrix::bitmap::MOUSE,
    led_matrix::bitmap::MUSHROOM,
    led_matrix::bitmap::PIKACHU,
    led_matrix::bitmap::RABBIT,
];

#[cfg_attr(target_os = "none", rp_pico::entry)]
fn main() -> ! {
    led_matrix::run(app);
}

fn app(matrix: &mut dyn led_matrix::LedMatrix) {
    loop {
        for bitmap in SOME_BITMAPS {
            matrix.draw_bitmap(bitmap);
            matrix.apply();
            matrix.sleep_ms(1_000)
        }
    }
}
