#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

extern crate rust_pico_badrandom;
extern crate rp_pico;
extern crate rp2040_hal;

use rust_pico_badrandom::setup;
use rust_pico_badrandom::fiddler::*;

/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
#[entry]
fn main() -> ! {
    match setup::clocks() {
        Ok(clocks) => {
            let fiddler = Fiddler::new(clocks, 1000);
            fiddler.fiddle()
        },
        Err(_e) => {
            loop {}
        }
    }
}
