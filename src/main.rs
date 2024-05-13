#![no_std]
#![no_main]

// halt the program on panic (as opposed to unwinding the stack)
use panic_halt as _;

use zhaw_led_matrix_bsp::LedMatrix;

#[rp_pico::entry]
fn main() -> ! {
    let mut led_matrix = LedMatrix::take().unwrap();

    led_matrix.set_brighness(0.25);

    let mut t = 0.0;

    // Slow down timer by this factor (0.1 will result in 10 seconds):
    let animation_speed = 0.1;

    loop {
        led_matrix.update(|i, j, led| {
            let distance = i + j; // max: 14

            // An offset to give 3 consecutive LEDs a different color:
            let hue_offs = f32::from(distance as u16) / 14.0 / 4.0;

            let sin_11 = LedMatrix::sin((t + hue_offs) * 2.0 * core::f32::consts::PI);
            // Bring -1..1 sine range to 0..1 range:
            let sin_01 = (sin_11 + 1.0) * 0.5;

            let hue = 360.0 * sin_01;
            let sat = 1.0;
            let val = 1.0;

            let rgb = hsv2rgb_u8(hue, sat, val);
            *led = rgb.into();
        });

        // Increase the time counter variable and make sure it
        // stays inbetween 0.0 to 1.0 range:
        t += (16.0 / 1000.0) * animation_speed;
        while t > 1.0 {
            t -= 1.0;
        }
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
