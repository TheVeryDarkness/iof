use super::utf8_len_from_first_byte;
use std::mem::transmute;

/// A UTF-8 character that is fixed in size.
///
/// 10xxxxxx: continuation byte
///
/// - `00000000..=01111111`: 1 byte
/// - `11000000..=11011111`: 2 bytes
/// - `11100000..=11101111`: 3 bytes
/// - `11110000..=11110111`: 4 bytes
/// - `11111000..=11111011`: 5 bytes (not valid)
/// - `11111100..=11111101`: 6 bytes (not valid)
///
/// # Invariants
///
/// - The byte slice is a single valid UTF-8 character.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Utf8Char {
    bytes: [u8],
}

impl AsRef<[u8]> for Utf8Char {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Utf8Char {
    fn as_ref(&self) -> &str {
        let bytes = self.as_ref();
        debug_assert!(std::str::from_utf8(bytes).is_ok());
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
}

impl Utf8Char {
    /// Create a new `Utf8Char` from a byte array.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check if the byte array is a valid UTF-8 character.
    pub const unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        debug_assert!(std::str::from_utf8(bytes).is_ok());
        debug_assert!(!bytes.is_empty());
        debug_assert!(utf8_len_from_first_byte(bytes[0]) == bytes.len());
        transmute(bytes)
    }
    /// Get the length in bytes of the UTF-8 character.
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }
    /// Check if the UTF-8 character is empty.
    ///
    /// This function always returns `false`.
    pub const fn is_empty(&self) -> bool {
        false
    }
    /// Get the bytes of the UTF-8 character.
    pub const fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Get the string of the UTF-8 character.
    pub const fn as_str(&self) -> &str {
        let bytes = self.as_bytes();
        debug_assert!(std::str::from_utf8(bytes).is_ok());
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
    /// Get first character of the UTF-8 string.
    ///
    /// Returns `None` if the string is empty.
    pub fn from_first_char(s: &str) -> Option<&Self> {
        let byte = s.as_bytes().first()?;
        let l = unsafe { utf8_len_from_first_byte(*byte) };
        let bytes: &Self = unsafe { transmute(s.as_bytes().get(0..l)?) };
        Some(bytes)
    }
}

impl PartialEq<char> for Utf8Char {
    fn eq(&self, other: &char) -> bool {
        let mut bytes = [0; 4];
        let _ = other.encode_utf8(&mut bytes);
        self.bytes == bytes[0..other.len_utf8()]
    }
}
impl PartialEq<Utf8Char> for char {
    fn eq(&self, other: &Utf8Char) -> bool {
        <Utf8Char as PartialEq<char>>::eq(other, self)
    }
}

impl PartialEq<char> for &Utf8Char {
    fn eq(&self, other: &char) -> bool {
        <Utf8Char as PartialEq<char>>::eq(self, other)
    }
}
impl PartialEq<&Utf8Char> for char {
    fn eq(&self, other: &&Utf8Char) -> bool {
        <Utf8Char as PartialEq<char>>::eq(other, self)
    }
}
