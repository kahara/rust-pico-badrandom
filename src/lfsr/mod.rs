#[derive(Debug, Copy, Clone)]
pub struct Lfsr {
    start: u16,
}

impl Lfsr {
    pub fn new(start: u16) -> Self {
        Lfsr { start: start }
    }
}

impl Lfsr {
    pub fn next(&mut self) -> bool {
        let bit =
            ((self.start >> 0) ^ (self.start >> 2) ^ (self.start >> 3) ^ (self.start >> 5)) & 1;

        self.start = (self.start >> 1) | (bit << 15);
        (bit & 0x1) != 0
    }
}
