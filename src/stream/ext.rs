//! Extensions for characters and strings.

use super::{CR, LF};
use crate::utf8char::{FixedUtf8Char, IterFixedUtf8Char};
use std::{
    error::Error,
    fmt::{self, Debug},
    marker::PhantomData,
    ops::Not,
};

/// Extension traits for characters.
pub trait CharExt: Copy + Debug {
    /// End of line characters.
    ///
    /// Represents the characters `'\n'` and `'\r'` respectively.
    const EOL: [Self; 2];

    /// Get the length of the character in UTF-8.
    fn len_utf8(&self) -> usize;

    // /// Get an iterator over characters.
    // fn chars_ext(s: &str) -> impl Iterator<Item = Self> + DoubleEndedIterator;

    // /// Get the first character.
    // fn first_char(s: &str) -> Option<Self>;
}

impl CharExt for FixedUtf8Char {
    const EOL: [Self; 2] = [LF, CR];

    #[inline]
    fn len_utf8(&self) -> usize {
        Self::len_utf8(self)
    }

    // #[inline]
    // fn chars_ext(s: &str) -> impl Iterator<Item = Self> + DoubleEndedIterator {
    //     IterFixedUtf8Char::new(s)
    // }

    // #[inline]
    // fn first_char(s: &str) -> Option<Self> {
    //     FixedUtf8Char::from_first_char(s)
    // }
}

impl CharExt for char {
    const EOL: [Self; 2] = ['\n', '\r'];

    #[inline]
    fn len_utf8(&self) -> usize {
        Self::len_utf8(*self)
    }

    // #[inline]
    // fn chars_ext(s: &str) -> impl Iterator<Item = Self> + DoubleEndedIterator {
    //     s.chars()
    // }

    // #[inline]
    // fn first_char(s: &str) -> Option<Self> {
    //     s.chars().next()
    // }
}

/// Extension traits for strings.
pub trait StrExt<'s, C: CharExt>: Sized + Copy {
    /// An iterator over characters.
    type Iterator: Iterator<Item = C> + DoubleEndedIterator;

    /// Get an iterator over characters.
    fn chars_ext(self) -> Self::Iterator;

    /// Get the first character.
    #[inline]
    fn first_char(self) -> Option<C> {
        self.chars_ext().next()
    }

    // /// Split the string at the middle.
    // fn split(self, mid: usize) -> (Self, Self);

    /// Get length of the string, in bytes.
    fn len(self) -> usize;

    /// Check if the string is empty.
    #[inline]
    fn is_empty(self) -> bool {
        self.len() == 0
    }
}

// impl<'s, Char: CharExt> StrExt<'s, Char> for &'s str {
//     type Iterator = Char::Iter;

//     #[inline]
//     fn chars_ext(self) -> Self::Iterator {
//         Char::chars_ext(self)
//     }

//     #[inline]
//     fn first_char(self) -> Option<Char> {
//         Char::first_char(self)
//     }

//     #[inline]
//     fn split(self, mid: usize) -> (Self, Self) {
//         self.split_at(mid)
//     }

//     #[inline]
//     fn len(self) -> usize {
//         self.len()
//     }
// }

impl<'s> StrExt<'s, FixedUtf8Char> for &'s str {
    type Iterator = IterFixedUtf8Char<'s>;

    #[inline]
    fn chars_ext(self) -> Self::Iterator {
        IterFixedUtf8Char::new(self)
    }

    #[inline]
    fn first_char(self) -> Option<FixedUtf8Char> {
        FixedUtf8Char::from_first_char(self)
    }

    // #[inline]
    // fn split(self, mid: usize) -> (Self, Self) {
    //     self.split_at(mid)
    // }

    #[inline]
    fn len(self) -> usize {
        self.len()
    }
}

impl<'s> StrExt<'s, char> for &'s str {
    type Iterator = std::str::Chars<'s>;

    #[inline]
    fn chars_ext(self) -> Self::Iterator {
        self.chars()
    }

    // #[inline]
    // fn first_char(self) -> Option<char> {
    //     self.chars().next()
    // }

    // #[inline]
    // fn split(self, mid: usize) -> (Self, Self) {
    //     self.split_at(mid)
    // }

    #[inline]
    fn len(self) -> usize {
        self.len()
    }
}

/// A set of chars.
pub trait CharSet: Sized + Copy
where
    for<'s> &'s str: StrExt<'s, Self::Item>,
{
    /// The item type.
    type Item: CharExt;

    /// Check whether the character matches the pattern.
    fn matches(&self, c: Self::Item) -> bool;

    /// Trim the start of the string.
    #[inline]
    fn trim_start(self, s: &str) -> &str {
        self.find_first_not_matching(s).map_or(s, |i| &s[i..])
        // let mut cursor = 0;
        // for c in s.chars_ext() {
        //     if !self.matches(c) {
        //         return &s[cursor..];
        //     }
        //     cursor += c.len_utf8();
        // }
        // &s[s.len()..]
    }

    /// Trim the end of the string.
    #[inline]
    fn trim_end(self, s: &str) -> &str {
        self.find_last_not_matching(s).map_or(&s[..0], |i| &s[..i])
        // let mut cursor = s.len();
        // for c in s.chars_ext().rev() {
        //     if !self.matches(c) {
        //         return &s[..cursor];
        //     }
        //     cursor -= c.len_utf8();
        // }
        // &s[..0]
    }

    /// Trim the string.
    #[inline]
    fn trim(self, s: &str) -> &str {
        self.trim_end(self.trim_start(s))
    }

    /// Find the first matching character.
    #[inline]
    fn find_first_matching(self, s: &str) -> Option<usize> {
        let mut cursor = 0;
        for c in s.chars_ext() {
            if self.matches(c) {
                return Some(cursor);
            }
            cursor += c.len_utf8();
        }
        None
    }

    /// Find the first not matching character.
    #[inline]
    fn find_first_not_matching(self, s: &str) -> Option<usize> {
        let mut cursor = 0;
        for c in s.chars_ext() {
            if !self.matches(c) {
                return Some(cursor);
            }
            cursor += c.len_utf8();
        }
        None
    }

    /// Find the end offset of the last matching character.
    #[inline]
    fn find_last_matching(self, s: &str) -> Option<usize> {
        let mut cursor = s.len();
        for c in s.chars_ext().rev() {
            if self.matches(c) {
                return Some(cursor);
            }
            cursor -= c.len_utf8();
        }
        None
    }

    /// Find the end offset of the last not matching character.
    #[inline]
    fn find_last_not_matching(self, s: &str) -> Option<usize> {
        let mut cursor = s.len();
        for c in s.chars_ext().rev() {
            if !self.matches(c) {
                return Some(cursor);
            }
            cursor -= c.len_utf8();
        }
        None
    }

    /// Subtract another pattern from this pattern.
    #[inline]
    fn except<B: CharSet<Item = Self::Item>>(self, b: B) -> impl CharSet<Item = Self::Item> {
        CharSetSubtract::new(self, b)
    }

    /// Reverse the pattern.
    #[inline]
    fn not(self) -> impl CharSet<Item = Self::Item> {
        AnyBut::new(self)
    }
}

impl<Char: CharExt, C: CharSet<Item = Char>> Pattern for C
where
    for<'s> &'s str: StrExt<'s, Char>,
{
    type Item = Char;

    #[inline]
    fn step(&mut self, c: <Self as Pattern>::Item) -> bool {
        self.matches(c)
    }

    #[inline]
    fn state(&self) -> State {
        State::Stoppable
    }

    #[inline]
    fn forward<E>(self, s: &str) -> Result<usize, PatternError<E>> {
        // Ok(self.find_first_not_matching_or_whole_length(s))
        match self.find_first_not_matching(s).unwrap_or(s.len()) {
            0 => Err(PatternError::UnexpectedChar(
                s.chars().next().map(|c| c.to_string()).unwrap_or_default(),
            )),
            i => Ok(i),
        }
    }

    #[inline]
    fn except<B: CharSet<Item = Self::Item>>(self, b: B) -> impl Pattern<Item = Self::Item> {
        self.except(b)
    }
}

impl CharSet for &[FixedUtf8Char] {
    type Item = FixedUtf8Char;

    #[inline]
    fn matches(&self, c: Self::Item) -> bool {
        self.contains(&c)
    }

    #[inline]
    fn trim_end(self, s: &str) -> &str {
        let mut line = s;
        while let Some(c) = self.iter().find(|&&c| line.ends_with(c.as_str())) {
            let cursor = line.len() - c.len_utf8();
            debug_assert!(line.is_char_boundary(cursor));
            line = unsafe { line.get_unchecked(..cursor) };
        }
        line
    }

    #[inline]
    fn trim_start(self, s: &str) -> &str {
        let mut line = s;
        while let Some(c) = self.iter().find(|&&c| line.starts_with(c.as_str())) {
            let cursor = c.len_utf8();
            debug_assert!(line.is_char_boundary(cursor));
            line = unsafe { line.get_unchecked(cursor..) };
        }
        line
    }

    #[inline]
    fn trim(self, s: &str) -> &str {
        self.trim_end(self.trim_start(s))
    }
}

impl CharSet for &[char] {
    type Item = char;

    #[inline]
    fn matches(&self, c: Self::Item) -> bool {
        self.contains(&c)
    }

    #[inline]
    fn trim_start(self, s: &str) -> &str {
        s.trim_start_matches(self)
    }

    #[inline]
    fn trim_end(self, s: &str) -> &str {
        s.trim_end_matches(self)
    }

    #[inline]
    fn trim(self, s: &str) -> &str {
        s.trim_matches(self)
    }

    #[inline]
    fn find_first_matching(self, s: &str) -> Option<usize> {
        s.find(self)
    }

    #[inline]
    fn find_first_not_matching(self, s: &str) -> Option<usize> {
        let l = s.trim_start_matches(self).len();
        if l == 0 {
            None
        } else {
            Some(s.len() - l)
        }
    }

    #[inline]
    fn find_last_matching(self, s: &str) -> Option<usize> {
        let start = s.rfind(self);
        start.map(|i| {
            i + s[i..]
                .char_indices()
                .next()
                .map(|(_, c)| c.len_utf8())
                .unwrap_or_else(|| unreachable!())
        })
    }

    #[inline]
    fn find_last_not_matching(self, s: &str) -> Option<usize> {
        let l = s.trim_end_matches(self).len();
        if l == 0 {
            None
        } else {
            Some(l)
        }
    }
}

// macro_rules! impl_pattern_for_range {
//     ($ty:ty) => {
//         impl Pattern for $ty {
//             type Item = char;

//             #[inline]
//             fn step<E>(
//                 &mut self,
//                 c: <Self as Pattern>::Item,
//             ) -> Result<bool, PatternError<E, Self::Item>> {
//                 if self.contains(&c) {
//                     Ok(false)
//                 } else {
//                     Err(PatternError::UnexpectedChar(c))
//                 }
//             }
//         }
//     };
// }

// impl_pattern_for_range!(Range<char>);
// impl_pattern_for_range!(RangeInclusive<char>);
// impl_pattern_for_range!(RangeToInclusive<char>);
// impl_pattern_for_range!(RangeTo<char>);
// impl_pattern_for_range!(RangeFrom<char>);

/// A pattern that matches any character.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Any<Char>(PhantomData<Char>);

impl<Char> Any<Char> {
    /// Create a new instance.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<Char: CharExt> CharSet for Any<Char>
where
    for<'s> &'s str: StrExt<'s, Char>,
{
    type Item = Char;

    #[inline]
    fn matches(&self, _: Self::Item) -> bool {
        true
    }

    #[inline]
    fn trim_start(self, s: &str) -> &str {
        &s[s.len()..]
    }

    #[inline]
    fn trim_end(self, s: &str) -> &str {
        &s[..0]
    }

    #[inline]
    fn find_first_matching(self, s: &str) -> Option<usize> {
        s.is_empty().not().then_some(0)
    }

    #[inline]
    fn find_first_not_matching(self, s: &str) -> Option<usize> {
        let _ = s;
        None
    }

    #[inline]
    fn except<B: CharSet<Item = Self::Item>>(self, b: B) -> impl CharSet<Item = Self::Item> {
        b.not()
    }

    #[inline]
    fn not(self) -> impl CharSet<Item = Self::Item> {
        AnyBut::new(self)
    }
}

/// A pattern that matches any character.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AnyBut<T>(T);

impl<T> AnyBut<T> {
    /// Create a new instance.
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl<Char: CharExt, C: CharSet<Item = Char>> CharSet for AnyBut<C>
where
    for<'s> &'s str: StrExt<'s, Char>,
{
    type Item = Char;

    #[inline]
    fn matches(&self, c: Self::Item) -> bool {
        !self.0.matches(c)
    }

    #[inline]
    fn trim_start(self, s: &str) -> &str {
        let cursor = self.0.find_first_matching(s).unwrap_or(s.len());
        &s[cursor..]
    }

    // #[inline]
    // fn trim_end(self, s: &str) -> &str {
    //     &s[..0]
    // }

    #[inline]
    fn find_first_matching(self, s: &str) -> Option<usize> {
        self.0.find_first_not_matching(s)
    }

    #[inline]
    fn find_first_not_matching(self, s: &str) -> Option<usize> {
        self.0.find_first_matching(s)
    }

    #[inline]
    fn find_last_matching(self, s: &str) -> Option<usize> {
        self.0.find_last_not_matching(s)
    }

    #[inline]
    fn find_last_not_matching(self, s: &str) -> Option<usize> {
        self.0.find_last_matching(s)
    }

    #[inline]
    fn not(self) -> impl CharSet<Item = Self::Item> {
        self.0
    }
}

/// A charset that match `A` but not `B`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CharSetSubtract<A, B>(A, B);

impl<A, B> CharSetSubtract<A, B> {
    /// Create a new instance.
    pub const fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<Char: CharExt, A: CharSet<Item = Char>, B: CharSet<Item = Char>> CharSet
    for CharSetSubtract<A, B>
where
    for<'s> &'s str: StrExt<'s, Char>,
{
    type Item = Char;

    #[inline]
    fn matches(&self, c: Self::Item) -> bool {
        self.0.matches(c) && !self.1.matches(c)
    }
}

/// A pattern that match `A` but not `B`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PatternSubtract<A, B>(A, B);

impl<A, B> PatternSubtract<A, B> {
    /// Create a new instance.
    pub const fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<Char: CharExt, A: Pattern<Item = Char>, B: CharSet<Item = Char>> Pattern
    for PatternSubtract<A, B>
where
    for<'s> &'s str: StrExt<'s, Char>,
{
    type Item = Char;

    #[inline]
    fn step(&mut self, c: <Self as Pattern>::Item) -> bool {
        !self.1.matches(c) && self.0.step(c)
    }

    #[inline]
    fn state(&self) -> State {
        self.0.state()
    }
}

/// An error that occurs during pattern matching.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PatternError<E> {
    /// An unexpected character at the end of the string.
    UnexpectedChar(String),
    /// An extra error. Probably from reading.
    Extra(E),
}

impl<E: Error> From<E> for PatternError<E> {
    #[inline]
    fn from(value: E) -> Self {
        Self::Extra(value)
    }
}

impl<E: Error> fmt::Display for PatternError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(s) => write!(f, "unexpected character at the end of {s:?}"),
            Self::Extra(e) => fmt::Display::fmt(e, f),
        }
    }
}

/// The state of a pattern matcher.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    /// The pattern is unfulfilled.
    Unfulfilled,
    /// The pattern is stoppable.
    Stoppable,
    // /// The pattern is overrun and not recoverable.
    // Overrun,
}

/// A state-machine for pattern matching.
pub trait Pattern: Sized + Copy
where
    for<'s> &'s str: StrExt<'s, <Self as Pattern>::Item>,
{
    /// The item type.
    type Item: CharExt;

    /// Step the pattern with a character.
    ///
    /// # Returns
    ///
    /// - `true`: Stepped.
    /// - `false`: Unexpected character, and not recoverable.
    fn step(&mut self, c: <Self as Pattern>::Item) -> bool;

    /// Get the current [`State`] of the pattern.
    fn state(&self) -> State;

    /// Step the pattern with a string to check if it matches the prefix.
    #[inline]
    fn forward<E>(mut self, s: &str) -> Result<usize, PatternError<E>> {
        let mut cursor = 0;
        let mut last_stoppable = None;
        for c in s.chars_ext() {
            if self.step(c) {
                match self.state() {
                    State::Stoppable => {
                        cursor += c.len_utf8();
                        last_stoppable = Some(cursor);
                    }
                    // State::Overrun => {
                    //     break;
                    // }
                    State::Unfulfilled => {
                        cursor += c.len_utf8();
                    }
                }
            } else {
                cursor += c.len_utf8();
                break;
            }
        }
        last_stoppable.ok_or(PatternError::UnexpectedChar(s[..cursor].to_owned()))
    }

    /// Subtract another pattern from this pattern.
    #[inline]
    fn except<B: CharSet<Item = Self::Item>>(self, b: B) -> impl Pattern<Item = Self::Item> {
        PatternSubtract::new(self, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ext::Any, stream::ext::StrExt, utf8char::FixedUtf8Char};
    use fmt::{Debug, Display};

    fn chars<Char>()
    where
        for<'a> &'a [Char]: CharSet<Item = Char>,
        Char: From<char> + Copy + CharExt + PartialEq<Char> + PartialEq<char> + Debug + Display,
        for<'a> &'a str: StrExt<'a, Char>,
        char: PartialEq<Char> + From<Char>,
    {
        let ws = Char::EOL;
        let ws = ws.as_slice();
        assert!(ws.matches('\n'.into()));
        assert!(ws.matches('\r'.into()));
        assert!(!ws.matches(' '.into()));
        assert!(!ws.matches('a'.into()));

        let s = " \n\r\n";
        assert_eq!(ws.trim_start(s), " \n\r\n");
        assert_eq!(ws.trim_end(s), " ");
        assert_eq!(ws.trim(s), " ");

        assert_eq!(ws.find_first_matching(s), Some(1));
        assert_eq!(ws.find_first_not_matching(s), Some(0));

        let s = "\r\nabc\n";
        assert_eq!(ws.trim_start(s), "abc\n");
        assert_eq!(ws.trim_end(s), "\r\nabc");
        assert_eq!(ws.trim(s), "abc");

        assert_eq!(ws.find_first_matching(s), Some(0));
        assert_eq!(ws.find_first_not_matching(s), Some(2));

        let s = "\n\r\n";
        assert_eq!(ws.trim_start(s), "");
        assert_eq!(ws.trim_end(s), "");
        assert_eq!(ws.trim(s), "");

        assert_eq!(ws.find_first_matching(s), Some(0));
        assert_eq!(ws.find_first_not_matching(s), None);

        let s = "+-*/";
        assert_eq!(ws.trim_start(s), "+-*/");
        assert_eq!(ws.trim_end(s), "+-*/");
        assert_eq!(ws.trim(s), "+-*/");

        assert_eq!(ws.find_first_matching(s), None);
        assert_eq!(ws.find_first_not_matching(s), Some(0));

        let s = "";
        assert_eq!(ws.trim_start(s), "");
        assert_eq!(ws.trim_end(s), "");
        assert_eq!(ws.trim(s), "");

        assert_eq!(ws.find_first_matching(s), None);
        assert_eq!(ws.find_first_not_matching(s), None);

        for s in [
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~",
            "🦀🦀🦀🦀",
            "Hello, World! 你好，世界！",
        ] {
            assert_eq!(s.len(), <&str as StrExt<Char>>::len(s));
            assert_eq!(s.is_empty(), <&str as StrExt<Char>>::is_empty(s));

            assert_eq!(Some(0), Any::new().find_first_matching(s));
            assert_eq!(None, Any::new().not().find_first_matching(s));
            assert_eq!(Some(0), Any::new().not().find_first_not_matching(s));
            assert_eq!(None, Any::new().find_first_not_matching(s));

            let characters = s.chars().collect::<Vec<_>>();
            for (i, c) in <&str as StrExt<Char>>::chars_ext(s).enumerate() {
                assert!(Any::new().matches(c));
                assert_eq!(c, characters[i]);
                assert_eq!(c.to_string(), characters[i].to_string());
                assert_eq!(c.len_utf8(), characters[i].len_utf8());
            }

            let characters: Vec<Char> = s.chars_ext().collect();
            for (i, (index, c)) in s.char_indices().enumerate() {
                let sub = &s[index..];
                assert_eq!(c, characters[i]);
                assert_eq!(sub.first_char().map(Into::into), Some(c));
                assert_eq!(sub.first_char().unwrap(), c);
            }

            let characters_reverse: Vec<Char> = s.chars_ext().rev().collect();
            assert_eq!(characters_reverse.len(), characters.len());
            assert_eq!(
                characters_reverse.into_iter().rev().collect::<Vec<_>>(),
                characters,
            );
        }
    }

    #[test]
    fn chars_char() {
        chars::<char>();
    }

    #[test]
    fn chars_fixed_utf8_char() {
        chars::<FixedUtf8Char>();
    }

    fn test_forward<Char>()
    where
        Char: CharExt + PartialEq + From<char>,
        for<'a> &'a [Char]: Pattern<Item = Char> + CharSet<Item = Char>,
        for<'s> &'s str: StrExt<'s, Char>,
    {
        type E = ();
        let ws = Char::EOL;
        let ws = ws.as_slice();
        let p = ws;

        assert_eq!(p.trim("\n\r\n"), "");
        assert_eq!(p.trim(" \n\r\n"), " ");
        assert_eq!(p.trim(" \n\r\nabc"), " \n\r\nabc");

        assert_eq!(p.not().not().trim("\n\r\n"), "");
        assert_eq!(p.not().not().trim(" \n\r\n"), " ");
        assert_eq!(p.not().not().trim(" \n\r\nabc"), " \n\r\nabc");

        assert_eq!(CharSet::except(p, p).trim(" \n\r\n"), " \n\r\n");
        assert_eq!(CharSet::except(p, p).trim(""), "");
        assert_eq!(CharSet::except(p, p).find_first_matching(""), None);
        assert_eq!(CharSet::except(p, p).find_first_not_matching(""), None);
        assert_eq!(CharSet::except(p, p).find_first_matching("a"), None);
        assert_eq!(CharSet::except(p, p).find_first_not_matching("a"), Some(0));
        assert_eq!(CharSet::except(p, p).find_last_matching(""), None);
        assert_eq!(CharSet::except(p, p).find_last_not_matching(""), None);
        assert_eq!(CharSet::except(p, p).find_last_matching("a"), None);
        assert_eq!(CharSet::except(p, p).find_last_not_matching("a"), Some(1));

        assert!(!p.matches(' '.into()));
        assert!(!p.matches('a'.into()));
        assert!(p.matches('\r'.into()));
        assert!(p.matches('\n'.into()));

        for s in [" ", "", "\r\n", "🦀", "中文", "aå bß"] {
            assert_eq!(p.find_first_matching(s), p.not().find_first_not_matching(s));
            assert_eq!(p.find_first_not_matching(s), p.not().find_first_matching(s));
            assert_eq!(p.find_last_matching(s), p.not().find_last_not_matching(s));
            assert_eq!(p.find_last_not_matching(s), p.not().find_last_matching(s));
        }

        assert_eq!(p.find_first_matching(" \n\r\n xyz"), Some(1));
        assert_eq!(p.find_first_not_matching(" \n\r\n xyz"), Some(0));
        assert_eq!(p.find_last_matching(" \n\r\n xyz"), Some(4));
        assert_eq!(p.find_last_not_matching(" \n\r\n xyz"), Some(8));

        assert_eq!(p.not().find_first_matching(" \n\r\n xyz"), Some(0));
        assert_eq!(p.not().find_first_not_matching(" \n\r\n xyz"), Some(1));
        assert_eq!(p.not().find_last_matching(" \n\r\n xyz"), Some(8));
        assert_eq!(p.not().find_last_not_matching(" \n\r\n xyz"), Some(4));

        assert_eq!(p.find_first_matching(" "), None);
        assert_eq!(p.find_first_not_matching(" "), Some(0));
        assert_eq!(p.find_last_matching(" "), None);
        assert_eq!(p.find_last_not_matching(" "), Some(1));

        assert_eq!(p.not().find_first_matching(" "), Some(0));
        assert_eq!(p.not().find_first_not_matching(" "), None);
        assert_eq!(p.not().find_last_matching(" "), Some(1));
        assert_eq!(p.not().find_last_not_matching(" "), None);

        assert_eq!(p.find_first_matching(""), None);
        assert_eq!(p.find_first_not_matching(""), None);
        assert_eq!(p.find_last_matching(""), None);
        assert_eq!(p.find_last_not_matching(""), None);

        assert_eq!(p.not().find_first_matching(""), None);
        assert_eq!(p.not().find_first_not_matching(""), None);
        assert_eq!(p.not().find_last_matching(""), None);
        assert_eq!(p.not().find_last_not_matching(""), None);

        assert_eq!(p.not().trim_start(" \n\r\n"), "\n\r\n");
        assert_eq!(p.not().trim_end(" \n\r\n"), " \n\r\n");
        assert_eq!(p.not().trim(" \n\r\n"), "\n\r\n");
        assert_eq!(p.not().trim_start("\n\r\n"), "\n\r\n");
        assert_eq!(p.not().trim_end("\n\r\n"), "\n\r\n");
        assert_eq!(p.not().trim("\n\r\n"), "\n\r\n");
        assert_eq!(p.not().trim_start(" \n\r\n xyz"), "\n\r\n xyz");
        assert_eq!(p.not().trim_end(" \n\r\n xyz"), " \n\r\n");
        assert_eq!(p.not().trim(" \n\r\n xyz"), "\n\r\n");

        assert_eq!(CharSet::except(p, p.not()).trim_start(" \n "), " \n ");
        assert_eq!(CharSet::except(p, p.not()).trim_end(" \n "), " \n ");
        assert_eq!(CharSet::except(p, p.not()).trim(" \n "), " \n ");

        assert_eq!(CharSet::except(p.not(), p).trim_start(" \n "), "\n ");
        assert_eq!(CharSet::except(p.not(), p).trim_end(" \n "), " \n");
        assert_eq!(CharSet::except(p.not(), p).trim(" \n "), "\n");
        assert_eq!(
            CharSet::except(p.not(), p).find_first_not_matching(" "),
            None,
        );
        assert_eq!(
            CharSet::except(p.not(), p).find_first_not_matching("\n"),
            Some(0),
        );
        assert_eq!(
            CharSet::except(p.not(), p).find_first_not_matching(""),
            None,
        );

        assert!(!p.clone().step(' '.into()));
        assert!(!p.clone().step('a'.into()));
        assert!(p.clone().step('\r'.into()));
        assert!(p.clone().step('\n'.into()));

        assert_eq!(p.state(), State::Stoppable);

        assert_eq!(
            p.forward::<E>(" \n\r\n"),
            Err(PatternError::UnexpectedChar(' '.into()))
        );
        assert_eq!(p.forward::<E>("\r\nabc\n"), Ok(2));
        assert_eq!(p.forward::<E>("\n\r\n"), Ok(3));
        assert_eq!(
            p.forward::<E>("+-*/"),
            Err(PatternError::UnexpectedChar('+'.into()))
        );
        assert_eq!(
            p.forward::<E>(""),
            Err(PatternError::UnexpectedChar("".into()))
        );
    }

    #[test]
    fn forward_char() {
        test_forward::<char>();
    }

    #[test]
    fn forward_fixed_utf8_char() {
        test_forward::<FixedUtf8Char>();
    }
}
