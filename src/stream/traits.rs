use super::{error::StreamError, is_eol, CHAR_EOL};
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

fn trim_start_matches<'s>(s: &'s str, white: &[FixedUtf8Char]) -> &'s str {
    let mut line = s;
    while let Some(c) = white.iter().find(|c| line.starts_with(c.as_str())) {
        let cursor = c.len();
        debug_assert!(line.is_char_boundary(cursor));
        line = unsafe { line.get_unchecked(cursor..) };
    }
    line
}

fn trim_matches<'s>(s: &'s str, white: &[FixedUtf8Char]) -> &'s str {
    trim_end_matches(trim_start_matches(s, white), white)
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
    /// Get the current line whatever state it is.
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
    #[must_use = "This method returns whether a new line is read, and should be checked."]
    fn read_buf(&mut self) -> Result<bool, StreamError>;

    /// Check whether is at the end of the line.
    #[inline]
    fn is_eol(&self) -> bool {
        self.get_cur_line().is_empty()
    }

    /// Get the current line. Read a new line if current line is empty.
    fn get_line(&mut self) -> Result<&str, StreamError> {
        let _: bool = self.fill_buf_if_eol()?;
        let line: &str = self.get_cur_line();
        let line: &str = unsafe { transmute(line) };
        Ok(line)
    }

    /// Get the next character in current line, if any.
    #[inline]
    fn get_in_cur_line(&mut self) -> Result<Option<char>, StreamError> {
        if let Some(c) = self.get_line()?.chars().next() {
            unsafe { self.skip(c.len_utf8()) };
            Ok(Some(c))
        } else {
            Ok(None)
        }
    }

    /// Get the next character in current line, if any.
    #[inline]
    fn get_in_cur_line_utf8(&mut self) -> Result<Option<FixedUtf8Char>, StreamError> {
        if let Some(c) = FixedUtf8Char::from_first_char(self.get_line()?) {
            unsafe { self.skip(c.len()) };
            Ok(Some(c))
        } else {
            Ok(None)
        }
    }

    /// Get the next character in current line, if any.
    #[inline]
    fn peek_in_cur_line(&mut self) -> Result<Option<char>, StreamError> {
        let line = self.get_line()?;
        Ok(line.chars().next())
    }

    /// Get the next character in current line, if any.
    #[inline]
    fn peek_in_cur_line_utf8(&self) -> Result<Option<FixedUtf8Char>, StreamError> {
        Ok(FixedUtf8Char::from_first_char(self.get_cur_line()))
    }

    /// Fill the buffer with a new line, ignoring the current line.
    ///
    /// - Returns `Ok(())` if a new line is read.
    /// - Returns `Err` if the buffer cannot be filled with a new line.
    fn fill_buf(&mut self) -> Result<(), StreamError>;

    /// Fill the buffer with a new line if the current line is empty.
    ///
    /// - Returns `Ok(true)` if a new line is read.
    /// - Returns `Ok(false)` if the current line is not empty.
    /// - Returns `Err` if the buffer cannot be filled with a new line.
    fn fill_buf_if_eol(&mut self) -> Result<bool, StreamError> {
        if self.is_eol() {
            self.fill_buf()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get a single character.
    #[inline]
    fn try_get(&mut self) -> Result<char, StreamError> {
        loop {
            if let Some(c) = self.get_in_cur_line()? {
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
            if let Some(c) = self.peek_in_cur_line()? {
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
            if let Some(c) = self.peek_in_cur_line_utf8()? {
                if pattern.contains(&c) {
                    unsafe { self.skip(c.len()) };
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
            if let Some(c) = self.get_in_cur_line_utf8()? {
                if !skipped.contains(&c) {
                    return Ok(From::from(c));
                }
            } else {
                self.fill_buf()?;
            }
        }
    }

    /// Skip all `skipped` characters until a non-`skipped` character is found or end of line.
    #[inline]
    fn try_skip_all(&mut self, skipped: &[FixedUtf8Char]) -> Result<usize, StreamError> {
        let mut count = 0;
        'lines: loop {
            let line = self.get_cur_line();
            let mut cursor = 0usize;
            for c in IterFixedUtf8Char::new(line) {
                if skipped.contains(&c) {
                    count += c.len();
                    cursor += c.len();
                } else {
                    unsafe { self.skip(cursor) };
                    break 'lines;
                }
            }
            unsafe { self.skip(cursor) };
            if !self.read_buf()? {
                break;
            }
        }
        Ok(count)
    }

    /// Skip a single character.
    #[inline]
    fn try_skip_any(&mut self) -> Result<(), StreamError> {
        self.try_get().map(|_| ())
    }

    /// Go to the next line if the remaining part are end of line characters.
    ///
    /// - Returns `Ok(Some(true))` if a new line is read.
    /// - Returns `Ok(Some(false))` if the current line is empty, but can't read a new line.
    /// - Returns `Ok(None)` if current line is not empty.
    #[inline]
    #[must_use = "This method returns whether a new line is read, and should be checked."]
    fn try_skip_eol(&mut self) -> Result<Option<bool>, StreamError> {
        let _: bool = self.fill_buf_if_eol()?;
        let mut count = 0usize;
        for c in IterFixedUtf8Char::new(self.get_cur_line()) {
            if !is_eol(c) {
                break;
            }
            count += c.len();
        }
        unsafe { self.skip(count) };
        if self.is_eol() {
            return Ok(Some(self.read_buf()?));
        }
        Ok(None)
    }

    /// Read until a character in `pattern` is found or end of line.
    #[inline]
    fn try_get_until_in_line(&mut self, pattern: &[FixedUtf8Char]) -> Result<&str, StreamError> {
        let line = self.get_line()?;
        let mut cursor = 0;
        for c in IterFixedUtf8Char::new(line) {
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

    /// Get a single line. The trailing newline will be consumed and trimmed. but no other white spaces will be trimmed.
    ///
    /// It can returns an empty string.
    #[inline]
    fn try_get_line(&mut self) -> Result<&str, StreamError> {
        let _: bool = self.fill_buf_if_eol()?;
        let line = self.try_get_until_in_line(&[])?;
        Ok(line.trim_end_matches(CHAR_EOL))
    }

    /// Get a single line. The trailing white spaces will be consumed and trimmed.
    ///
    /// It can returns an empty string.
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
            self.fill_buf()?;
        }
    }

    /// Get a single not-empty line. Both leading and trailing white spaces will be consumed and trimmed.
    ///
    /// Repeatedly read a new line if current line is empty.
    #[inline]
    fn try_get_line_some_trimmed(&mut self, white: &[FixedUtf8Char]) -> Result<&str, StreamError> {
        loop {
            let line = self.try_get_line_some()?;
            let line: &str = trim_matches(line, white);
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
