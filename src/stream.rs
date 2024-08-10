use std::{
    io::{BufRead, Error, ErrorKind},
    marker::PhantomData,
};

use crate::ReadInto;

/// C++-like Stream.
pub struct InputStream<B> {
    buffer: B,
    line_buf: String,
    cursor: usize,
}

const WHITE: [char; 4] = [' ', '\t', '\n', '\r'];

impl<B> InputStream<B> {
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
impl<B: BufRead> InputStream<B> {
    fn remove_white(&mut self) -> Result<&str, Error> {
        while self.cursor == self.line_buf.len() {
            self.line_buf.clear();
            self.cursor = 0;
            let i = self.buffer.read_line(&mut self.line_buf)?;
            if i == 0 {
                return Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    "failed to read a non-whitespace character before EOF",
                ));
            }
        }
        // Assume we get correct encoding.
        let remaining = if cfg!(debug_assertions) {
            &self.line_buf[self.cursor..]
        } else {
            unsafe { self.line_buf.get_unchecked(self.cursor..) }
        };
        let remaining = remaining.trim_start_matches(WHITE);
        self.cursor = self.line_buf.len() - remaining.len();
        debug_assert!(self.line_buf.is_char_boundary(self.cursor));
        Ok(remaining)
    }
    /// Consume a charater that is not ' ', '\t' or '\n'.
    pub fn consume_char(&mut self) -> Result<char, Error> {
        loop {
            let remaining = self.remove_white()?;
            if let Some(c) = remaining.chars().next() {
                self.cursor += c.len_utf8();
                return Ok(c);
            }
        }
    }
    /// Consume some charaters until ' ', '\t' or '\n'.
    pub fn consume_string<T>(&mut self, f: impl FnOnce(&str) -> T) -> Result<T, Error> {
        loop {
            let remaining = self.remove_white()?;
            if remaining.is_empty() {
                continue;
            } else {
                let i = remaining.find(WHITE).unwrap_or(remaining.len());
                let frag = if cfg!(debug_assertions) {
                    &remaining[..i]
                } else {
                    unsafe { remaining.get_unchecked(..i) }
                };
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
    /// Consume the remained line.
    pub fn consume_remained_line<T>(&mut self, f: impl FnOnce(&str) -> T) -> Result<T, Error> {
        let line = &self.line_buf[self.cursor..];
        self.cursor = self.line_buf.len();
        return Ok(f(line));
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
