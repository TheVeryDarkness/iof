use super::FixedUtf8Char;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IterFixedUtf8Char<'a> {
    bytes: &'a [u8],
}

impl<'a> IterFixedUtf8Char<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl<'a> Iterator for IterFixedUtf8Char<'a> {
    type Item = FixedUtf8Char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }
        let (l, c) = match self.bytes[0] {
            0..=0x7F => (1, unsafe {
                FixedUtf8Char::from_bytes_unchecked([self.bytes[0], 0, 0, 0])
            }),
            0xC0..=0xDF => (2, unsafe {
                FixedUtf8Char::from_bytes_unchecked([self.bytes[0], self.bytes[1], 0, 0])
            }),
            0xE0..=0xEF => (3, unsafe {
                FixedUtf8Char::from_bytes_unchecked([
                    self.bytes[0],
                    self.bytes[1],
                    self.bytes[2],
                    0,
                ])
            }),
            _ => (4, unsafe {
                FixedUtf8Char::from_bytes_unchecked([
                    self.bytes[0],
                    self.bytes[1],
                    self.bytes[2],
                    self.bytes[3],
                ])
            }),
        };
        self.bytes = &self.bytes[l..];
        Some(c)
    }
}
