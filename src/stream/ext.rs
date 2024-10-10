//! Extensions for characters and strings.

use super::{CR, LF};
use crate::utf8char::{FixedUtf8Char, IterFixedUtf8Char};

/// Extension traits for characters.
pub trait CharExt {
    /// Get the length of the character in UTF-8.
    fn len_utf8(&self) -> usize;
}

impl CharExt for FixedUtf8Char {
    fn len_utf8(&self) -> usize {
        Self::len_utf8(self)
    }
}

impl CharExt for char {
    fn len_utf8(&self) -> usize {
        Self::len_utf8(*self)
    }
}

/// Extension traits for strings.
pub trait StrExt<'s, C: CharExt> {
    /// An iterator over characters.
    type Iterator: Iterator<Item = C>;

    /// Get an iterator over characters.
    fn chars_ext(self) -> Self::Iterator;

    /// Get the first character.
    fn first_char(self) -> Option<C>;
}

impl<'s> StrExt<'s, FixedUtf8Char> for &'s str {
    type Iterator = IterFixedUtf8Char<'s>;

    fn chars_ext(self) -> Self::Iterator {
        IterFixedUtf8Char::new(self)
    }

    fn first_char(self) -> Option<FixedUtf8Char> {
        FixedUtf8Char::from_first_char(self)
    }
}

impl<'s> StrExt<'s, char> for &'s str {
    type Iterator = std::str::Chars<'s>;

    fn chars_ext(self) -> Self::Iterator {
        self.chars()
    }

    fn first_char(self) -> Option<char> {
        self.chars().next()
    }
}

/// Extension trait for patterns.
pub trait Pattern: Sized {
    /// The item type.
    type Item: CharExt;

    /// End of line characters.
    ///
    /// Represents the characters `'\n'` and `'\r'`.
    const EOL: [Self::Item; 2];

    /// Check whether the character matches the pattern.
    fn matches(&self, c: Self::Item) -> bool;

    /// Trim the start of the string.
    fn trim_start(self, s: &str) -> &str;

    /// Trim the end of the string.
    fn trim_end(self, s: &str) -> &str;

    /// Trim the string.
    fn trim(self, s: &str) -> &str;

    /// Find the first matching character.
    fn find_first_matching(self, s: &str) -> Option<usize>;

    /// Find the first matching character or the whole length.
    fn find_first_matching_or_whole_length(self, s: &str) -> usize {
        self.find_first_matching(s).unwrap_or(s.len())
    }

    /// Find the first not matching character.
    fn find_first_not_matching(self, s: &str) -> Option<usize>;

    /// Find the first not matching character or the whole length.
    fn find_first_not_matching_or_whole_length(self, s: &str) -> usize {
        self.find_first_not_matching(s).unwrap_or(s.len())
    }
}

impl Pattern for &[FixedUtf8Char] {
    type Item = FixedUtf8Char;

    const EOL: [Self::Item; 2] = [LF, CR];

    fn matches(&self, c: Self::Item) -> bool {
        self.contains(&c)
    }

    fn trim_end(self, s: &str) -> &str {
        let mut line = s;
        while let Some(c) = self.iter().find(|&&c| line.ends_with(c.as_str())) {
            let cursor = line.len() - c.len_utf8();
            debug_assert!(line.is_char_boundary(cursor));
            line = unsafe { line.get_unchecked(..cursor) };
        }
        line
    }

    fn trim_start(self, s: &str) -> &str {
        let mut line = s;
        while let Some(c) = self.iter().find(|&&c| line.starts_with(c.as_str())) {
            let cursor = c.len_utf8();
            debug_assert!(line.is_char_boundary(cursor));
            line = unsafe { line.get_unchecked(cursor..) };
        }
        line
    }

    fn trim(self, s: &str) -> &str {
        self.trim_end(self.trim_start(s))
    }

    fn find_first_matching(self, s: &str) -> Option<usize> {
        let mut cursor = 0;
        for c in <&str as StrExt<FixedUtf8Char>>::chars_ext(s) {
            if self.contains(&c) {
                return Some(cursor);
            }
            cursor += c.len_utf8();
        }
        None
    }

    fn find_first_not_matching(self, s: &str) -> Option<usize> {
        let mut cursor = 0;
        for c in <&str as StrExt<FixedUtf8Char>>::chars_ext(s) {
            if !self.contains(&c) {
                return Some(cursor);
            }
            cursor += c.len_utf8();
        }
        None
    }
}

impl Pattern for &[char] {
    type Item = char;

    const EOL: [Self::Item; 2] = ['\n', '\r'];

    fn matches(&self, c: Self::Item) -> bool {
        self.contains(&c)
    }

    fn trim_start(self, s: &str) -> &str {
        s.trim_start_matches(self)
    }

    fn trim_end(self, s: &str) -> &str {
        s.trim_end_matches(self)
    }

    fn trim(self, s: &str) -> &str {
        s.trim_matches(self)
    }

    fn find_first_matching(self, s: &str) -> Option<usize> {
        s.find(self)
    }

    fn find_first_not_matching(self, s: &str) -> Option<usize> {
        let l = s.trim_start_matches(self).len();
        if l == 0 {
            None
        } else {
            Some(s.len() - l)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Pattern;

    #[test]
    fn chars() {
        let ws = <&[char] as Pattern>::EOL.as_slice();
        assert!(ws.matches('\n'));
        assert!(ws.matches('\r'));
        assert!(!ws.matches(' '));
        assert!(!ws.matches('a'));

        let s = " \n\r\n";
        assert_eq!(ws.trim_start(s), " \n\r\n");
        assert_eq!(ws.trim_end(s), " ");
        assert_eq!(ws.trim(s), " ");

        assert_eq!(ws.find_first_matching(s), Some(1));
        assert_eq!(ws.find_first_not_matching(s), Some(0));
    }
}
