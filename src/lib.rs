//! This library provides a simple, user-friendly API to program the LED matrix.
//!
//! The heart of this Library is the [LedMatrix] trait, which provides the API
//! and at the same time abstracts over the physical LED matrix itself as well
//! as the TUI emulator. Call the function [init] to acquire an object which
//! implements this trait.

#![no_std]

pub use led_matrix_core::LedMatrix;
use led_matrix_core::{HEIGHT, WIDTH};

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
