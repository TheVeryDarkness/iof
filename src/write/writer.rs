use super::WriteInto;
use std::io::{self, Write};

/// Configuration for the writer.
pub struct Writer<'sep, 'end> {
    sep: Option<&'sep [&'sep str]>,
    end: &'end str,
}

impl<'sep, 'end> Default for Writer<'sep, 'end> {
    fn default() -> Self {
        Self {
            sep: None,
            end: "\n",
        }
    }
}

impl<'sep, 'end> Writer<'sep, 'end> {
    /// Create a new writer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the separator.
    pub fn sep(&mut self, sep: &'sep [&'sep str]) -> &mut Self {
        self.sep = Some(sep);
        self
    }
    /// Set the end.
    pub fn end(&mut self, end: &'end str) -> &mut Self {
        self.end = end;
        self
    }
    /// Write a value.
    pub fn write<V: WriteInto + ?Sized>(
        &mut self,
        value: &V,
        buf: &mut impl Write,
    ) -> io::Result<()> {
        if let Some(sep) = self.sep {
            value.try_write_into_with_sep(buf, sep)?;
        } else {
            value.try_write_into(buf)?;
        }
        buf.write_all(self.end.as_bytes())
    }
}
