#![no_std]

pub trait LedMatrix {
    /// Get a mutable reference to the state of an LED. The state is represented
    /// as RGB values, i.e. a tuple of three bytes.
    ///
    /// Once you are done updating the values, don't forget to call
    /// [Self::draw]!
    fn led_mut(&mut self, row: usize, column: usize) -> &mut (u8, u8, u8);

    /// Tell the LED matrix to display the currently stored color values for
    /// each LED.
    ///
    /// If you are drawing in an endless loop, consider calling
    /// [Self::sleep_ms] at some point to slow down the execution.
    fn draw(&mut self);

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
                *self.led_mut(row, column) = color;
            }
        }
    }

    /// Turn off all LEDs.
    fn clear(&mut self) {
        self.fill((0, 0, 0));
    }

    /// Set a list of LEDs to a single color.
    fn fill_list<T: AsRef<[(usize, usize)]>>(&mut self, list: T, color: (u8, u8, u8)) {
        for (row, column) in list.as_ref() {
            *self.led_mut(*row, *column) = color;
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
