#![no_std]

pub use zhaw_led_matrix_core::LedMatrix;

pub fn init() -> impl LedMatrix {
    #[cfg(target_os = "none")]
    {
        zhaw_led_matrix_bsp::LedMatrix::take().unwrap()
    }
    #[cfg(not(target_os = "none"))]
    {
        zhaw_led_matrix_emulator::LedMatrix::new()
    }
}

#[macro_export]
macro_rules! entry {
    ($main: ident) => {
        #[cfg(target_os = "none")]
        {
            use panic_halt as _;

            #[rp_pico::entry]
            fn zhaw_led_matrix_entry() {
                $main();
            }
        }
        #[cfg(not(target_os = "none"))]
        {
            fn main() {
                $main();
            }
        }
    };
}
