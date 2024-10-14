use super::{as_slice_from, err_eof, error::StreamError, traits::BufReadExt};
use std::io::BufRead;

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
    #[inline]
    pub const fn new(buffer: B) -> Self {
        let line_buf = String::new();
        let cursor = 0;
        Self {
            buffer,
            line_buf,
            cursor,
        }
    }
}

impl<B: BufRead> BufReadExt<char> for InputStream<B> {
    #[inline]
    fn get_cur_line(&self) -> &str {
        as_slice_from(&self.line_buf, self.cursor)
    }
    #[inline]
    unsafe fn skip(&mut self, n: usize) {
        self.cursor += n;
        debug_assert!(self.line_buf.is_char_boundary(self.cursor));
    }
    #[inline]
    fn read_buf(&mut self) -> Result<bool, StreamError> {
        self.line_buf.clear();
        self.cursor = 0;
        let i = self.buffer.read_line(&mut self.line_buf)?;
        debug_assert!(self.line_buf.is_char_boundary(self.cursor));
        Ok(i > 0)
    }
    #[inline]
    fn fill_buf(&mut self) -> Result<(), StreamError> {
        if self.read_buf()? {
            Ok(())
        } else {
            Err(err_eof())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        fmt::{Default, Format},
        stream::{error::StreamError, input_stream::InputStream},
        BufReadExt,
    };
    use std::io::Cursor;

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
        assert!(matches!(stream.try_get().unwrap_err(), StreamError::Eof),);
        assert_eq!(
            stream.try_get().unwrap_err().to_string(),
            StreamError::Eof.to_string(),
        );
    }

    #[test]
    fn try_get_string() {
        let s = "Hello, world!\nHello, Rust!";
        let mut stream = InputStream::new(Cursor::new(s));
        assert_eq!(
            stream.try_get_string_some(Default.skipped_chars()).unwrap(),
            "Hello,"
        );
        assert_eq!(
            stream.try_get_string_some(Default.skipped_chars()).unwrap(),
            "world!"
        );
        assert_eq!(
            stream.try_get_string_some(Default.skipped_chars()).unwrap(),
            "Hello,"
        );
        assert_eq!(
            stream.try_get_string_some(Default.skipped_chars()).unwrap(),
            "Rust!"
        );
        assert!(matches!(
            stream
                .try_get_string_some(Default.skipped_chars())
                .unwrap_err(),
            StreamError::Eof
        ),);
        assert_eq!(
            stream.try_get().unwrap_err().to_string(),
            StreamError::Eof.to_string(),
        );
    }
}
