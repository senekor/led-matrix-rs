//! This library provides a simple, user-friendly API to program the LED matrix.
//!
//! The heart of this Library is the [LedMatrix] trait, which provides the API
//! and at the same time abstracts over the physical LED matrix itself as well
//! as the TUI emulator. Call the function [init] to acquire an object which
//! implements this trait.

#![no_std]

pub use led_matrix_core::LedMatrix;

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
