//! UTF-8 character types.
//!
//! This module provides two types for representing UTF-8 characters: `Utf8Char` and `FixedUtf8Char`.
//!
//! `Utf8Char` is a dynamically-sized type that can represent any valid UTF-8 character. It is
//! implemented as a wrapper around a byte slice.
//!
//! `FixedUtf8Char` is a fixed-size type that can represent any valid UTF-8 character. It is
//! implemented as a wrapper around a fixed-size byte array.

pub use extensible::Utf8Char;
pub use fixed::FixedUtf8Char;
pub use iter_extensible::IterUtf8Char;
pub use iter_fixed::IterFixedUtf8Char;

mod extensible;
mod fixed;
pub(super) mod iter_extensible;
pub(super) mod iter_fixed;
#[cfg(test)]
mod tests;

/// Get the length in bytes of a UTF-8 character from its first byte.
///
/// # Safety
///
/// This function is unsafe because it does not check if the byte is a valid first byte of a UTF-8.
///
/// # Note
///
/// See [`std::str::Chars`] and [`std::str::Chars::next_back`] for more information.
#[inline]
const unsafe fn utf8_len_from_first_byte(byte: u8) -> usize {
    debug_assert!(matches!(byte, 0..=0x7F | 0xC0..=0xDF | 0xE0..=0xEF | 0xF0..=0xF7));
    match byte {
        0..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        _ => 4,
    }
}

/// Check if a byte is a UTF-8 continuation byte.
///
/// # Safety
///
/// This function is unsafe because it does not check if the byte is a valid UTF-8 continuation byte.
///
/// # Note
///
/// See [`std::str::Chars`] and [`std::str::Chars::next_back`] for more information.
#[inline]
const unsafe fn is_utf8_continuation_byte(byte: u8) -> bool {
    byte & 0xC0 == 0x80
}

impl PartialEq<FixedUtf8Char> for Utf8Char {
    #[inline]
    fn eq(&self, other: &FixedUtf8Char) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<Utf8Char> for FixedUtf8Char {
    #[inline]
    fn eq(&self, other: &Utf8Char) -> bool {
        <Utf8Char as PartialEq<FixedUtf8Char>>::eq(other, self)
    }
}

impl PartialEq<FixedUtf8Char> for &Utf8Char {
    #[inline]
    fn eq(&self, other: &FixedUtf8Char) -> bool {
        <Utf8Char as PartialEq<FixedUtf8Char>>::eq(self, other)
    }
}

impl PartialEq<&Utf8Char> for FixedUtf8Char {
    #[inline]
    fn eq(&self, other: &&Utf8Char) -> bool {
        <Utf8Char as PartialEq<FixedUtf8Char>>::eq(other, self)
    }
}
