#![no_std]
#![no_main]

extern crate rust_pico_badrandom;

// The macro for our start-up function
use cortex_m_rt::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

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
