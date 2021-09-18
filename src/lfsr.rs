#[derive(Debug, Copy, Clone)]
pub struct Lfsr {
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
