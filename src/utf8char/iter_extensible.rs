use super::Utf8Char;
use std::mem::transmute;

/// An iterator over the UTF-8 characters of a byte slice.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct IterUtf8Char<'a> {
    bytes: &'a [u8],
}

impl<'a> IterUtf8Char<'a> {
    /// Create a new `IterUtf8Char` from a string slice.
    pub const fn new(bytes: &'a str) -> Self {
        let bytes = bytes.as_bytes();
        Self { bytes }
    }
    /// Create a new `IterUtf8Char` from a byte slice.
    pub const unsafe fn new_from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl<'a> Iterator for IterUtf8Char<'a> {
    type Item = &'a Utf8Char;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.bytes.get(0)?;
        let l = unsafe { super::utf8_len_from_first_byte(*byte) };
        let c = unsafe { transmute(self.bytes.get(0..l)?) };
        self.bytes = &self.bytes[l..];
        Some(c)
    }
}
