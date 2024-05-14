#![no_std]

pub trait LedMatrix {
    /// `1.0` for full brightness. Default is `0.25`.
    fn set_brighness(&mut self, brightness: f32);

    /// The given closure will be called for each LED.
    /// At the end, the new frame will be drawn followed by a frame delay.
    fn update(&mut self, f: impl FnMut(usize, usize, &mut (u8, u8, u8)));

    /// Get a sinus function.
    /// This is necessary to abstract over hardware and emulator.
    fn get_sin(&self) -> fn(f32) -> f32;

    fn joystick_is_up(&mut self) -> bool;
    fn joystick_is_down(&mut self) -> bool;
    fn joystick_is_left(&mut self) -> bool;
    fn joystick_is_right(&mut self) -> bool;
}
