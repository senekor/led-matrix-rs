#![no_std]
#![no_main]

// halt the program on panic (as opposed to unwinding the stack)
use panic_halt as _;

use zhaw_led_matrix_bsp::LedMatrix;

#[rp_pico::entry]
fn main() -> ! {
    let mut led_matrix = LedMatrix::take().unwrap();

    let mut brightness: f32 = 0.25;
    let mut hue: f32 = 1.0;

    loop {
        if led_matrix.joystick_is_up() {
            brightness += 0.01;
            if brightness > 1.0 {
                brightness = 1.0;
            }
            led_matrix.set_brighness(brightness);
        }
        if led_matrix.joystick_is_down() {
            brightness -= 0.01;
            if brightness < 0.0 {
                brightness = 0.0;
            }
            led_matrix.set_brighness(brightness);
        }
        if led_matrix.joystick_is_right() {
            hue += 0.01;
            if hue > 1.0 {
                hue -= 1.0;
            }
        }
        if led_matrix.joystick_is_left() {
            hue -= 0.01;
            if hue < 0.0 {
                hue += 1.0;
            }
        }

        let color = hsv2rgb_u8(hue * 360.0, 1.0, 1.0).into();

        led_matrix.update(|_, _, led| {
            *led = color;
        });
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
