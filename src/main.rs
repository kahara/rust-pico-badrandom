#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::rate::*;
use panic_halt as _;

use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::clocks::Clock;
use rp2040_hal::pac;
use rp2040_hal::sio::Sio;
use rp2040_hal::watchdog::Watchdog;

//    Pins, XOSC_CRYSTAL_FREQ,

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[derive(Debug, Copy, Clone)]
struct Lfsr {
    start: u16,
}

impl Lfsr {
    pub fn new() -> Self {
        Lfsr { start: 0xffff }
    }
}

impl Iterator for Lfsr {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let bit =
            ((self.start >> 0) ^ (self.start >> 2) ^ (self.start >> 3) ^ (self.start >> 5)) & 1;

        self.start = (self.start >> 1) | (bit << 15);
        Some((bit & 0x1) != 0)
    }
}

#[entry]
fn main() -> ! {
    let lfsr = Lfsr::new();
    let mut byte: u8 = 0x0;
    let mut bit: u8 = 0x0;

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
        .ok()
        .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    for x in lfsr {

    }

    loop { cortex_m::asm::nop(); }
}
