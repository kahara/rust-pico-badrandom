#![no_std]
#![no_main]
#![feature(asm)]

use cortex_m_rt::entry;
use panic_probe as _;
use rp2040_pac as pac;

mod pll;
mod resets;
mod lfsr;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

fn init(
    resets: pac::RESETS,
    watchdog: pac::WATCHDOG,
    clocks: pac::CLOCKS,
    xosc: pac::XOSC,
    pll_sys: pac::PLL_SYS,
    pll_usb: pac::PLL_USB,
) {
    // Now reset all the peripherals, except QSPI and XIP (we're using those
    // to execute from external flash!)

    let resets = resets::Resets::new(resets);

    // Reset everything except:
    // - QSPI (we're using it to run this code!)
    // - PLLs (it may be suicide if that's what's clocking us)
    resets.reset(!(resets::IO_QSPI | resets::PADS_QSPI | resets::PLL_SYS | resets::PLL_USB));

    resets.unreset_wait(
        resets::ALL
            & !(resets::ADC
            | resets::RTC
            | resets::SPI0
            | resets::SPI1
            | resets::UART0
            | resets::UART1
            | resets::USBCTRL),
    );

    // xosc 12 mhz
    watchdog
        .tick
        .write(|w| unsafe { w.cycles().bits(XOSC_MHZ as u16).enable().set_bit() });

    clocks.clk_sys_resus_ctrl.write(|w| unsafe { w.bits(0) });

    // Enable XOSC
    // TODO extract to HAL module
    const XOSC_MHZ: u32 = 12;
    xosc.ctrl.write(|w| w.freq_range()._1_15mhz());
    let startup_delay = (((XOSC_MHZ * 1_000_000) / 1000) + 128) / 256;
    xosc.startup
        .write(|w| unsafe { w.delay().bits(startup_delay as u16) });
    xosc.ctrl
        .write(|w| w.freq_range()._1_15mhz().enable().enable());

    while !xosc.status.read().stable().bit_is_set() {}

    // Before we touch PLLs, switch sys and ref cleanly away from their aux sources.
    clocks.clk_sys_ctrl.modify(|_, w| w.src().clk_ref());
    while clocks.clk_sys_selected.read().bits() != 1 {}
    clocks.clk_ref_ctrl.modify(|_, w| w.src().rosc_clksrc_ph());
    while clocks.clk_ref_selected.read().bits() != 1 {}

    resets.reset(resets::PLL_SYS | resets::PLL_USB);
    resets.unreset_wait(resets::PLL_SYS | resets::PLL_USB);

    pll::PLL::new(pll_sys).configure(1, 888_000_000, 3, 1);
    pll::PLL::new(pll_usb).configure(1, 480_000_000, 5, 2);

    // Switch clk_sys to pll_sys
    clocks
        .clk_sys_ctrl
        .modify(|_, w| w.auxsrc().clksrc_pll_sys());
    clocks
        .clk_sys_ctrl
        .modify(|_, w| w.src().clksrc_clk_sys_aux());
    while clocks.clk_sys_selected.read().bits() != 2 {}
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    init(p.RESETS,p.WATCHDOG, p.CLOCKS, p.XOSC, p.PLL_SYS, p.PLL_USB);

    let output = 15;
    let led = 25;
    let lfsr = lfsr::Lfsr::new();

    p.IO_BANK0.gpio[output].gpio_ctrl.write(|w| {
        w.oeover().enable();
        w
    });
    p.IO_BANK0.gpio[led].gpio_ctrl.write(|w| {
        w.oeover().enable();
        w
    });

    for x in lfsr {
        p.IO_BANK0.gpio[output].gpio_ctrl.write(|w| {
            if x { w.outover().high() } else { w.outover().low()};
            w
        });
        p.IO_BANK0.gpio[led].gpio_ctrl.write(|w| {
            if x { w.outover().high() } else { w.outover().low()};
            w
        });
    }

    loop { cortex_m::asm::nop(); }
}
