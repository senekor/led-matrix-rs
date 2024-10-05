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
        for row in 0..HEIGHT as usize {
            for column in 0..WIDTH as usize {
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

    /// Draw a bitmap file with a color depth of 24 bit.
    ///
    /// bitmap format: https://en.wikipedia.org/wiki/BMP_file_format
    fn draw_bitmap(&mut self, bitmap: &[u8]) {
        let color_depth = u16::from_le_bytes(bitmap[28..30].try_into().unwrap());
        let bitmap_size = u32::from_le_bytes(bitmap[2..6].try_into().unwrap());
        let bitmap_offset = u32::from_le_bytes(bitmap[10..14].try_into().unwrap());
        let image_size = u32::from_le_bytes(bitmap[34..38].try_into().unwrap());
        let bitmap_height = u16::from_be_bytes(bitmap[22..24].try_into().unwrap()) as u32;
        // let upper_left_origin = bitmap_height < 0;
        let pic = &bitmap[bitmap_offset as usize..];
        let bitmap_width = (image_size / 3) / bitmap_height;

        if bitmap_height > HEIGHT as u32 || bitmap_width > WIDTH as u32 {
            // TODO: How to debug? println not available on no_std.
            // println!(format!(
            //     "bitmap is larger than matrix: {bitmap_width}x{bitmap_height}"
            // ));
        }
        if (color_depth) != 24 {
            panic!("Wrong color-depth ({color_depth}) detected. Use bitmaps with a color-depth of 24 bits.");
        }
        if (bitmap_size) != 246 {
            // TODO: How to debug? println not available on no_std.
            // println!("The bitmap size is different than expected. The image may be defective.");
        }
        for row in 0..HEIGHT as usize {
            for column in 0..WIDTH as usize {
                let i = (row * WIDTH as usize + column) * 3;
                self[(row, column)] = (pic[i + 2], pic[i + 1], pic[i]);
            }
        }
    }
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

// Remove these when enabling support for multiple matrices to discover all
// places where code needs to change.
pub const HEIGHT: u8 = 8;
pub const WIDTH: u8 = 8;
