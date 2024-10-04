#![no_std]
#![cfg_attr(target_os = "none", no_main)]

use led_matrix_core::JoystickPosition;
#[cfg(target_os = "none")]
use panic_halt as _;

use led_matrix::{all_led_coordinates, LedMatrix};

#[cfg_attr(target_os = "none", rp_pico::entry)]
fn main() -> ! {
    let mut matrix = led_matrix::init();

    let mut brightness: u8 = 64;
    let mut hue: f32 = 1.0;

    loop {
        match matrix.joystick_position() {
            JoystickPosition::Center => { /* no input */ }
            JoystickPosition::Up => {
                brightness = brightness.saturating_add(1);
                matrix.set_brighness(brightness);
            }
            JoystickPosition::Down => {
                brightness = brightness.saturating_sub(1);
                matrix.set_brighness(brightness);
            }
            JoystickPosition::Right => {
                hue += 0.01;
                if hue > 1.0 {
                    hue -= 1.0;
                }
            }
            JoystickPosition::Left => {
                hue -= 0.01;
                if hue < 0.0 {
                    hue += 1.0;
                }
            }
        }

        let color = hsv2rgb_u8(hue * 360.0, 1.0, 1.0);

        for (row, column) in all_led_coordinates() {
            *matrix.led_mut(row, column) = color
        }
        matrix.draw();
        matrix.sleep_ms(16)
    }
}

pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    let c = val * sat;
    let v = (hue / 60.0) % 2.0 - 1.0;
    let v = if v < 0.0 { -v } else { v };
    let x = c * (1.0 - v);
    let m = val - c;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

pub fn hsv2rgb_u8(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let r = hsv2rgb(h, s, v);

    (
        (r.0 * 255.0) as u8,
        (r.1 * 255.0) as u8,
        (r.2 * 255.0) as u8,
    )
}
