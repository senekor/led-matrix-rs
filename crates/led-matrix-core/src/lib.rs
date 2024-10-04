#![no_std]

/// A high-level wrapper around peripherals and LED libraries
/// to program the LED matrix easily.
///
/// To update the color of an LED, you can index the `LedMatrix` with a tuple
/// of integers (row, column) in the range `0..8` each.
///
/// After changing the values of one or several LEDs, don't forget to call
/// `apply` to actually apply these changes in a batch.
///
/// The LEDs are set at a default brightness of about 20%, which can be
/// changed with `set_brightness`. This has no effect on the TUI emulator.
pub trait LedMatrix:
    core::ops::Index<(usize, usize), Output = (u8, u8, u8)> + core::ops::IndexMut<(usize, usize)>
{
    /// Tell the LED matrix to display the currently stored color values for
    /// each LED.
    ///
    /// If you are drawing in an endless loop, consider calling
    /// [Self::sleep_ms] at some point to slow down the execution.
    fn apply(&mut self);

    fn set_brighness(&mut self, brightness: u8);

    fn sleep_ms(&mut self, duration: u32);

    /// Get a sinus function.
    /// This is necessary to abstract over hardware and emulator.
    fn get_sin(&self) -> fn(f32) -> f32;

    fn joystick_position(&mut self) -> JoystickPosition;

    // ---------------- default implementated methods ---------------- //

    /// Set every LED to a single color at the same time.
    fn fill(&mut self, color: (u8, u8, u8)) {
        for row in 0..8 {
            for column in 0..8 {
                self[(row, column)] = color;
            }
        }
    }

    /// Turn off all LEDs.
    fn clear(&mut self) {
        self.fill((0, 0, 0));
    }

    /// Set a list of LEDs to a single color.
    fn draw_list<T: AsRef<[(usize, usize)]>>(&mut self, list: T, color: (u8, u8, u8)) {
        for (row, column) in list.as_ref() {
            self[(*row, *column)] = color;
        }
    }

    // TODO: draw_line
}

/// Represents all five positions the joystick can be in.
/// It is returned by [LedMatrix::joystick_position].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoystickPosition {
    Center,
    Up,
    Down,
    Left,
    Right,
}
