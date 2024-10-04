#![no_std]

use embedded_hal::digital::InputPin;
use rp_pico::hal::{
    self,
    gpio::{
        bank0::{Gpio19, Gpio2, Gpio3, Gpio6, Gpio7, Gpio8, Gpio9},
        FunctionPio0, FunctionSio, Pin, PullDown, PullUp, SioInput,
    },
    pac::{self, PIO0},
    pio::{PIOExt, SM0},
    timer::{CountDown, Timer},
    Clock,
};

// Import useful traits to handle the ws2812 LEDs:
use smart_leds::{brightness, SmartLedsWrite};

// Import the actual crate to handle the Ws2812 protocol:
use ws2812_pio::Ws2812;

type Values = [[(u8, u8, u8); 8]; 8];

/// A high-level wrapper around peripherals and LED libraries
/// to program the LED matrix easily.
///
/// An `LedMatrix` may be indexed using an integer in the range `0..64`
/// or a tuple of integers (row, column) in the range `0..8` each.
///
/// The LEDs are set at a default brightness of about 1/4.
/// Use `set_brightness` to change it.
pub struct LedMatrix {
    ws: Ws2812<PIO0, SM0, CountDown<'static>, Pin<Gpio19, FunctionPio0, PullDown>>,
    frame_delay: cortex_m::delay::Delay,

    joystick_up: Pin<Gpio3, FunctionSio<SioInput>, PullUp>,
    joystick_down: Pin<Gpio6, FunctionSio<SioInput>, PullUp>,
    joystick_left: Pin<Gpio7, FunctionSio<SioInput>, PullUp>,
    joystick_right: Pin<Gpio2, FunctionSio<SioInput>, PullUp>,

    // TODO implement API
    joystick_center: Pin<Gpio8, FunctionSio<SioInput>, PullUp>,
    switch: Pin<Gpio9, FunctionSio<SioInput>, PullUp>,

    values: Values,

    // Bring down the overall brightness of the strip to not blow
    // the USB power supply: every LED draws ~60mA, RGB means 3 LEDs per
    // ws2812 LED, for 3 LEDs that would be: 3 * 3 * 60mA, which is
    // already 540mA for just 3 white LEDs!
    //
    // default: 64 (or around 1/4 brightness)
    brightness: u8,
}

static mut TIMER: Option<Timer> = None;

impl LedMatrix {
    /// Returns the LED matrix _once_.
    ///
    /// Takes ownership of all hardware necessary to run the LED Matrix.
    /// Returns `None` if called more than once, or the underlying hardware
    /// is already taken.
    pub fn take() -> Option<Self> {
        // This function corresponds closely to the initilization code of the
        // example from the rp_pico repository.

        // Grab our singleton objects
        let mut pac = pac::Peripherals::take()?;
        let core = pac::CorePeripherals::take()?;

        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        //
        // The default is to generate a 125 MHz system clock
        let clocks = hal::clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();

        // The single-cycle I/O block controls our GPIO pins
        let sio = hal::Sio::new(pac.SIO);

        // Set the pins up according to their function on this particular board
        let pins = rp_pico::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        // "pull up input" copied from Python version.
        // what is the difference to "pull down" ??
        let joystick_up = pins.gpio3.into_pull_up_input();
        let joystick_down = pins.gpio6.into_pull_up_input();
        let joystick_left = pins.gpio7.into_pull_up_input();
        let joystick_right = pins.gpio2.into_pull_up_input();
        let joystick_center = pins.gpio8.into_pull_up_input();

        let switch = pins.gpio9.into_pull_up_input();

        // Setup a delay for the LED blink signals:
        let frame_delay =
            cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        // Create a count down timer for the Ws2812 instance:
        let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
        let count_down = unsafe {
            TIMER = Some(timer);
            TIMER.as_ref().unwrap().count_down()
        };

        // Split the PIO state machine 0 into individual objects, so that
        // Ws2812 can use it:
        let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

        // Instanciate a Ws2812 LED strip:
        let ws = Ws2812::new(
            // Use pin 25 on the Raspberry Pi Pico (which is GPIO19 of the rp2040 chip)
            // for the LED data output:
            pins.gpio19.into_function(),
            &mut pio,
            sm0,
            clocks.peripheral_clock.freq(),
            count_down,
        );

        Some(Self {
            ws,
            frame_delay,
            joystick_up,
            joystick_down,
            joystick_left,
            joystick_right,
            joystick_center,
            switch,
            values: Values::default(),
            brightness: 64, // default brightness of about 0.25
        })
    }
}

impl led_matrix_core::LedMatrix for LedMatrix {
    fn set_brighness(&mut self, brightness: f32) {
        assert!((0.0..=1.0).contains(&brightness));
        self.brightness = (brightness * 255.0) as u8
    }

    fn update(&mut self, mut f: impl FnMut(usize, usize, &mut (u8, u8, u8))) {
        for (i, row) in self.values.iter_mut().enumerate() {
            for (j, led) in row.iter_mut().enumerate() {
                f(i, j, led)
            }
        }
        self.ws
            .write(brightness(
                self.values
                    .iter()
                    .flat_map(|row| row.iter())
                    .map(|&c| c.into()),
                self.brightness,
            ))
            .unwrap();
        self.frame_delay.delay_ms(16)
    }

    fn get_sin(&self) -> fn(f32) -> f32 {
        |x| hal::rom_data::float_funcs::fsin::ptr()(x)
    }

    fn joystick_is_up(&mut self) -> bool {
        self.joystick_up.is_low().unwrap()
    }
    fn joystick_is_down(&mut self) -> bool {
        self.joystick_down.is_low().unwrap()
    }
    fn joystick_is_left(&mut self) -> bool {
        self.joystick_left.is_low().unwrap()
    }
    fn joystick_is_right(&mut self) -> bool {
        self.joystick_right.is_low().unwrap()
    }
}

impl core::ops::Index<usize> for LedMatrix {
    type Output = (u8, u8, u8);

    fn index(&self, index: usize) -> &Self::Output {
        assert!((0..64).contains(&index));
        &self.values[index % 8][index / 8]
    }
}
impl core::ops::Index<(usize, usize)> for LedMatrix {
    type Output = (u8, u8, u8);

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!((0..8).contains(&row));
        assert!((0..8).contains(&col));
        &self.values[row][col]
    }
}
impl core::ops::IndexMut<usize> for LedMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!((0..64).contains(&index));
        &mut self.values[index % 8][index / 8]
    }
}
impl core::ops::IndexMut<(usize, usize)> for LedMatrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        assert!((0..8).contains(&row));
        assert!((0..8).contains(&col));
        &mut self[(row, col)]
    }
}

pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    let c = val * sat;
    let v = (hue / 60.0) % 2.0 - 1.0;
    let v = if v < 0.0 { -v } else { v };
    let x = c * (1.0 - v);
    let m = val - c;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

pub fn hsv2rgb_u8(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let r = hsv2rgb(h, s, v);

    (
        (r.0 * 255.0) as u8,
        (r.1 * 255.0) as u8,
        (r.2 * 255.0) as u8,
    )
}
