use std::{
    io::{BufRead, Error, ErrorKind},
    marker::PhantomData,
};

use crate::ReadInto;

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
        self.read_buf()?;
        Ok(result)
    }
}

/// Iterator for all elements.
pub struct RealAll<'s, S: ?Sized, T> {
    stream: &'s mut S,
    phantom: PhantomData<T>,
}

impl<'s, S: ?Sized, T> RealAll<'s, S, T> {
    pub(crate) fn new(stream: &'s mut S) -> Self {
        let phantom = PhantomData;
        Self { stream, phantom }
    }
}

impl<S: ReadInto<T> + ?Sized, T> Iterator for RealAll<'_, S, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.try_read().ok()
    }
}

/// A wrapper that converts [std::io::Write] into [std::fmt::Write].
pub struct OutputStream<W> {
    buffer: W,
}

impl<W> OutputStream<W> {
    /// Create an output stream from a buffer that implements [std::io::Write].
    pub fn new(buffer: W) -> Self {
        Self { buffer }
    }
}

impl<W: std::io::Write> std::fmt::Write for OutputStream<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer
            .write_all(s.as_bytes())
            .map_err(|_| std::fmt::Error)
    }
}
