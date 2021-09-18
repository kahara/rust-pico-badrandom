use std::io::{self, Write};

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

fn main() -> io::Result<()> {
    let lfsr = Lfsr::new();
    let mut byte: u8 = 0x0;
    let mut bit: u8 = 0x0;
    let mut buffer: [u8; 1] = [0];
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for x in lfsr {
        byte = byte | ((x as u8) << bit);
        if bit == 7 {
            buffer[0] = byte; // FIXME: do without?
            out.write(&buffer)?;
            byte = 0;
            bit = 0;
        } else {
            bit = bit + 1;
        }
    }

    Ok(())
}
