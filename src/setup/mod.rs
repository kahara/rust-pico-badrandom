use core::result::Result;
use core::result::Result::Ok;

use embedded_time::rate::*;
use embedded_time::rate::Extensions;

use rp2040_hal::{
    clocks::{
        Clock,
        ClocksManager,
        InitError
    },
    pll::{
        PLLConfig,
        common_configs::PLL_USB_48MHZ,
        setup_pll_blocking
    },
    watchdog::Watchdog,
    xosc::setup_xosc_blocking,
    Sio,
};
use rp_pico::pac;

// 10ns per clock cycle for easier mentals
pub const PLL_SYS_100MHZ: PLLConfig<Megahertz> = PLLConfig {
    vco_freq: Megahertz(1500),
    refdiv: 1,
    post_div1: 5,
    post_div2: 3,
};

pub fn clocks() -> Result<ClocksManager, InitError> {
    let mut pac = unsafe { pac::Peripherals::steal() };

    let xosc = setup_xosc_blocking(pac.XOSC,
                                   rp_pico::XOSC_CRYSTAL_FREQ.Hz()).map_err(InitError::XoscErr)?;

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    watchdog.enable_tick_generation((rp_pico::XOSC_CRYSTAL_FREQ / 1_000_000) as u8);

    let mut clocks = ClocksManager::new(pac.CLOCKS);

    // PLLs
    let pll_sys = setup_pll_blocking(pac.PLL_SYS,
                                     xosc.operating_frequency().into(),
                                     PLL_SYS_100MHZ,
                                     &mut clocks,
                                     &mut pac.RESETS)
        .map_err(InitError::PllError)?;
    let pll_usb = setup_pll_blocking(pac.PLL_USB,
                                     xosc.operating_frequency().into(),
                                     PLL_USB_48MHZ,
                                     &mut clocks,
                                     &mut pac.RESETS)
        .map_err(InitError::PllError)?;

    clocks
        .init_default(&xosc, &pll_sys, &pll_usb)
        .map_err(InitError::ClockError)?;

    Ok(clocks)
}

pub fn delay(cm: ClocksManager) -> cortex_m::delay::Delay {
    let core = unsafe { pac::CorePeripherals::steal() };
    cortex_m::delay::Delay::new(core.SYST, cm.system_clock.freq().integer())
}

pub fn pins(mut pac: pac::Peripherals) -> rp_pico::Pins {
    let sio = Sio::new(pac.SIO);

    rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    )
}
