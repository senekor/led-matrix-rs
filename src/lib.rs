//! This library provides a simple, user-friendly API to program the LED matrix.
//!
//! The heart of this Library is the [LedMatrix] trait, which provides the API
//! and at the same time abstracts over the physical LED matrix itself as well
//! as the TUI emulator. Call the function [init] to acquire an object which
//! implements this trait.

#![no_std]

pub use led_matrix_core::JoystickPosition;

use led_matrix_core::{LedMatrixCore, HEIGHT, WIDTH};

/// A high-level interface for programming the LED-matrix.
///
/// To update the color of an LED, you can index the `LedMatrix` with a tuple
/// of integers (row, column) in the range `0..8` each.
///
/// After changing the values of one or several LEDs, don't forget to call
/// `apply` to actually apply these changes in a batch.
///
pub trait LedMatrix: LedMatrixCore {
    /// Tell the LED matrix to display the currently stored color values for
    /// each LED.
    ///
    /// If you are drawing in an endless loop, consider calling
    /// [Self::sleep_ms] at some point to slow down the execution.
    ///
    fn apply(&mut self) {
        <Self as LedMatrixCore>::apply(self)
    }

    /// Set the brightness of the display.
    ///
    /// The display is set at a default brightness of `50` (about 20%).
    ///
    /// This method is a no-op for the TUI emulator.
    ///
    fn set_brightness(&mut self, brightness: u8) {
        <Self as LedMatrixCore>::set_brightness(self, brightness)
    }

    /// Sleep for the specified amount of milliseconds.
    ///
    fn sleep_ms(&mut self, duration: u32) {
        <Self as LedMatrixCore>::sleep_ms(self, duration)
    }

    /// Get a sinus function.
    ///
    /// This is necessary to abstract over hardware and emulator.
    ///
    fn get_sin(&self) -> fn(f32) -> f32 {
        <Self as LedMatrixCore>::get_sin(self)
    }

    /// Get the current joystick position.
    ///
    fn joystick_position(&mut self) -> JoystickPosition {
        <Self as LedMatrixCore>::joystick_position(self)
    }

    /// Set every LED to a single color at the same time.
    ///
    /// You still need to call [draw] afterwards.
    ///
    fn fill(&mut self, color: (u8, u8, u8)) {
        for row in 0..HEIGHT as usize {
            for column in 0..WIDTH as usize {
                self[(row, column)] = color;
            }
        }
    }

    /// Turn off all LEDs.
    ///
    fn clear(&mut self) {
        self.fill((0, 0, 0));
    }

    /// Set a list of LEDs to a single color.
    ///
    /// LEDs are specified as a slice of coordinates (row, column).
    ///
    fn draw_list<T: AsRef<[(usize, usize)]>>(&mut self, list: T, color: (u8, u8, u8)) {
        for (row, column) in list.as_ref() {
            self[(*row, *column)] = color;
        }
    }

    // TODO: draw_line

    /// Draw a bitmap file with a color depth of 24 bit.
    ///
    /// bitmap format: https://en.wikipedia.org/wiki/BMP_file_format
    ///
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
impl<T: LedMatrixCore> LedMatrix for T {}

pub fn init() -> impl LedMatrix {
    #[cfg(target_os = "none")]
    {
        led_matrix_bsp::LedMatrix::take().unwrap()
    }
    #[cfg(not(target_os = "none"))]
    {
        led_matrix_tui::LedMatrix::new()
    }
}

/// Contains a number of predefined color values.
///
/// You can set an LED to one of these colors like this:
///
/// ```
/// *matrix.get_mut(2, 6) = color::PURPLE;
/// ```
pub mod color {
    pub const YELLOW: (u8, u8, u8) = (255, 255, 0);
    pub const ORANGE: (u8, u8, u8) = (255, 165, 0);
    pub const RED: (u8, u8, u8) = (255, 0, 0);
    pub const PURPLE: (u8, u8, u8) = (128, 0, 128);
    pub const PINK: (u8, u8, u8) = (255, 0, 255);
    pub const BLUE: (u8, u8, u8) = (0, 0, 255);
    pub const TEAL: (u8, u8, u8) = (0, 128, 128);
    pub const AQUA: (u8, u8, u8) = (0, 255, 255);
    pub const LIME: (u8, u8, u8) = (0, 255, 0);
    pub const GREEN: (u8, u8, u8) = (0, 128, 0);
    pub const LIGHT_GREY: (u8, u8, u8) = (119, 136, 153);
    pub const GREY: (u8, u8, u8) = (100, 100, 100);
    pub const BROWN: (u8, u8, u8) = (139, 69, 19);
    pub const LIGHT_BROWN: (u8, u8, u8) = (205, 133, 63);
    pub const WHITE: (u8, u8, u8) = (255, 255, 255);
    pub const BLACK: (u8, u8, u8) = (0, 0, 0);
}

/// Contains a number of predefined bitmaps.
pub mod bitmap {
    pub static APPLE: &[u8] = include_bytes!("../bitmaps/apple.bmp");
    pub static BAT: &[u8] = include_bytes!("../bitmaps/bat.bmp");
    pub static BIG_IMG: &[u8] = include_bytes!("../bitmaps/big_img.bmp");
    pub static BIRD: &[u8] = include_bytes!("../bitmaps/bird.bmp");
    pub static CHICKEN: &[u8] = include_bytes!("../bitmaps/chicken.bmp");
    pub static CRAB: &[u8] = include_bytes!("../bitmaps/crab.bmp");
    pub static DINO: &[u8] = include_bytes!("../bitmaps/dino.bmp");
    pub static DRINK: &[u8] = include_bytes!("../bitmaps/drink.bmp");
    pub static DUCK: &[u8] = include_bytes!("../bitmaps/duck.bmp");
    pub static ERLENMEYER: &[u8] = include_bytes!("../bitmaps/erlenmeyer.bmp");
    pub static FOX: &[u8] = include_bytes!("../bitmaps/fox.bmp");
    pub static MC_CREEPER: &[u8] = include_bytes!("../bitmaps/mc_creeper.bmp");
    pub static MC_PIG: &[u8] = include_bytes!("../bitmaps/mc_pig.bmp");
    pub static MOB_1: &[u8] = include_bytes!("../bitmaps/mob_1.bmp");
    pub static MOUSE: &[u8] = include_bytes!("../bitmaps/mouse.bmp");
    pub static MUSHROOM: &[u8] = include_bytes!("../bitmaps/mushroom.bmp");
    pub static PIKACHU: &[u8] = include_bytes!("../bitmaps/pikachu.bmp");
    pub static RABBIT: &[u8] = include_bytes!("../bitmaps/rabbit.bmp");
    pub static SKULL: &[u8] = include_bytes!("../bitmaps/skull.bmp");
    pub static TEST_GRID: &[u8] = include_bytes!("../bitmaps/test_grid.bmp");
    pub static WINE: &[u8] = include_bytes!("../bitmaps/wine.bmp");
}

/// Returns an iterator over the coordinates of all LEDs. Useful for avoiding
/// nested loops.
pub fn all_led_coordinates() -> impl Iterator<Item = (usize, usize)> {
    (0..HEIGHT as usize).flat_map(|row| (0..WIDTH as usize).map(move |column| (row, column)))
}
