#![no_std]
#![cfg_attr(target_os = "none", no_main)]

use led_matrix_core::JoystickPosition;
#[cfg(target_os = "none")]
use panic_halt as _;

use led_matrix::color;

#[cfg_attr(target_os = "none", rp_pico::entry)]
fn main() -> ! {
    led_matrix::run(app);
}

fn app(matrix: &mut dyn led_matrix::LedMatrix) {
    let mut x: usize = 0;
    let mut y: usize = 0;

    loop {
        match matrix.joystick_position() {
            JoystickPosition::Center => { /* no input */ }
            JoystickPosition::Up => y = (y + 1).min(7),
            JoystickPosition::Down => y = y.saturating_sub(1),
            JoystickPosition::Right => x = (x + 1).min(7),
            JoystickPosition::Left => x = x.saturating_sub(1),
        }

        let color = match matrix.switch() {
            true => color::RED,
            false => color::WHITE,
        };

        matrix.clear();
        matrix[(x, y)] = color;
        matrix.apply();
        matrix.sleep_ms(33)
    }
}
