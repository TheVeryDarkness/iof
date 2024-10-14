//! [Format] trait for input format and built-in formats.

use crate::{
    stream::{COMMA, CR, HT, LF, SP},
    utf8char::FixedUtf8Char,
};

/// Trait for input format.
pub trait Format<Char = char> {
    /// Get the list of whitespace characters.
    fn skipped_chars(&self) -> &[Char];
}

impl<L: Format<Char> + ?Sized, Char> Format<Char> for &L {
    #[inline]
    fn skipped_chars(&self) -> &[Char] {
        <L as Format<Char>>::skipped_chars(self)
    }
}

/// Default Format.
///
/// Whitespace characters here are `' '`, `'\t'`, `'\n'`, and `'\r'`.
pub struct Default;

const WHITE_SPACES: [FixedUtf8Char; 4] = [SP, HT, LF, CR];

impl Format<FixedUtf8Char> for Default {
    #[inline]
    fn skipped_chars(&self) -> &[FixedUtf8Char] {
        &WHITE_SPACES
    }
}

impl Format<char> for Default {
    #[inline]
    fn skipped_chars(&self) -> &[char] {
        &[' ', '\t', '\n', '\r']
    }
}

/// Format for CSV.
///
/// Whitespace characters here are `' '`, `'\t'`, `','`, `'\n'`, and `'\r'`.
pub struct CSV;

const CSV_SEP: [FixedUtf8Char; 5] = [SP, HT, COMMA, LF, CR];

impl Format<FixedUtf8Char> for CSV {
    #[inline]
    fn skipped_chars(&self) -> &[FixedUtf8Char] {
        &CSV_SEP
    }
}

impl Format<char> for CSV {
    #[inline]
    fn skipped_chars(&self) -> &[char] {
        &[' ', '\t', ',', '\n', '\r']
    }
}

/// Special format that skip the given characters.
pub struct Skip<Char = char>(Vec<Char>);

impl<Char: From<C> + Ord, C> FromIterator<C> for Skip<Char> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        let mut v: Vec<_> = iter.into_iter().map(From::from).collect();
        v.sort();
        v.dedup();
        Self(v)
    }
}

impl Format<FixedUtf8Char> for Skip<FixedUtf8Char> {
    #[inline]
    fn skipped_chars(&self) -> &[FixedUtf8Char] {
        &self.0
    }
}

impl Format<char> for Skip<char> {
    #[inline]
    fn skipped_chars(&self) -> &[char] {
        &self.0
    }
}

/// Create a [Format] instance that skip the given characters.
#[inline]
pub fn skip<Char: Ord, T: IntoIterator<Item = Char>>(iter: T) -> Skip<Char> {
    iter.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::Default;
    use crate::{
        fmt::{Format, Skip, CSV, WHITE_SPACES},
        utf8char::FixedUtf8Char,
    };

    #[test]
    fn equivalence() {
        assert_eq!(
            <Default as Format<char>>::skipped_chars(&Default),
            <Default as Format<FixedUtf8Char>>::skipped_chars(&Default),
        );
        assert_eq!(
            <&Default as Format<char>>::skipped_chars(&&Default),
            <&Default as Format<FixedUtf8Char>>::skipped_chars(&&Default),
        );
        assert_eq!(
            <CSV as Format<char>>::skipped_chars(&CSV),
            <CSV as Format<FixedUtf8Char>>::skipped_chars(&CSV),
        );
        assert_eq!(
            <&CSV as Format<char>>::skipped_chars(&&CSV),
            <&CSV as Format<FixedUtf8Char>>::skipped_chars(&&CSV),
        );

        let seps = [' ', '\t', '\n', '\r'];
        assert_eq!(
            <Skip<char> as Format<char>>::skipped_chars(&FromIterator::from_iter(seps)),
            <Skip<FixedUtf8Char> as Format<FixedUtf8Char>>::skipped_chars(
                &FromIterator::from_iter(seps)
            ),
        );
        assert_eq!(
            <Skip<char> as Format<char>>::skipped_chars(&FromIterator::from_iter(WHITE_SPACES)),
            <Skip<FixedUtf8Char> as Format<FixedUtf8Char>>::skipped_chars(
                &FromIterator::from_iter(WHITE_SPACES)
            ),
        );
    }
}
