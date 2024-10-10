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
    let mut just_painted = false;
    let mut canvas: [[(u8, u8, u8); 8]; 8] = Default::default();

    loop {
        match matrix.joystick_position() {
            JoystickPosition::Center => { /* no input */ }
            JoystickPosition::Up => y = (y + 1).min(7),
            JoystickPosition::Down => y = y.saturating_sub(1),
            JoystickPosition::Right => x = (x + 1).min(7),
            JoystickPosition::Left => x = x.saturating_sub(1),
        }
        if !just_painted && matrix.joystick_pressed() {
            just_painted = true;
            canvas[x][y] = cycle_color(canvas[x][y]);
        } else if just_painted && !matrix.joystick_pressed() {
            just_painted = false;
        }

        matrix.clear();
        for x in 0..8 {
            for y in 0..8 {
                matrix[(x, y)] = canvas[x][y];
            }
        }

        matrix[(x, y)] = if canvas[x][y] == color::WHITE {
            color::BLACK
        } else {
            color::WHITE
        };
        matrix.apply();
        matrix.sleep_ms(67)
    }
}

fn cycle_color(c: (u8, u8, u8)) -> (u8, u8, u8) {
    match c {
        color::BLACK => color::WHITE,
        color::WHITE => color::RED,
        color::RED => color::GREEN,
        color::GREEN => color::BLUE,
        color::BLUE => color::BLACK,
        _ => c, // unrecognized color, no-op
    }
}
