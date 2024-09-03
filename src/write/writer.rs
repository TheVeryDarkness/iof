use super::WriteInto;
use crate::stdout;
use std::io::{self, Write};

/// Configuration for the writer.
pub struct Writer<'sep, 'end, 'buf> {
    index: usize,
    sep: &'sep str,
    end: &'end str,
    buf: Option<&'buf mut dyn Write>,
}

impl<'sep, 'end, 'buf> Default for Writer<'sep, 'end, 'buf> {
    fn default() -> Self {
        Self {
            index: 0,
            sep: " ",
            end: "\n",
            buf: None,
        }
    }
}

impl<'sep, 'end, 'buf> Writer<'sep, 'end, 'buf> {
    /// Create a new writer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the separator.  
    pub fn sep(&mut self, sep: &'sep str) -> &mut Self {
        self.sep = sep;
        self
    }
    /// Set the end.
    pub fn end(&mut self, end: &'end str) -> &mut Self {
        self.end = end;
        self
    }
    /// Set the buffer.
    pub fn buf(&mut self, buf: &'buf mut impl Write) -> &mut Self {
        self.buf = Some(buf);
        self
    }
    /// Write a value.
    pub fn write<V: WriteInto>(&mut self, value: &V) -> io::Result<&mut Self> {
        match self.buf.as_mut() {
            Some(buf) => {
                if self.index > 0 {
                    self.sep.try_write_into(buf)?;
                }
                value.try_write_into(buf)?;
            }
            None => {
                let buf = &mut stdout();
                if self.index > 0 {
                    self.sep.try_write_into(buf)?;
                }
                value.try_write_into(buf)?;
            }
        }
        self.index += 1;
        Ok(self)
    }
    /// Finish writing.
    pub fn finish(&mut self) -> io::Result<()> {
        if let Some(buf) = self.buf.as_mut() {
            self.end.try_write_into(buf)
        } else {
            let buf = &mut stdout();
            self.end.try_write_into(buf)
        }
    }
}
