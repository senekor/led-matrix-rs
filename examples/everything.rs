#![no_std]
#![cfg_attr(target_os = "none", no_main)]

use led_matrix::{
    billboard::{horizontal, Billboard},
    bitmap,
    character::convert_str,
    color, JoystickPosition,
};
#[cfg(target_os = "none")]
use panic_halt as _;

static SHRUG_KAOMOJI: Billboard = &horizontal([
    *b"   ###          #    #    # #          ###   ",
    *b"      #        #  #  ##   #  #        #      ",
    *b"      #        #  ##  ##  #  #        #      ",
    *b"       #      #    ##    #    #      #       ",
    *b"       #      #          #    #      #       ",
    *b"        #      #        #    #      #        ",
    *b"        #      #      ##     #      #        ",
    *b"         #####  #  ###      #  #####         ",
]);

#[cfg_attr(target_os = "none", rp_pico::entry)]
fn main() -> ! {
    led_matrix::run(app);
}

fn app(matrix: &mut dyn led_matrix::LedMatrix) {
    'main_loop: loop {
        // execute different code based on inputs
        if matrix.switch() {
            let (mut x, mut y): (usize, usize) = (0, 0);
            while matrix.switch() {
                match matrix.joystick_position() {
                    JoystickPosition::Center => { /* no input */ }
                    JoystickPosition::Up => y = (y + 1).min(7),
                    JoystickPosition::Down => y = y.saturating_sub(1),
                    JoystickPosition::Right => x = (x + 1).min(7),
                    JoystickPosition::Left => x = x.saturating_sub(1),
                }
                matrix.clear();
                matrix[(x, y)] = color::WHITE;
                matrix.apply();
                matrix.sleep_ms(33)
            }
        }

        // draw a blinking bitmap
        for _ in 0..3 {
            matrix.draw_bitmap(bitmap::CRAB);
            matrix.apply();
            matrix.sleep_ms(1_000);
            matrix.clear();
            matrix.apply();
            matrix.sleep_ms(500);
        }
        if matrix.switch() {
            continue 'main_loop;
        }

        // run some text across the screen
        let (billboard, length) = &convert_str(*b" REWRITE IT IN RUST ");
        for offset in 0..*length {
            matrix.draw_text_billboard_frame(billboard, offset);
            matrix.apply();
            matrix.sleep_ms(50);
        }
        if matrix.switch() {
            continue 'main_loop;
        }

        // draw a custom-drawn billboard
        for offset in 0..SHRUG_KAOMOJI.len() {
            matrix.draw_horizontal_billboard_frame(SHRUG_KAOMOJI, offset);
            matrix.apply();
            matrix.sleep_ms(100);
        }
    }
}
