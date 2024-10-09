use super::{error::StreamError, is_eol, STR_EOL};
use crate::utf8char::{iter_fixed::IterFixedUtf8Char, FixedUtf8Char};
use std::mem::transmute;

fn trim_end_matches<'s>(s: &'s str, white: &[FixedUtf8Char]) -> &'s str {
    let mut line = s;
    while let Some(c) = white.iter().find(|c| line.ends_with(c.as_str())) {
        let cursor = line.len() - c.len();
        debug_assert!(line.is_char_boundary(cursor));
        line = unsafe { line.get_unchecked(..cursor) };
    }
    line
}

/// Extension trait for [BufRead].
///
/// It provides a way to read:
///
/// - A single non-ASCII-whitespace character ([BufReadExt::try_get_non_ws]),
/// - A single ASCII-white-space-separated string ([BufReadExt::try_get_string_some]),
/// - A single non-empty line ([BufReadExt::try_get_line_some]),
/// - Or just the remained line ([BufReadExt::try_get_line]).
///
/// ASCII whitespace characters here are `' '`, `'\t'`, `'\n'`, and `'\r'`.
///
/// [BufRead]: std::io::BufRead
pub trait BufReadExt {
    /// Get the current line.
    fn get_cur_line(&self) -> &str;

    /// Skip `n` bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `n` is a valid UTF-8 character boundary.
    unsafe fn skip(&mut self, n: usize);

    /// Try to fill the buffer with a new line, ignoring the current line.
    ///
    /// Returns `true` if a new line is read.
    fn read_buf(&mut self) -> Result<bool, StreamError>;

    /// Get the next character in current line, if any.
    #[inline]
    fn get_in_cur_line(&mut self) -> Option<char> {
        if let Some(c) = self.get_cur_line().chars().next() {
            unsafe { self.skip(c.len_utf8()) };
            Some(c)
        } else {
            None
        }
    }

    /// Get the next character in current line, if any.
    #[inline]
    fn get_in_cur_line_utf8(&mut self) -> Option<FixedUtf8Char> {
        if let Some(c) = FixedUtf8Char::from_first_char(self.get_cur_line()) {
            unsafe { self.skip(c.len()) };
            Some(c)
        } else {
            None
        }
    }

    /// Get the next character in current line, if any.
    #[inline]
    fn peek_in_cur_line(&self) -> Option<char> {
        let line = self.get_cur_line();
        line.chars().next()
    }

    /// Get the next character in current line, if any.
    #[inline]
    fn peek_in_cur_line_utf8(&self) -> Option<FixedUtf8Char> {
        FixedUtf8Char::from_first_char(self.get_cur_line())
    }

    /// Fill the buffer with a new line, ignoring the current line.
    ///
    /// Returns `Ok(())` if a new line is read.
    fn fill_buf(&mut self) -> Result<(), StreamError>;

    /// Get a single character.
    #[inline]
    fn try_get(&mut self) -> Result<char, StreamError> {
        loop {
            if let Some(c) = self.get_in_cur_line() {
                return Ok(c);
            } else {
                self.fill_buf()?;
            }
        }
    }

    /// Peek a single character.
    #[inline]
    fn try_peek(&mut self) -> Result<char, StreamError> {
        loop {
            if let Some(c) = self.peek_in_cur_line() {
                return Ok(c);
            } else {
                self.fill_buf()?;
            }
        }
    }

    /// Get a single character if it is in `pattern`, otherwise leave it in the buffer.
    #[inline]
    fn try_get_if(&mut self, pattern: &[FixedUtf8Char]) -> Result<Option<char>, StreamError> {
        loop {
            if let Some(c) = self.peek_in_cur_line_utf8() {
                if pattern.contains(&c) {
                    return Ok(Some(From::from(c)));
                } else {
                    return Ok(None);
                }
            } else {
                self.fill_buf()?;
            }
        }
    }

    /// Get a single non-`skipped` character.
    #[inline]
    fn try_get_non(&mut self, skipped: &[FixedUtf8Char]) -> Result<char, StreamError> {
        loop {
            if let Some(c) = self.get_in_cur_line_utf8() {
                if skipped.contains(&c) {
                    return Ok(From::from(c));
                }
            } else {
                self.fill_buf()?;
            }
        }
    }

    /// Skip all `skipped` characters.
    #[inline]
    fn try_skip_all(&mut self, skipped: &[FixedUtf8Char]) -> Result<usize, StreamError> {
        let mut count = 0;
        loop {
            if let Some(c) = self.get_in_cur_line_utf8() {
                if skipped.contains(&c) {
                    count += c.len();
                } else {
                    return Ok(count);
                }
            } else {
                self.fill_buf()?;
            }
        }
    }

    /// Skip a single character.
    #[inline]
    fn try_skip_any(&mut self) -> Result<(), StreamError> {
        self.try_get().map(|_| ())
    }

    /// Go to the next line if the remaining part are end of line characters.
    ///
    /// Only skip the first end of line character.
    #[inline]
    fn try_skip_eol(&mut self) -> Result<(), StreamError> {
        loop {
            if let Some(c) = self.get_in_cur_line_utf8() {
                if is_eol(c) {
                    return Ok(());
                } else {
                    return Err(StreamError::Eol);
                }
            } else {
                self.fill_buf()?;
            }
        }
    }

    /// Read until a character in `pattern` is found or end of line.
    #[inline]
    fn try_get_until_in_line(&mut self, pattern: &[FixedUtf8Char]) -> Result<&str, StreamError> {
        let line = self.get_cur_line();
        let mut cursor = 0;
        for c in IterFixedUtf8Char::new(line.as_bytes()) {
            if pattern.contains(&c) {
                break;
            }
            cursor += c.len();
        }
        let selected: &str = line.get(0..cursor).unwrap_or_default();
        let selected: &str = unsafe { transmute(selected) };
        unsafe { self.skip(cursor) };
        Ok(selected)
    }

    /// Get a single `skipped`-separated string.
    /// If current line is empty or all `skipped`, it will read a new line.
    #[inline]
    fn try_get_string_some(&mut self, skipped: &[FixedUtf8Char]) -> Result<&str, StreamError> {
        let _ = self.try_skip_all(skipped)?;
        self.try_get_until_in_line(skipped)
    }

    /// Get a single line. The trailing newline will be consumed and trimmed.
    #[inline]
    fn try_get_line(&mut self) -> Result<&str, StreamError> {
        let line = self.try_get_until_in_line(&[])?;
        Ok(line.trim_end_matches(STR_EOL))
    }

    /// Get a single line. The trailing white spaces will be consumed and trimmed.
    #[inline]
    fn try_get_line_trimmed(&mut self, white: &[FixedUtf8Char]) -> Result<&str, StreamError> {
        let line = self.try_get_line()?;
        Ok(trim_end_matches(line, white))
    }

    /// Get a single not-empty line. The trailing newline will be consumed and trimmed.
    ///
    /// Repeatedly read a new line if current line is empty.
    #[inline]
    fn try_get_line_some(&mut self) -> Result<&str, StreamError> {
        loop {
            let line: &str = self.try_get_line()?;
            let line: &str = unsafe { transmute(line) };
            if !line.is_empty() {
                return Ok(line);
            }
        }
    }

    /// Get a single not-empty line. The trailing white spaces will be consumed and trimmed.
    ///
    /// Repeatedly read a new line if current line is empty.
    #[inline]
    fn try_get_line_some_trimmed(&mut self, white: &[FixedUtf8Char]) -> Result<&str, StreamError> {
        loop {
            let line = self.try_get_line_some()?;
            let line: &str = trim_end_matches(line, white);
            let line: &str = unsafe { transmute(line) };
            if !line.is_empty() {
                return Ok(line);
            }
        }
    }
}

impl<S: ?Sized + BufReadExt> BufReadExt for &mut S {
    #[inline]
    fn get_cur_line(&self) -> &str {
        S::get_cur_line(self)
    }
    #[inline]
    unsafe fn skip(&mut self, n: usize) {
        S::skip(self, n)
    }
    #[inline]
    fn read_buf(&mut self) -> Result<bool, StreamError> {
        S::read_buf(self)
    }
    #[inline]
    fn fill_buf(&mut self) -> Result<(), StreamError> {
        S::fill_buf(self)
    }
}
