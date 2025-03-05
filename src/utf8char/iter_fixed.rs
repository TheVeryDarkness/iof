use super::FixedUtf8Char;
use std::str::from_utf8_unchecked;

/// An iterator over fixed-size UTF-8 characters.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct IterFixedUtf8Char<'a> {
    bytes: &'a [u8],
}

impl<'a> IterFixedUtf8Char<'a> {
    /// Create a new `IterFixedUtf8Char` from a string slice.
    #[inline]
    pub const fn new(bytes: &'a str) -> Self {
        let bytes = bytes.as_bytes();
        Self { bytes }
    }
    /// Create a new `IterFixedUtf8Char` from a byte slice.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check if the byte slice is a valid UTF-8 string.
    #[inline]
    pub const unsafe fn new_from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl Iterator for IterFixedUtf8Char<'_> {
    type Item = FixedUtf8Char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        FixedUtf8Char::from_first_char(unsafe { from_utf8_unchecked(self.bytes) })
            .inspect(|c| self.bytes = &self.bytes[c.len_utf8()..])
    }
}

impl DoubleEndedIterator for IterFixedUtf8Char<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        FixedUtf8Char::from_last_char(unsafe { from_utf8_unchecked(self.bytes) })
            .inspect(|c| self.bytes = &self.bytes[..c.len_utf8()])
    }
}
