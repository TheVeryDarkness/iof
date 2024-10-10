//! Locale trait and default locale.

use crate::{
    stream::{COMMA, CR, HT, LF, SP},
    utf8char::FixedUtf8Char,
};

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
            string = &string[c.len_utf8()..];
        }
        count
    }
}

impl<L: Locale + ?Sized> Locale for &L {
    #[inline]
    fn whitespace_chars(&self) -> &[FixedUtf8Char] {
        <L as Locale>::whitespace_chars(self)
    }
}

/// Default locale.
///
/// ASCII whitespace characters here are `' '`, `'\t'`, `'\n'`, and `'\r'`.
pub struct ASCII;

pub(crate) const WHITE_SPACES: [FixedUtf8Char; 4] = [SP, HT, LF, CR];

impl Locale for ASCII {
    #[inline]
    fn whitespace_chars(&self) -> &[FixedUtf8Char] {
        &WHITE_SPACES
    }
}

/// Locale for CSV.
///
/// ASCII whitespace characters here are `' '`, `'\t'`, `','`, `'\n'`, and `'\r'`.
pub struct CSV;

pub(crate) const CSV_SEP: [FixedUtf8Char; 5] = [SP, HT, COMMA, LF, CR];

impl Locale for CSV {
    #[inline]
    fn whitespace_chars(&self) -> &[FixedUtf8Char] {
        &CSV_SEP
    }
}

/// Specific locale for whitespace characters.
pub struct WS(Vec<FixedUtf8Char>);

impl FromIterator<FixedUtf8Char> for WS {
    fn from_iter<T: IntoIterator<Item = FixedUtf8Char>>(iter: T) -> Self {
        let mut v: Vec<_> = iter.into_iter().collect();
        v.sort();
        v.dedup();
        Self(v)
    }
}

impl FromIterator<char> for WS {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        Self::from_iter(iter.into_iter().map(FixedUtf8Char::from))
    }
}

impl Locale for WS {
    fn whitespace_chars(&self) -> &[FixedUtf8Char] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::locale::{Locale, WS};
    use crate::read::locale::ASCII;

    #[test]
    fn prefix_whitespace_utf8() {
        let locale = ASCII;
        let s = "  \t\n\rHello, world!";
        assert_eq!(locale.prefix_whitespace_utf8(s), 5);
    }

    #[test]
    fn prefix_comma_utf8() {
        let locale = WS::from_iter([',', ' ']);
        let s = " , ,";
        assert_eq!(locale.prefix_whitespace_utf8(s), 4);
    }
}
