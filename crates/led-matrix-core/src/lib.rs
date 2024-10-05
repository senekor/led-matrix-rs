#![no_std]

/// A minimal specification of what an LED-matrix must be capable of.
///
/// This trait is non-user facing. All user-facing conveniences should be
/// defined on [led_matrix::LedMatrix] instead.
pub trait LedMatrixCore:
    core::ops::Index<(usize, usize), Output = (u8, u8, u8)> + core::ops::IndexMut<(usize, usize)>
{
    fn apply(&mut self);

    fn set_brightness(&mut self, brightness: u8);

    fn sleep_ms(&mut self, duration: u32);

    fn get_sin(&self) -> fn(f32) -> f32;

    fn joystick_position(&mut self) -> JoystickPosition;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoystickPosition {
    Center,
    Up,
    Down,
    Left,
    Right,
}

// Remove these when enabling support for multiple matrices to discover all
// places where code needs to change.
pub const HEIGHT: u8 = 8;
pub const WIDTH: u8 = 8;
