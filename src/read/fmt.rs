//! [Format] trait for input format and built-in formats.

use crate::{
    ext::CharExt,
    stream::{
        ext::{CharSet, StrExt},
        COMMA, CR, HT, LF, SP,
    },
    utf8char::FixedUtf8Char,
};
use std::marker::PhantomData;

/// Trait for input format.
pub trait Format<Char: CharExt = char>: Copy
where
    for<'s> &'s str: StrExt<'s, Char>,
{
    /// The type for skipped characters.
    type Skip: CharSet<Item = Char>;
    /// Get the pattern for skipped characters.
    fn skip(self) -> Self::Skip;

    // /// Check if the character should be skipped.
    // ///
    // /// # Note
    // ///
    // /// Normally this function should be marked with `#[inline]`.
    // fn skip(self, c: Char) -> bool;

    // /// Skip the prefix of a string.
    // #[inline]
    // fn skip_prefix<'s, S: StrExt<'s, Char>>(self, s: S) -> (S, S) {
    //     let mut i = 0;
    //     for c in s.chars_ext() {
    //         if !self.skip(c) {
    //             break;
    //         }
    //         i += c.len_utf8();
    //     }
    //     s.split(i)
    // }

    // /// Find the first not matching character.
    // #[inline]
    // fn find_first_not_skipped<'s, S: StrExt<'s, Char>>(self, s: S) -> Option<usize> {
    //     let mut cursor = 0;
    //     for c in s.chars_ext() {
    //         if !self.skip(c) {
    //             return Some(cursor);
    //         }
    //         cursor += c.len_utf8();
    //     }
    //     None
    // }

    // /// Find the first not matching character or the whole length.
    // ///
    // /// Returns the whole length if the string is fully matching.
    // #[inline]
    // fn find_first_not_skipped_or_whole_length<'s, S: StrExt<'s, Char>>(self, s: S) -> usize {
    //     self.find_first_not_skipped(s).unwrap_or(s.len())
    // }

    // /// Find the first matching character.
    // #[inline]
    // fn find_first_skipped<'s, S: StrExt<'s, Char>>(self, s: S) -> Option<usize> {
    //     let mut cursor = 0;
    //     for c in s.chars_ext() {
    //         if self.skip(c) {
    //             return Some(cursor);
    //         }
    //         cursor += c.len_utf8();
    //     }
    //     None
    // }

    // /// Find the first matching character or the whole length.
    // ///
    // /// Returns the whole length if the string is fully not matching.
    // #[inline]
    // fn find_first_skipped_or_whole_length<'s, S: StrExt<'s, Char>>(self, s: S) -> usize {
    //     self.find_first_skipped(s).unwrap_or(s.len())
    // }

    // /// Trim the end of a string.
    // #[inline]
    // fn trim_end<'s, S: StrExt<'s, Char>>(self, s: S) -> S {
    //     let i = self.find_first_skipped_or_whole_length(s);
    //     s.split(i).0
    // }

    // /// Trim the start of a string.
    // #[inline]
    // fn trim_start<'s, S: StrExt<'s, Char>>(self, s: S) -> S {
    //     // 0..i is the prefix to skip.
    //     let i = self.find_first_not_skipped_or_whole_length(s);
    //     s.split(i).1
    // }

    // /// Trim the start and end of a string.
    // #[inline]
    // fn trim<'s, S: StrExt<'s, Char>>(self, s: S) -> S {
    //     self.trim_end(self.trim_start(s))
    // }
}

impl<L: Format<Char>, Char: CharExt> Format<Char> for &L
where
    for<'s> &'s str: StrExt<'s, Char>,
{
    type Skip = L::Skip;
    #[inline]
    fn skip(self) -> Self::Skip {
        L::skip(*self)
    }
    // #[inline]
    // fn skip(self, c: Char) -> bool {
    //     <L as Format<Char>>::skip(*self, c)
    // }
}

// impl<Char: CharExt> Format<Char> for fn(Char) -> bool {
//     #[inline]
//     fn skip(self, c: Char) -> bool {
//         self(c)
//     }
// }

/// Default Format.
///
/// Whitespace characters here are `' '`, `'\t'`, `'\n'`, and `'\r'`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Default<Char>(PhantomData<Char>);

impl<Char> Default<Char> {
    /// Create a new instance.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

const WHITE_SPACES: [FixedUtf8Char; 4] = [SP, HT, LF, CR];

impl Format<FixedUtf8Char> for Default<FixedUtf8Char> {
    type Skip = &'static [FixedUtf8Char];
    #[inline]
    fn skip(self) -> Self::Skip {
        &WHITE_SPACES
    }
    // #[inline]
    // fn skip(self, c: FixedUtf8Char) -> bool {
    //     WHITE_SPACES.contains(&c)
    // }
}

impl Format<char> for Default<char> {
    type Skip = &'static [char];
    #[inline]
    fn skip(self) -> Self::Skip {
        &[' ', '\t', '\n', '\r']
    }
    // #[inline]
    // fn skip(self, c: char) -> bool {
    //     [' ', '\t', '\n', '\r'].contains(&c)
    // }
}

/// Format for CSV.
///
/// Whitespace characters here are `' '`, `'\t'`, `','`, `'\n'`, and `'\r'`.
#[derive(Debug, Clone, Copy, Default)]
pub struct CSV<Char>(PhantomData<Char>);

impl<Char> CSV<Char> {
    /// Create a new instance.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

const CSV_SEP: [FixedUtf8Char; 5] = [SP, HT, COMMA, LF, CR];

impl Format<FixedUtf8Char> for CSV<FixedUtf8Char> {
    type Skip = &'static [FixedUtf8Char];
    #[inline]
    fn skip(self) -> Self::Skip {
        &CSV_SEP
    }
    // #[inline]
    // fn skip(self, c: FixedUtf8Char) -> bool {
    //     CSV_SEP.contains(&c)
    // }
}

impl Format<char> for CSV<char> {
    type Skip = &'static [char];
    #[inline]
    fn skip(self) -> Self::Skip {
        &[' ', '\t', ',', '\n', '\r']
    }
    // #[inline]
    // fn skip(self, c: char) -> bool {
    //     [' ', '\t', ',', '\n', '\r'].contains(&c)
    // }
}

/// Special format that skip the given characters.
#[derive(Debug, Clone, Default)]
pub struct Skip<Char = char>(Vec<Char>);

impl<Char> Skip<Char> {
    /// Create a new instance.
    pub const fn new() -> Self {
        Self(Vec::new())
    }
}

impl<Char: From<C> + Ord, C> FromIterator<C> for Skip<Char> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        let mut v: Vec<_> = iter.into_iter().map(From::from).collect();
        v.sort();
        v.dedup();
        Self(v)
    }
}

impl<'s> Format<FixedUtf8Char> for &'s Skip<FixedUtf8Char> {
    type Skip = &'s [FixedUtf8Char];
    #[inline]
    fn skip(self) -> Self::Skip {
        &self.0
    }
    // #[inline]
    // fn skip(self, c: FixedUtf8Char) -> bool {
    //     self.0.contains(&c)
    // }
}

impl<'s> Format<char> for &'s Skip<char> {
    type Skip = &'s [char];
    #[inline]
    fn skip(self) -> Self::Skip {
        &self.0
    }
    // #[inline]
    // fn skip(self, c: char) -> bool {
    //     self.0.contains(&c)
    // }
}

/// Create a [Format] instance that skip `' '`, `'\t'`, `'\n'`, and `'\r'`.
#[inline]
pub fn default<Char>() -> Default<Char> {
    Default::new()
}

/// Create a [Format] instance that skip `' '`, `'\t'`, `','`, `'\n'`, and `'\r'`.
#[inline]
pub fn csv<Char>() -> CSV<Char> {
    CSV::new()
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
        ext::{Any, CharSet as _},
        fmt::{Format, Skip, CSV, WHITE_SPACES},
        utf8char::FixedUtf8Char,
    };

    fn equivalence_for_char(c: char) {
        assert_eq!(
            Default::<char>::new().skip().matches(c),
            Default::<FixedUtf8Char>::new().skip().matches(c.into()),
        );
        assert_eq!(
            Default::<char>::new().skip().matches(c),
            Default::<FixedUtf8Char>::new().skip().matches(c.into()),
        );
        assert_eq!(
            CSV::<char>::new().skip().matches(c),
            CSV::<FixedUtf8Char>::new().skip().matches(c.into()),
        );
        assert_eq!(
            CSV::<char>::new().skip().matches(c),
            CSV::<FixedUtf8Char>::new().skip().matches(c.into()),
        );

        let seps = [' ', '\t', '\n', '\r'];
        assert_eq!(
            Skip::<char>::from_iter(seps).skip().matches(c),
            Skip::<FixedUtf8Char>::from_iter(seps)
                .skip()
                .matches(c.into()),
        );
        assert_eq!(
            Skip::<char>::from_iter(WHITE_SPACES).skip().matches(c),
            Skip::<FixedUtf8Char>::from_iter(WHITE_SPACES)
                .skip()
                .matches(c.into()),
        );
    }

    #[test]
    fn equivalence_char() {
        equivalence_for_char(' ');
        equivalence_for_char('\t');
        equivalence_for_char('\n');
        equivalence_for_char('\r');
        equivalence_for_char('a');
        equivalence_for_char('å');
        equivalence_for_char('🦀');
        equivalence_for_char('中');
        equivalence_for_char('文');
    }

    fn equivalence_for_string(s: &str) {
        assert_eq!(
            Default::<char>::new().skip().trim_end(s),
            Default::<FixedUtf8Char>::new().skip().trim_end(s),
        );
        assert_eq!(
            Default::<char>::new().skip().trim(s),
            Default::<FixedUtf8Char>::new().skip().trim(s),
        );
    }

    #[test]
    fn equivalence_string() {
        equivalence_for_string(" ");
        equivalence_for_string("\t");
        equivalence_for_string("\n");
        equivalence_for_string("\r");
        equivalence_for_string("a");
        equivalence_for_string("å");
        equivalence_for_string("🦀");
        equivalence_for_string("中");
        equivalence_for_string("文");
    }

    #[test]
    fn any() {
        for s in ["", " ", "\t", "\n", "\r", "a", "å", "🦀", "中", "文"].iter() {
            let d = Any::<char>::new();
            assert_eq!(d.trim_start(s), "");
            assert_eq!(d.trim_end(s), "");
            assert_eq!(d.trim(s), "");
        }
    }
}
