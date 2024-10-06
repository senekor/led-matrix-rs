# led-matrix-rs

ZHAW [LED-Matrix-Workshop] spin-off using Rust instead of Python.

It provides a TUI emulator for local debugging.

The documentation is hosted [here](https://github.zhaw.ch/pages/senk/led-matrix-rs/led_matrix/).

## Usage

You can find examples to run in the directory `examples`.

Setup:
1. [install the Rust toolchain](https://www.rust-lang.org/tools/install)
1. add the cross-compilation target:
   `rustup target add thumbv6m-none-eabi`
1. needed to convert the firmware to a flashable format: `cargo install elf2uf2-rs`

To run the emulator:

```sh
cargo run --example $EXAMPLE
```

To run on hardware, first connect the LED-matrix while keeping BOOTSEL pressed, then:

```sh
cargo run --release --target thumbv6m-none-eabi --example $EXAMPLE
```

<!-- TODO
    instructions for importing the library in your own Rust project.
    provide cargo-generate template?
-->

To read the API documentation:
1. `cd zhaw-led-matrix`
1. `cargo doc --open`

## Contributing

There is a `justfile` for common development tasks.
For example, run `just check` to make sure everything compiles.

The project is split into four crates:
- `led-matrix`:
  The main library users interact with.
  Located in the root directory of this repository.
- `led-matrix-core`:
  Defines a trait `LedMatrix`, which defines the common capabilities of both hardware and TUI emulator.
  Located in `crates/led-matrix-core`.
- `led-matrix-bsp`:
  Implements the `LedMatrix` trait on actual hardware.
  Located in `crates/led-matrix-bsp`.
- `led-matrix-tui`:
  Implements the `LedMatrix` trait with a TUI emulator.
  Located in `crates/led-matrix-tui`.

Much of the hardware code is based on the [pico_ws2812_led] example of [rp-pico].


[LED-Matrix-Workshop]: https://github.com/InES-HPMM/LED-Matrix-Workshop/tree/main
[pico_ws2812_led]: https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_ws2812_led.rs
[rp-pico]: https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico
