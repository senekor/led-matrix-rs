//! This library provides a user-friendly API to program the LED-matrix.
//!
//! The heart of this Library is the [LedMatrix] trait, which provides the API
//! and at the same time abstracts over the physical LED-matrix itself as well
//! as the TUI emulator. Call the function [init] to acquire an object which
//! implements this trait.

#![no_std]

use core::cmp::Ordering;

pub use led_matrix_core::JoystickPosition;

use led_matrix_core::{LedMatrixCore, HEIGHT, WIDTH};

pub mod billboard;
pub mod character;

/// A high-level interface for programming the LED-matrix.
///
/// To update the color of an LED, you can index the `LedMatrix` with a tuple
/// of integers (x, y) in the range `0..8` each.
///
/// After changing the values of one or several LEDs, don't forget to call
/// [`apply`](LedMatrix::apply) to actually apply these changes in a batch.
///
/// Here is the coordinate system visualized:
///
/// ```txt
/// ╭─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────╮
/// │ 7,0 │ 7,1 │ 7,2 │ 7,3 │ 7,4 │ 7,5 │ 7,6 │ 7,7 │
/// ├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤
/// │ 6,0 │ 6,1 │ 6,2 │ 6,3 │ 6,4 │ 6,5 │ 6,6 │ 6,7 │
/// ├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤
/// │ 5,0 │ 5,1 │ 5,2 │ 5,3 │ 5,4 │ 5,5 │ 5,6 │ 5,7 │
/// ├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤
/// │ 4,0 │ 4,1 │ 4,2 │ 4,3 │ 4,4 │ 4,5 │ 4,6 │ 4,7 │
/// ├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤
/// │ 3,0 │ 3,1 │ 3,2 │ 3,3 │ 3,4 │ 3,5 │ 3,6 │ 3,7 │
/// ├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤
/// │ 2,0 │ 2,1 │ 2,2 │ 2,3 │ 2,4 │ 2,5 │ 2,6 │ 2,7 │
/// ├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤
/// │ 1,0 │ 1,1 │ 1,2 │ 1,3 │ 1,4 │ 1,5 │ 1,6 │ 1,7 │
/// ├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤
/// │ 0,0 │ 0,1 │ 0,2 │ 0,3 │ 0,4 │ 0,5 │ 0,6 │ 0,7 │
/// ╰─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────╯
/// ```
///
pub trait LedMatrix: LedMatrixCore {
    /// Tell the LED-matrix to display the currently stored color values for
    /// each LED.
    ///
    /// If you are drawing in an endless loop, consider calling
    /// [sleep_ms](Self::sleep_ms) at some point to slow down the execution.
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
    /// You still need to call [apply](Self::apply) afterwards.
    ///
    fn fill(&mut self, color: (u8, u8, u8)) {
        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                self[(x, y)] = color;
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
    /// LEDs are specified as an iterator of coordinates (x, y).
    ///
    fn draw_coordinates<T: IntoIterator<Item = (usize, usize)>>(
        &mut self,
        coords: T,
        color: (u8, u8, u8),
    ) {
        for (x, y) in coords {
            self[(x, y)] = color;
        }
    }

    // TODO: draw_line

    /// Draw a bitmap file with a color depth of 24 bit.
    ///
    /// bitmap format: <https://en.wikipedia.org/wiki/BMP_file_format>
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
        for x in 0..WIDTH as usize {
            for y in 0..HEIGHT as usize {
                let i = (y * WIDTH as usize + x) * 3;
                let y = HEIGHT as usize - y - 1;
                self[(x, y)] = (pic[i + 2], pic[i + 1], pic[i]);
            }
        }
    }

    /// Draw a frame of a horizontal billboard at a specified offset.
    ///
    /// Construct such a billboard with [`billboard::horizontal`].
    ///
    /// This function only draws a single frame, you probably want to
    /// loob over offsets and draw each frame with a desired delay using
    /// [sleep_ms](Self::sleep_ms).
    ///
    /// See the module documentation of [billboard] for more information.
    ///
    /// TODO: Example
    ///
    fn draw_horizontal_billboard_frame(&mut self, billboard: billboard::Billboard, offset: usize) {
        for (x, column) in (offset..offset + WIDTH as usize).enumerate() {
            for y in 0..HEIGHT as usize {
                self[(x, y)] = match billboard
                    .get(column)
                    .map(|col| col[HEIGHT as usize - y - 1])
                {
                    Some(true) => color::WHITE,
                    _ => color::BLACK,
                }
            }
        }
    }

    /// Draw a frame of a vertical billboard at a specified offset.
    ///
    /// Construct such a billboard with [`billboard::vertical`].
    ///
    /// This function is analogous to [draw_horizontal_billboard_frame](Self::draw_horizontal_billboard_frame).
    ///
    fn draw_vertical_billboard_frame(&mut self, billboard: billboard::Billboard, offset: usize) {
        for (y, row) in (offset..offset + HEIGHT as usize).enumerate() {
            let y = HEIGHT as usize - y - 1;
            for x in 0..WIDTH as usize {
                self[(x, y)] = match billboard.get(row).map(|row| row[x]) {
                    Some(true) => color::WHITE,
                    _ => color::BLACK,
                }
            }
        }
    }

    /// Draw a frame of a strip of text at a specified offset.
    ///
    /// Construct such a strip of text with [`character::convert_str`].
    ///
    /// Like [`draw_horizontal_billboard_frame`], this function only draws a
    /// single frame. You probably want to loob over offsets and draw each frame
    /// with a desired delay using [sleep_ms](Self::sleep_ms).
    ///
    fn draw_text_billboard_frame(
        &mut self,
        text: &[character::Character],
        frame_offset: usize, // colors=ColorTable.WHITE, delay_ms=50, direction="left"
    ) {
        // let length: usize = text.iter().map(|c| c.width).sum::<usize>() - 1;

        // TODO: custom color support
        // // use same color for all bitmaps if only one color is supplied
        // if not isinstance(colors, list):
        //     colors = [colors] * len(bitmaps)

        self.clear();

        for &c in text.iter() {
            if c.offset + c.width < frame_offset {
                continue;
            } else if frame_offset + WIDTH as usize - 1 < c.offset {
                break;
            }
            let coords = c.coordinates.iter().copied().filter_map(|(mut x, y)| {
                // remove out-of-bounds coordinates and apply offset
                match frame_offset.cmp(&c.offset) {
                    Ordering::Less => {
                        x += c.offset - frame_offset;
                        if x >= WIDTH as usize {
                            // Character is partially in frame, but this
                            // specific pixel is beyond the right border of
                            // the frame.
                            return None;
                        }
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        let offset_diff = frame_offset - c.offset;
                        if offset_diff > x {
                            // Character is partially in frame, but this
                            // specific pixel is beyond the left border of
                            // the frame.
                            return None;
                        }
                        x -= offset_diff;
                    }
                }
                Some((x, y))
            });

            self.draw_coordinates(coords, color::WHITE);
        }
    }
}
impl<T: LedMatrixCore> LedMatrix for T {}

/// Initializes and returns an [LedMatrix].
///
/// The implementation (hardware or emulator) is automatically chosen based on
/// the compilation target.
///
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
    (0..HEIGHT as usize).flat_map(|y| (0..WIDTH as usize).map(move |x| (x, y)))
}
