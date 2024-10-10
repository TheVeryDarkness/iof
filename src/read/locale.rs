//! Locale trait and default locale.

use crate::utf8char::FixedUtf8Char;

/// Locale trait.
pub trait Locale {
    /// Get the list of whitespace characters.
    fn whitespace_chars(&self) -> &[FixedUtf8Char];
    /// Get the length of the leading whitespace in `bytes`.
    #[inline]
    fn prefix_whitespace_utf8(&self, mut string: &str) -> usize {
        let mut count = 0;
        while let Some(c) = self
            .whitespace_chars()
            .iter()
            .find(|c| string.as_bytes().starts_with(c.as_bytes()))
        {
            count += 1;
            string = &string[c.len()..];
        }
        count
    }
}

/// Default locale.
///
/// ASCII whitespace characters here are `' '`, `'\t'`, `'\n'`, and `'\r'`.
pub struct ASCII;

pub(crate) const WHITE_SPACES: [FixedUtf8Char; 4] = unsafe {
    [
        FixedUtf8Char::from_bytes_unchecked([b' ', 0, 0, 0]),
        FixedUtf8Char::from_bytes_unchecked([b'\t', 0, 0, 0]),
        FixedUtf8Char::from_bytes_unchecked([b'\n', 0, 0, 0]),
        FixedUtf8Char::from_bytes_unchecked([b'\r', 0, 0, 0]),
    ]
};

impl Locale for ASCII {
    fn whitespace_chars(&self) -> &[FixedUtf8Char] {
        &WHITE_SPACES
    }
}

#[cfg(test)]
mod tests {
    use crate::locale::Locale;
    use crate::read::locale::ASCII;

    #[test]
    fn prefix_whitespace_utf8() {
        let locale = ASCII;
        let s = "  \t\n\rHello, world!";
        assert_eq!(locale.prefix_whitespace_utf8(s), 5);
    }
}
