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
    pac,
    pll::{
        PLLConfig,
        common_configs::PLL_USB_48MHZ,
        setup_pll_blocking
    },
    watchdog::Watchdog,
    xosc::setup_xosc_blocking,
    Sio,
};

// 10ns per clock cycle for easier mentals
pub const PLL_SYS_100MHZ: PLLConfig<Megahertz> = PLLConfig {
    vco_freq: Megahertz(1500),
    refdiv: 1,
    post_div1: 5,
    post_div2: 3,
};

pub fn clocks() -> Result<ClocksManager, InitError> {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);

    watchdog.enable_tick_generation(rp_pico::XOSC_CRYSTAL_FREQ as u8);

    let mut clocks = ClocksManager::new(peripherals.CLOCKS);
    let xosc = setup_xosc_blocking(peripherals.XOSC,
                                   rp_pico::XOSC_CRYSTAL_FREQ.Hz()).map_err(InitError::XoscErr)?;

    // PLLs
    let pll_sys = setup_pll_blocking(peripherals.PLL_SYS,
                                     xosc.operating_frequency().into(),
                                     PLL_SYS_100MHZ,
                                     &mut clocks,
                                     &mut peripherals.RESETS)
        .map_err(InitError::PllError)?;
    let pll_usb = setup_pll_blocking(peripherals.PLL_USB,
                                     xosc.operating_frequency().into(),
                                     PLL_USB_48MHZ,
                                     &mut clocks,
                                     &mut peripherals.RESETS)
        .map_err(InitError::PllError)?;

    clocks
        .init_default(&xosc, &pll_sys, &pll_usb)
        .map_err(InitError::ClockError)?;

    Ok(clocks)
}

pub fn delay(cm: ClocksManager) -> cortex_m::delay::Delay {
    let core = pac::CorePeripherals::take().unwrap();
    cortex_m::delay::Delay::new(core.SYST, cm.system_clock.freq().integer())
}

pub fn pins() -> rp_pico::Pins {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    )
}
