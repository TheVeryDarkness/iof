use std::{
    io::{BufRead, Error, ErrorKind},
    mem::transmute,
};

/// C++-like Stream.
///
/// This struct provides a way to read from a buffer that implements [BufRead].
///
/// It provides a way to read:
///
/// - A single non-ASCII-whitespace character ([InputStream::consume_char]),
/// - A single ASCII-white-space-separated string ([InputStream::consume_string]),
/// - A single non-empty line ([InputStream::consume_line]),
/// - Just the remained line ([InputStream::consume_remained_line]),
/// - Or all ASCII-white-space-separated strings ([InputStream::read_all]).
///
/// ASCII whitespace characters are `' '`, `'\t'`, `'\n'`, and `'\r'`.
pub struct InputStream<B> {
    buffer: B,
    line_buf: String,
    cursor: usize,
}

const WHITE: [char; 4] = [' ', '\t', '\n', '\r'];
const EOL: [char; 2] = ['\n', '\r'];

fn err_eof(msg: &'static str) -> Error {
    Error::new(ErrorKind::UnexpectedEof, msg)
}

impl<B: BufRead> InputStream<B> {
    /// Create an input stream from a buffer that implements [BufRead].
    pub fn new(buffer: B) -> Self {
        let line_buf = String::new();
        let cursor = 0;
        Self {
            buffer,
            line_buf,
            cursor,
        }
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

const MSG_EOF: &str = "failed to read a non-whitespace character before EOF";

impl<B: BufRead> InputStream<B> {
    /// Fill the buffer with a new line.
    ///
    /// # Errors
    ///
    /// If [BufRead::read_line] returns an error.
    #[inline]
    fn fill_buf(&mut self, msg: &'static str) -> Result<(), Error> {
        self.line_buf.clear();
        self.cursor = 0;
        let i = self.buffer.read_line(&mut self.line_buf)?;
        if i == 0 {
            return Err(err_eof(msg));
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
    fn remove_white(&mut self) -> Result<&str, Error> {
        while self.cursor == self.line_buf.len() {
            self.fill_buf(MSG_EOF)?;
        }
        let remaining = as_slice_from(&self.line_buf, self.cursor);
        let remaining = remaining.trim_start_matches(WHITE);
        self.cursor = self.line_buf.len() - remaining.len();
        debug_assert!(self.line_buf.is_char_boundary(self.cursor));
        Ok(remaining)
    }
    /// Consume a charater that is not ' ', '\t', '\r' or '\n'.
    pub fn consume_char(&mut self) -> Result<char, Error> {
        loop {
            let remaining = self.remove_white()?;
            if let Some(c) = remaining.chars().next() {
                self.cursor += c.len_utf8();
                return Ok(c);
            }
        }
    }
    /// Consume some charaters until ' ', '\t', '\r' or '\n'.
    pub fn consume_string<T>(&mut self, f: impl FnOnce(&str) -> T) -> Result<T, Error> {
        loop {
            let remaining = self.remove_white()?;
            if remaining.is_empty() {
                continue;
            } else {
                let i = remaining.find(WHITE).unwrap_or(remaining.len());
                let frag = as_slice_to(remaining, i);
                let res = f(frag);
                self.cursor += i;
                return Ok(res);
            }
        }
    }
    /// Consume a line of non-empty string.
    pub fn consume_line<T>(&mut self, f: impl FnOnce(&str) -> T) -> Result<T, Error> {
        loop {
            let remaining = self.remove_white()?;
            if remaining.is_empty() {
                continue;
            } else {
                let line = &self.line_buf[self.cursor..].trim_end_matches(WHITE);
                self.cursor = self.line_buf.len();
                return Ok(f(line));
            }
        }
    }
    /// Return an [Iterator] that consumes all ASCII-white-space-separated strings in current line.
    pub fn consume_strings_in_line(&mut self) -> impl Iterator<Item = &str> {
        // You may wonder why we use `transmute` here, and whether it is safe.
        // Notice that `RealAllIn` is a struct with a lifetime parameter `'s` and a reference field `buffer`.
        // The lifetime parameter `'s` is the lifetime of the buffer, and the reference field `buffer` is a reference to the buffer.
        // So you won't be able to create a `RealAllIn` instance without a valid buffer,
        // or modify the buffer while the `RealAllIn` instance is alive.
        self.consume_line(|s| -> RealAllIn<'_> { unsafe { transmute(RealAllIn::new(s)) } })
            .unwrap_or_default()
    }
    /// Return an [Iterator] that consumes all ASCII-white-space-separated strings in current line.
    pub fn consume_strings_in_remained_line(&mut self) -> impl Iterator<Item = &str> {
        self.consume_remained_line(|s| -> RealAllIn<'_> { unsafe { transmute(RealAllIn::new(s)) } })
            .unwrap_or_default()
    }
    /// Return an [Iterator] that consumes all ASCII-white-space-separated strings in this buffer.
    pub fn consume_all<T, F: FnMut(&str) -> T>(&mut self, f: F) -> RealAll<'_, Self, T, F> {
        RealAll::new(self, f)
    }
    /// Try to fill the buffer.
    /// Won't return EOF error.
    #[inline]
    fn read_buf(&mut self) -> Result<(), Error> {
        self.line_buf.clear();
        self.cursor = 0;
        let _ = self.buffer.read_line(&mut self.line_buf)?;
        Ok(())
    }
    /// Consume the remained line without trailing CR or LF.
    ///
    /// Similar to `std::getline` in C++.
    pub fn consume_remained_line<T>(&mut self, f: impl FnOnce(&str) -> T) -> Result<T, Error> {
        if self.cursor == self.line_buf.len() {
            self.read_buf()?;
        }
        let line = &self.line_buf[self.cursor..];
        let result = f(line.trim_end_matches(EOL));
        self.cursor = self.line_buf.len();
        // self.read_buf()?;
        Ok(result)
    }
}

/// Iterator for all elements.
pub struct RealAll<'s, S: ?Sized, T, F: FnMut(&str) -> T> {
    stream: &'s mut S,
    f: F,
}

impl<'s, S, T, F: FnMut(&str) -> T> RealAll<'s, S, T, F> {
    pub(crate) fn new(stream: &'s mut S, f: F) -> Self {
        Self { stream, f }
    }
}

impl<'s, B: BufRead, T, F: FnMut(&str) -> T> Iterator for RealAll<'s, InputStream<B>, T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.consume_string(&mut self.f).ok()
    }
}

/// Iterator for all elements in a string.
#[derive(Default)]
pub(crate) struct RealAllIn<'s> {
    buffer: &'s str,
}

impl<'s> RealAllIn<'s> {
    pub(crate) fn new(buffer: &'s str) -> Self {
        Self { buffer }
    }
}

impl<'s> Iterator for RealAllIn<'s> {
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
