# led-matrix-rs

ZHAW [LED-Matrix-Workshop] spin-off using Rust instead of Python.

Based on [rp-pico] and more specifically its [pico_ws2812_led] example.

# Usage

1. [install the Rust toolchain](https://www.rust-lang.org/tools/install)
1. `cargo install elf2uf2-rs`
1. connect the LED matrix while keeping BOOTSEL pressed
1. `cargo run --release --example $EXAMPLE`


[LED-Matrix-Workshop]: https://github.com/InES-HPMM/LED-Matrix-Workshop/tree/main
[rp-pico]: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico
[pico_ws2812_led]: https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_ws2812_led.rs
