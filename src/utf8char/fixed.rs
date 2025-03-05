use super::{is_utf8_continuation_byte, utf8_len_from_first_byte};
use std::fmt::Display;

/// A UTF-8 character that is fixed in size.
///
/// `10xxxxxx`: continuation byte
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
/// - The byte array is a single valid UTF-8 character.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FixedUtf8Char {
    bytes: [u8; 4],
}

impl AsRef<[u8]> for FixedUtf8Char {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for FixedUtf8Char {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<char> for FixedUtf8Char {
    #[inline]
    fn from(c: char) -> Self {
        let mut bytes = [0; 4];
        let _ = c.encode_utf8(&mut bytes);
        Self { bytes }
    }
}

impl FixedUtf8Char {
    /// Create a new `FixedUtf8Char` from a byte array.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check if the byte array is a valid UTF-8 character.
    #[inline]
    pub const unsafe fn from_bytes_unchecked(bytes: [u8; 4]) -> Self {
        debug_assert!(std::str::from_utf8(&bytes).is_ok());
        debug_assert!(bytes[0] > 0);
        debug_assert!(utf8_len_from_first_byte(bytes[0]) > 1 || bytes[1] == 0);
        debug_assert!(utf8_len_from_first_byte(bytes[0]) > 2 || bytes[2] == 0);
        debug_assert!(utf8_len_from_first_byte(bytes[0]) > 3 || bytes[3] == 0);
        Self { bytes }
    }
    /// Get the length in bytes of the UTF-8 character.
    #[inline]
    pub const fn len_utf8(&self) -> usize {
        unsafe { utf8_len_from_first_byte(self.bytes[0]) }
    }
    /// Get the bytes of the UTF-8 character.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[0..self.len_utf8()]
    }
    /// Get the string of the UTF-8 character.
    #[inline]
    pub fn as_str(&self) -> &str {
        let bytes = self.as_bytes();
        debug_assert!(std::str::from_utf8(bytes).is_ok());
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
    /// Get first character of the UTF-8 string.
    ///
    /// Returns `None` if the string is empty.
    #[inline]
    pub fn from_first_char(s: &str) -> Option<Self> {
        let mut bytes = [0; 4];
        let byte = s.as_bytes().first()?;
        let l = unsafe { utf8_len_from_first_byte(*byte) };
        bytes[0..l].copy_from_slice(s.as_bytes().get(0..l)?);
        Some(unsafe { Self::from_bytes_unchecked(bytes) })
        // Some(Self { bytes })
    }
    /// Get last character of the UTF-8 string.
    ///
    /// Returns `None` if the string is empty.
    #[inline]
    pub fn from_last_char(s: &str) -> Option<Self> {
        let mut bytes = [0; 4];
        let buf = s.as_bytes();
        let (last, mut b) = buf.split_last()?;
        if last.is_ascii() {
            bytes[0] = *last;
            return Some(Self { bytes });
        }
        let mut l = 1;
        while let Some((last, b_)) = b.split_last() {
            l += 1;
            b = b_;
            if !unsafe { is_utf8_continuation_byte(*last) } {
                break;
            }
        }
        bytes[0..l].copy_from_slice(&buf[b.len()..]);
        Some(unsafe { Self::from_bytes_unchecked(bytes) })
        // Some(Self { bytes })
    }
}

impl PartialEq<char> for FixedUtf8Char {
    #[inline]
    fn eq(&self, other: &char) -> bool {
        let mut bytes = [0; 4];
        let _ = other.encode_utf8(&mut bytes);
        self.bytes == bytes
    }
}

impl PartialEq<FixedUtf8Char> for char {
    #[inline]
    fn eq(&self, other: &FixedUtf8Char) -> bool {
        <FixedUtf8Char as PartialEq<char>>::eq(other, self)
    }
}

impl From<FixedUtf8Char> for char {
    #[inline]
    fn from(f: FixedUtf8Char) -> Self {
        From::from(&f)
    }
}

impl From<&FixedUtf8Char> for char {
    #[inline]
    fn from(f: &FixedUtf8Char) -> Self {
        unsafe { f.as_str().chars().next().unwrap_unchecked() }
    }
}

impl Display for FixedUtf8Char {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}
