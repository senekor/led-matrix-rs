# led-matrix-rs

ZHAW [LED-Matrix-Workshop] spin-off using Rust instead of Python.

It provides a TUI emulator for easy debuggability.

# Usage

Setup:
1. [install the Rust toolchain](https://www.rust-lang.org/tools/install)
1. add the cross-compilation target:
   `rustup target add thumbv6m-none-eabi`
1. needed to convert the firmware to a flashable format: `cargo install elf2uf2-rs`

To run the emulator:
1. `cargo run --example $EXAMPLE`

To run on hardware:
1. connect the LED matrix while keeping BOOTSEL pressed
1. `RUSTUP_TARGET=thumbv6m-none-eabi cargo run --release --example $EXAMPLE`

<!-- TODO
    instructions for importing the library in your own Rust project.
    provide cargo-generate template?
-->

# Contributing

The project is split into four crates:
- `zhaw-led-matrix`: the main library users interact with
- `zhaw-led-matrix-bsp`: provides a user-friendly wrapper around hardware stuff
- `zhaw-led-matrix-emulator`: emulates the LED matrix in a terminal
- `zhaw-led-matrix-core`: defines a trait `LedMatrix`, implemented by both `bsp` and `emulator`

Much of the hardware code is based on the [pico_ws2812_led] example of [rp-pico].


[LED-Matrix-Workshop]: https://github.com/InES-HPMM/LED-Matrix-Workshop/tree/main
[pico_ws2812_led]: https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_ws2812_led.rs
[rp-pico]: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico
