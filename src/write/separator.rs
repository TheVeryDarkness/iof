//! Separator and default separator.
use std::{
    fmt::{self, Formatter, Write as _},
    io::{self, Write},
};

/// Position.
///
/// The index of the last item that has been written.
pub type Position = usize;

/// Separator.
pub trait Separator {
    /// Write the separator to [fmt::Write].
    fn write_fmt(&self, f: &mut Formatter<'_>) -> fmt::Result;
    /// Write the separator to [io::Write].
    fn write_io(&self, s: &mut (impl Write + ?Sized)) -> io::Result<()>;
}

impl Separator for char {
    #[inline]
    fn write_fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(*self)
    }

    #[inline]
    fn write_io(&self, s: &mut (impl Write + ?Sized)) -> io::Result<()> {
        s.write_all(self.encode_utf8(&mut [0; 4]).as_bytes())
    }
}

impl Separator for str {
    #[inline]
    fn write_fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }

    #[inline]
    fn write_io(&self, s: &mut (impl Write + ?Sized)) -> io::Result<()> {
        s.write_all(self.as_bytes())
    }
}

impl Separator for String {
    #[inline]
    fn write_fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }

    #[inline]
    fn write_io(&self, s: &mut (impl Write + ?Sized)) -> io::Result<()> {
        s.write_all(self.as_bytes())
    }
}

impl<T: Separator + ?Sized> Separator for &T {
    #[inline]
    fn write_fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <T as Separator>::write_fmt(*self, f)
    }

    #[inline]
    fn write_io(&self, s: &mut (impl Write + ?Sized)) -> io::Result<()> {
        <T as Separator>::write_io(*self, s)
    }
}
