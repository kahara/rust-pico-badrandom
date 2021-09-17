use std::io::{self, Write};

#[derive(Debug, Copy, Clone)]
struct Lfsr {
    start: u32,
}

impl Lfsr {
    pub fn new() -> Self {
        Lfsr { start: 0xa5a5a5a5 }
    }
}

impl Iterator for Lfsr {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let bit = ((self.start >> 1) ^ (self.start >> 4)) & 1;

        self.start = (self.start >> 1) | (bit << 31);
        Some((bit & 0x1) != 0)
    }
}

fn main() -> io::Result<()> {
    let lfsr = Lfsr::new();
    let mut byte: u8 = 0x0;
    let mut bit: u8 = 0x0;
    let mut buffer: [u8; 1] = [0];
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for x in lfsr {
        byte = byte | ((x as u8) << bit);
        if bit == 7 {
            buffer[0] = byte; // FIXME: do without?
            handle.write(&buffer)?;
            byte = 0;
            bit = 0;
        } else {
            bit = bit + 1;
        }
    }

    Ok(())
}
