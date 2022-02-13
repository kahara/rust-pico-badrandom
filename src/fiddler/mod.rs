use rp2040_hal::{
    clocks::ClocksManager,
};
use embedded_hal::digital::v2::OutputPin;

use super::lfsr::Lfsr;
use super::setup::{delay, pins};

pub struct Fiddler {
    cm: ClocksManager,
    halfcycle: u32,
}

impl Fiddler {
    pub fn new(cm: ClocksManager, halfcycle: u32) -> Self {
        Fiddler {
            cm: cm,
            halfcycle: halfcycle,
        }
    }

    pub fn fiddle(self: Fiddler) -> ! {
        let pins = pins();

        // toggle LED and pin 31
        let mut pin_25 = pins.led.into_push_pull_output();
        let mut pin_26 = pins.gpio26.into_push_pull_output();
        // pins 32, 34 output a pseudorandom bit
        let mut pin_27 = pins.gpio27.into_push_pull_output();
        let mut pin_28 = pins.gpio28.into_push_pull_output();

        let mut lfsr_27 = Lfsr::new(0b0101010101010101);
        let mut lfsr_28 = Lfsr::new(0b1010101010101010);

        let mut delay = delay(self.cm);

        loop {
            // first half
            delay.delay_ms(self.halfcycle);

            pin_25.set_high().unwrap();
            pin_26.set_high().unwrap();

            if lfsr_27.next() { pin_27.set_high().unwrap();
            } else { pin_27.set_low().unwrap(); }
            if lfsr_28.next() { pin_28.set_high().unwrap();
            } else { pin_28.set_low().unwrap(); }

            // second half
            delay.delay_ms(self.halfcycle);

            pin_25.set_low().unwrap();
            pin_26.set_low().unwrap();

            if lfsr_27.next() { pin_27.set_high().unwrap();
            } else { pin_27.set_low().unwrap(); }
            if lfsr_28.next() { pin_28.set_high().unwrap();
            } else { pin_28.set_low().unwrap(); }
        }
    }
}
