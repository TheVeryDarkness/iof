use std::{
    io::{BufRead, Error, ErrorKind},
    mem::transmute,
};

/// ASCII whitespace characters.
const WHITE: [char; 4] = [' ', '\t', '\n', '\r'];
/// End of line characters.
const EOL: [char; 2] = ['\n', '\r'];

/// Extension trait for [BufRead].
///
/// It provides a way to read:
///
/// - A single non-ASCII-whitespace character ([BufReadExt::try_get_non_ws]),
/// - A single ASCII-white-space-separated string ([BufReadExt::try_get_string]),
/// - A single non-empty line ([BufReadExt::try_get_line_some]),
/// - Just the remained line ([BufReadExt::try_get_line]),
/// - Or all ASCII-white-space-separated strings ([BufReadExt::try_get_all]).
///
/// ASCII whitespace characters are `' '`, `'\t'`, `'\n'`, and `'\r'`.
///
/// ASCII whitespace characters here are `' '`, `'\t'`, `'\n'`, and `'\r'`.
pub trait BufReadExt {
    /// Get a single character.
    fn try_get(&mut self) -> Result<char, Error>;
    /// Peek a single character.
    fn try_peek(&mut self) -> Result<char, Error>;

    /// Get a single character if it is in `pattern`.
    fn try_get_if(&mut self, pattern: &[char]) -> Result<Option<char>, Error> {
        let c = self.try_peek()?;
        if pattern.contains(&c) {
            self.try_skip_any()?;
            Ok(Some(c))
        } else {
            Ok(None)
        }
    }

    /// Get a single non-`skipped` character.
    #[inline]
    fn try_get_non(&mut self, skipped: &[char]) -> Result<char, Error> {
        loop {
            let c = self.try_get()?;
            if !skipped.contains(&c) {
                return Ok(c);
            }
        }
    }

    /// Get a single non-ASCII-whitespace character.
    #[inline]
    fn try_get_non_ws(&mut self) -> Result<char, Error> {
        self.try_get_non(&WHITE)
    }

    /// Skip a single character.
    #[inline]
    fn try_skip_any(&mut self) -> Result<(), Error> {
        self.try_get().map(|_| ())
    }

    /// Skip a single character if it is in `skipped`.
    ///
    /// Returns `true` if a character is skipped.
    #[inline]
    fn try_skip(&mut self, skipped: &[char]) -> Result<bool, Error> {
        self.try_get_if(skipped).map(|x| x.is_some())
    }

    /// Skip all characters in `skipped`.
    fn try_skip_all(&mut self, skipped: &[char]) -> Result<usize, Error>;

    /// Skip a single ASCII-whitespace character.
    #[inline]
    fn try_skip_ws(&mut self) -> Result<bool, Error> {
        self.try_skip(&WHITE)
    }

    /// Skip all ASCII-whitespace characters.
    #[inline]
    fn try_skip_all_ws(&mut self) -> Result<usize, Error> {
        self.try_skip_all(&WHITE)
    }

    /// Read until a character in `pattern` is found or end of line.
    fn try_get_until_in_line(&mut self, pattern: &[char]) -> Result<&str, Error>;

    /// Get a single ASCII-whitespace-separated string.
    /// It won't contain the trailing ASCII-whitespace characters.
    /// If current line is empty or all ASCII-whitespaces, it will read a new line.
    #[inline]
    fn try_get_string(&mut self) -> Result<&str, Error> {
        loop {
            let _len = self.try_skip_all(&WHITE)?;
            let s = self.try_get_until_in_line(&WHITE)?;
            let s: &str = s.trim_end_matches(WHITE);
            let s: &str = unsafe { transmute(s) };
            if !s.is_empty() {
                return Ok(s);
            }
        }
    }

    /// Get a single line. The trailing newline will be trimmed.
    #[inline]
    fn try_get_line(&mut self) -> Result<&str, Error> {
        let line = self.try_get_until_in_line(&[])?;
        Ok(line.trim_end_matches(EOL))
    }

    /// Get a single line. The trailing whitespaces will be trimmed.
    #[inline]
    fn try_get_line_trimmed(&mut self) -> Result<&str, Error> {
        let line = self.try_get_line()?;
        Ok(line.trim_end_matches(WHITE))
    }

    /// Get a single not-empty line. Repeat reading a new line if current line is empty.
    #[inline]
    fn try_get_line_some(&mut self) -> Result<&str, Error> {
        loop {
            let line: &str = self.try_get_line()?;
            let line: &str = unsafe { transmute(line) };
            if !line.is_empty() {
                return Ok(line);
            }
        }
    }

    /// Get a single not-empty line. The trailing whitespaces will be trimmed.
    #[inline]
    fn try_get_line_some_trimmed(&mut self) -> Result<&str, Error> {
        loop {
            let line = self.try_get_line_some()?.trim_end_matches(WHITE);
            let line: &str = unsafe { transmute(line) };
            if !line.is_empty() {
                return Ok(line);
            }
        }
    }

    /// Get all ASCII-whitespace-separated strings in current line.
    fn try_get_all_in_line(&mut self) -> Result<impl Iterator<Item = &str>, Error> {
        Ok(ReadAllIn::new(self.try_get_line()?))
    }
    /// Get all ASCII-whitespace-separated strings in next non-all-ASCII-whitespace line.
    fn try_get_all_in_line_some(&mut self) -> Result<impl Iterator<Item = &str>, Error> {
        Ok(ReadAllIn::new(self.try_get_line_some()?))
    }
    /// Get all ASCII-whitespace-separated strings in this stream.
    fn try_get_all(&mut self) -> impl Iterator<Item = &str> {
        RealAll::new(self)
    }
}

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

fn err_eof(msg: &'static str) -> Error {
    Error::new(ErrorKind::UnexpectedEof, msg)
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

#[inline]
fn as_slice_from(s: &str, i: usize) -> &str {
    // Assume we get correct encoding.
    debug_assert!(s.is_char_boundary(i));
    unsafe { s.get_unchecked(i..) }
}
#[inline]
fn as_slice_to(s: &str, i: usize) -> &str {
    // Assume we get correct encoding.
    debug_assert!(s.is_char_boundary(i));
    unsafe { s.get_unchecked(..i) }
}

const MSG_EOF: &str = "failed to read one more character before EOF";

impl<B: BufRead> InputStream<B> {
    /// Try to fill the buffer with a new line.
    ///
    /// # Errors
    ///
    /// If [BufRead::read_line] returns an error.
    #[inline]
    fn read_buf(&mut self) -> Result<bool, Error> {
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
    fn fill_buf(&mut self, msg: &'static str) -> Result<(), Error> {
        if self.read_buf()? {
            Ok(())
        } else {
            Err(err_eof(msg))
        }
    }
    /// [Self::fill_buf] if is at the end of the line.
    #[inline]
    fn fill_buf_if_eol(&mut self, msg: &'static str) -> Result<(), Error> {
        if self.is_eol() {
            self.fill_buf(msg)?;
        }
        Ok(())
    }
    /// Remove leading white spaces, and return the remaining string.
    ///
    /// If the line is empty or all whitespaces, it will read a new line.
    ///
    /// # Errors
    ///
    /// See [Self::fill_buf].
    #[inline]
    fn remove_leading(&mut self, pattern: &[char]) -> Result<(&str, usize), Error> {
        if self.is_eol() {
            let _read = self.read_buf()?;
        }
        let remaining = as_slice_from(&self.line_buf, self.cursor);
        let len_skipped = remaining.len();
        let remaining = remaining.trim_start_matches(pattern);
        let len_skipped = len_skipped - remaining.len();
        self.cursor = self.line_buf.len() - remaining.len();
        debug_assert!(self.line_buf.is_char_boundary(self.cursor));
        Ok((remaining, len_skipped))
    }
}

impl<B: BufRead> BufReadExt for InputStream<B> {
    fn try_get(&mut self) -> Result<char, Error> {
        loop {
            if let Some(c) = as_slice_from(&self.line_buf, self.cursor).chars().next() {
                self.cursor += c.len_utf8();
                debug_assert!(self.line_buf.is_char_boundary(self.cursor));
                return Ok(c);
            } else {
                self.fill_buf(MSG_EOF)?;
            }
        }
    }

    fn try_peek(&mut self) -> Result<char, Error> {
        loop {
            if let Some(c) = as_slice_from(&self.line_buf, self.cursor).chars().next() {
                return Ok(c);
            } else {
                self.fill_buf(MSG_EOF)?;
            }
        }
    }

    fn try_get_non_ws(&mut self) -> Result<char, Error> {
        loop {
            let (remaining, _len) = self.remove_leading(&WHITE)?;
            if let Some(c) = remaining.chars().next() {
                self.cursor += c.len_utf8();
                return Ok(c);
            } else {
                self.fill_buf(MSG_EOF)?;
            }
        }
    }

    fn try_skip_all(&mut self, skipped: &[char]) -> Result<usize, Error> {
        let mut count = 0;
        loop {
            let (_, len) = self.remove_leading(skipped)?;
            count += len;
            if len == 0 {
                return Ok(count);
            }
        }
    }

    fn try_get_until_in_line(&mut self, pattern: &[char]) -> Result<&str, Error> {
        self.fill_buf_if_eol(MSG_EOF)?;
        let s = as_slice_from(&self.line_buf, self.cursor);
        let i = s.find(pattern).unwrap_or(s.len());
        let frag: &str = as_slice_to(s, i);
        let frag: &str = unsafe { transmute(frag) };
        self.cursor += i;
        Ok(frag)
    }
}

/// Iterator for all elements.
pub struct RealAll<'s, S: ?Sized> {
    stream: &'s mut S,
}

impl<'s, S: ?Sized> RealAll<'s, S> {
    pub(crate) fn new(stream: &'s mut S) -> Self {
        Self { stream }
    }
}

impl<'s, S: BufReadExt + ?Sized> Iterator for RealAll<'s, S> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream
            .try_get_string()
            .ok()
            .map(|s| unsafe { transmute(s) })
    }
}

/// Iterator for all elements in a string.
#[derive(Default)]
struct ReadAllIn<'s> {
    buffer: &'s str,
}

impl<'s> ReadAllIn<'s> {
    pub(crate) fn new(buffer: &'s str) -> Self {
        Self { buffer }
    }
}

impl<'s> Iterator for ReadAllIn<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.buffer.trim_start_matches(WHITE);
        if remaining.is_empty() {
            return None;
        }
        let i = remaining.find(WHITE).unwrap_or(remaining.len());
        let frag = as_slice_to(remaining, i);
        self.buffer = as_slice_from(remaining, i);
        Some(frag)
    }
}
