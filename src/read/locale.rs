//! Locale trait and default locale.

use crate::{
    stream::{COMMA, CR, HT, LF, SP},
    utf8char::FixedUtf8Char,
};

/// Locale trait.
pub trait Locale<Char = char> {
    /// Get the list of whitespace characters.
    fn whitespace_chars(&self) -> &[Char];
}

impl<L: Locale<Char> + ?Sized, Char> Locale<Char> for &L {
    #[inline]
    fn whitespace_chars(&self) -> &[Char] {
        <L as Locale<Char>>::whitespace_chars(self)
    }
}

/// Default locale.
///
/// Whitespace characters here are `' '`, `'\t'`, `'\n'`, and `'\r'`.
pub struct ASCII;

const WHITE_SPACES: [FixedUtf8Char; 4] = [SP, HT, LF, CR];

impl Locale<FixedUtf8Char> for ASCII {
    #[inline]
    fn whitespace_chars(&self) -> &[FixedUtf8Char] {
        &WHITE_SPACES
    }
}

impl Locale<char> for ASCII {
    #[inline]
    fn whitespace_chars(&self) -> &[char] {
        &[' ', '\t', '\n', '\r']
    }
}

/// Locale for CSV.
///
/// Whitespace characters here are `' '`, `'\t'`, `','`, `'\n'`, and `'\r'`.
pub struct CSV;

const CSV_SEP: [FixedUtf8Char; 5] = [SP, HT, COMMA, LF, CR];

impl Locale<FixedUtf8Char> for CSV {
    #[inline]
    fn whitespace_chars(&self) -> &[FixedUtf8Char] {
        &CSV_SEP
    }
}

impl Locale<char> for CSV {
    #[inline]
    fn whitespace_chars(&self) -> &[char] {
        &[' ', '\t', ',', '\n', '\r']
    }
}

/// Specific locale for whitespace characters.
pub struct WS<Char = char>(Vec<Char>);

impl<Char: From<FixedUtf8Char> + Ord> FromIterator<FixedUtf8Char> for WS<Char> {
    fn from_iter<T: IntoIterator<Item = FixedUtf8Char>>(iter: T) -> Self {
        let mut v: Vec<_> = iter.into_iter().map(From::from).collect();
        v.sort();
        v.dedup();
        Self(v)
    }
}

impl<Char: From<char> + Ord> FromIterator<char> for WS<Char> {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut v: Vec<_> = iter.into_iter().map(From::from).collect();
        v.sort();
        v.dedup();
        Self(v)
    }
}

impl Locale<FixedUtf8Char> for WS<FixedUtf8Char> {
    fn whitespace_chars(&self) -> &[FixedUtf8Char] {
        &self.0
    }
}

impl Locale<char> for WS<char> {
    fn whitespace_chars(&self) -> &[char] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ASCII;
    use crate::{
        locale::{Locale, CSV, WHITE_SPACES, WS},
        utf8char::FixedUtf8Char,
    };

    #[test]
    fn equivalence() {
        assert_eq!(
            <ASCII as Locale<char>>::whitespace_chars(&ASCII),
            <ASCII as Locale<FixedUtf8Char>>::whitespace_chars(&ASCII),
        );
        assert_eq!(
            <&ASCII as Locale<char>>::whitespace_chars(&&ASCII),
            <&ASCII as Locale<FixedUtf8Char>>::whitespace_chars(&&ASCII),
        );
        assert_eq!(
            <CSV as Locale<char>>::whitespace_chars(&CSV),
            <CSV as Locale<FixedUtf8Char>>::whitespace_chars(&CSV),
        );
        assert_eq!(
            <&CSV as Locale<char>>::whitespace_chars(&&CSV),
            <&CSV as Locale<FixedUtf8Char>>::whitespace_chars(&&CSV),
        );

        let seps = [' ', '\t', '\n', '\r'];
        assert_eq!(
            <WS<char> as Locale<char>>::whitespace_chars(&FromIterator::from_iter(seps)),
            <WS<FixedUtf8Char> as Locale<FixedUtf8Char>>::whitespace_chars(
                &FromIterator::from_iter(seps)
            ),
        );
        assert_eq!(
            <WS<char> as Locale<char>>::whitespace_chars(&FromIterator::from_iter(WHITE_SPACES)),
            <WS<FixedUtf8Char> as Locale<FixedUtf8Char>>::whitespace_chars(
                &FromIterator::from_iter(WHITE_SPACES)
            ),
        );
    }
}
