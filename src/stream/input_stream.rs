use crate::stream::WHITE;

use super::{as_slice_from, as_slice_to, err_eof, error::StreamError, is_eol, traits::BufReadExt};
use std::{io::BufRead, mem::transmute};

/// C++-like Stream.
///
/// This struct provides a way to read from a buffer that implements [BufRead].
///
/// It implements [BufReadExt] for reading characters, strings, and lines.
pub struct InputStream<B> {
    buffer: B,
    line_buf: String,
    cursor: usize,
}

impl<B: BufRead> InputStream<B> {
    /// Create an input stream from a buffer that implements [BufRead].
    pub const fn new(buffer: B) -> Self {
        let line_buf = String::new();
        let cursor = 0;
        Self {
            buffer,
            line_buf,
            cursor,
        }
    }
    /// Check whether is at the end of the line.
    #[inline]
    pub fn is_eol(&self) -> bool {
        self.cursor == self.line_buf.len()
    }
}

impl<B: BufRead> InputStream<B> {
    /// Try to fill the buffer with a new line.
    ///
    /// # Errors
    ///
    /// If [BufRead::read_line] returns an error.
    #[inline]
    fn read_buf(&mut self) -> Result<bool, StreamError> {
        self.line_buf.clear();
        self.cursor = 0;
        let i = self.buffer.read_line(&mut self.line_buf)?;
        debug_assert!(self.line_buf.is_char_boundary(self.cursor));
        Ok(i > 0)
    }
    /// Fill the buffer with a new line.
    ///
    /// # Errors
    ///
    /// If [BufRead::read_line] returns an error, or the buffer is empty.
    #[inline]
    fn fill_buf(&mut self) -> Result<(), StreamError> {
        if self.read_buf()? {
            Ok(())
        } else {
            Err(err_eof())
        }
    }
    /// [Self::fill_buf] if is at the end of the line.
    #[inline]
    fn fill_buf_if_eol(&mut self) -> Result<(), StreamError> {
        if self.is_eol() {
            self.fill_buf()?;
        }
        Ok(())
    }
    /// Remove leading white spaces, and return the remaining string.
    ///
    /// If the line is empty or all white spaces, it will read a new line.
    ///
    /// # Errors
    ///
    /// See [Self::fill_buf].
    #[inline]
    fn remove_leading(&mut self, pattern: &[char]) -> Result<(&str, usize), StreamError> {
        let mut len_skipped = 0;
        loop {
            if self.is_eol() {
                let read = self.read_buf()?;
                if !read {
                    return Ok(("", len_skipped));
                }
            }
            let remaining = as_slice_from(&self.line_buf, self.cursor);
            let remaining_trimmed = remaining.trim_start_matches(pattern);
            len_skipped += remaining.len() - remaining_trimmed.len();
            self.cursor = self.line_buf.len() - remaining_trimmed.len();
            debug_assert!(self.line_buf.is_char_boundary(self.cursor));
            if self.is_eol() {
                continue;
            }
            let remaining: &str = unsafe { transmute(remaining_trimmed) };
            return Ok((remaining, len_skipped));
        }
    }
}

impl<B: BufRead> BufReadExt for InputStream<B> {
    fn try_get(&mut self) -> Result<char, StreamError> {
        loop {
            if let Some(c) = as_slice_from(&self.line_buf, self.cursor).chars().next() {
                self.cursor += c.len_utf8();
                debug_assert!(self.line_buf.is_char_boundary(self.cursor));
                return Ok(c);
            } else {
                self.fill_buf()?;
            }
        }
    }

    fn try_peek(&mut self) -> Result<char, StreamError> {
        loop {
            if let Some(c) = as_slice_from(&self.line_buf, self.cursor).chars().next() {
                return Ok(c);
            } else {
                self.fill_buf()?;
            }
        }
    }

    fn try_get_if(&mut self, pattern: &[char]) -> Result<Option<char>, StreamError> {
        loop {
            if let Some(c) = as_slice_from(&self.line_buf, self.cursor).chars().next() {
                if pattern.contains(&c) {
                    self.cursor += c.len_utf8();
                    debug_assert!(self.line_buf.is_char_boundary(self.cursor));
                    return Ok(Some(c));
                } else {
                    return Ok(None);
                }
            } else {
                self.fill_buf()?;
            }
        }
    }

    fn try_skip_eol(&mut self) -> Result<(), StreamError> {
        self.fill_buf_if_eol()?;
        let remaining = as_slice_from(&self.line_buf, self.cursor);
        for c in remaining.chars() {
            if is_eol(c) {
                self.cursor += c.len_utf8();
                debug_assert!(self.line_buf.is_char_boundary(self.cursor));
            } else {
                break;
            }
        }
        Ok(())
    }

    fn try_skip_all(&mut self, skipped: &[char]) -> Result<usize, StreamError> {
        let mut count = 0;
        loop {
            let (_, len) = self.remove_leading(skipped)?;
            count += len;
            if len == 0 {
                return Ok(count);
            }
        }
    }

    fn try_get_until_in_line(&mut self, pattern: &[char]) -> Result<&str, StreamError> {
        self.fill_buf_if_eol()?;
        let s = as_slice_from(&self.line_buf, self.cursor);
        let i = s.find(pattern).unwrap_or(s.len());
        let frag: &str = as_slice_to(s, i);
        let frag: &str = unsafe { transmute(frag) };
        self.cursor += i;
        Ok(frag)
    }

    fn try_get_string_some(&mut self) -> Result<&str, StreamError> {
        let (remaining, _) = self.remove_leading(&WHITE)?;
        let i = remaining.find(WHITE).unwrap_or(remaining.len());
        let s = as_slice_to(remaining, i);
        if s.is_empty() {
            Err(err_eof())?
        }
        let s: &str = unsafe { transmute(s) };
        self.cursor += i;
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{
        stream::{error::StreamError, input_stream::InputStream},
        BufReadExt,
    };

    #[test]
    fn try_get() {
        let s = "Hello, world!\nHello, Rust!";
        let mut stream = InputStream::new(Cursor::new(s));
        assert_eq!(stream.try_get().unwrap(), 'H');
        assert_eq!(stream.try_get().unwrap(), 'e');
        assert_eq!(stream.try_get().unwrap(), 'l');
        assert_eq!(stream.try_get().unwrap(), 'l');
        assert_eq!(stream.try_get().unwrap(), 'o');
        assert_eq!(stream.try_get().unwrap(), ',');
        assert_eq!(stream.try_get().unwrap(), ' ');
        assert_eq!(stream.try_get().unwrap(), 'w');
        assert_eq!(stream.try_get().unwrap(), 'o');
        assert_eq!(stream.try_get().unwrap(), 'r');
        assert_eq!(stream.try_get().unwrap(), 'l');
        assert_eq!(stream.try_get().unwrap(), 'd');
        assert_eq!(stream.try_get().unwrap(), '!');
        assert_eq!(stream.try_get().unwrap(), '\n');
        assert_eq!(stream.try_get().unwrap(), 'H');
        assert_eq!(stream.try_get().unwrap(), 'e');
        assert_eq!(stream.try_get().unwrap(), 'l');
        assert_eq!(stream.try_get().unwrap(), 'l');
        assert_eq!(stream.try_get().unwrap(), 'o');
        assert_eq!(stream.try_get().unwrap(), ',');
        assert_eq!(stream.try_get().unwrap(), ' ');
        assert_eq!(stream.try_get().unwrap(), 'R');
        assert_eq!(stream.try_get().unwrap(), 'u');
        assert_eq!(stream.try_get().unwrap(), 's');
        assert_eq!(stream.try_get().unwrap(), 't');
        assert_eq!(stream.try_get().unwrap(), '!');
        assert!(
            matches!(stream.try_get().unwrap_err(), StreamError::Eof),
            "{:?}",
            stream.try_get_string_some(),
        );
        assert_eq!(
            stream.try_get().unwrap_err().to_string(),
            StreamError::Eof.to_string(),
        );
    }

    #[test]
    fn try_get_string() {
        let s = "Hello, world!\nHello, Rust!";
        let mut stream = InputStream::new(Cursor::new(s));
        assert_eq!(stream.try_get_string_some().unwrap(), "Hello,");
        assert_eq!(stream.try_get_string_some().unwrap(), "world!");
        assert_eq!(stream.try_get_string_some().unwrap(), "Hello,");
        assert_eq!(stream.try_get_string_some().unwrap(), "Rust!");
        assert!(
            matches!(stream.try_get_string_some().unwrap_err(), StreamError::Eof),
            "{:?}",
            stream.try_get_string_some()
        );
        assert_eq!(
            stream.try_get().unwrap_err().to_string(),
            StreamError::Eof.to_string(),
        );
    }
}
