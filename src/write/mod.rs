use crate::{stdout, SepBy};
use dimension::Dimension;
use separator::{GetDefaultSeparator, Separator};
use std::io::{self, Write};

pub mod dimension;
mod impls;
mod macros;
pub(super) mod sep_by;
pub mod separator;
pub(super) mod writer;

type Result<T = ()> = io::Result<T>;

/// Write into a stream.
///
/// - Most types that implement [std::fmt::Display] also implement this.
/// - [Vec] and `[T]` where `T` implements [std::fmt::Display] also implements this.
///   They write each item separated by a space.
/// - [Mat] where `T` implements [std::fmt::Display] also implements this.
///   They write each row separated by a newline, and each item in a row separated by a space.
///
/// [Mat]: crate::Mat
pub trait WriteInto: Dimension {
    /// Write into a stream with given separator.
    fn try_write_into_with_sep<S: Write + ?Sized>(
        &self,
        s: &mut S,
        sep: &[impl Separator],
    ) -> Result;
    /// Write into a stream.
    #[inline]
    fn try_write_into<S: Write + ?Sized>(&self, s: &mut S) -> Result
    where
        Self: GetDefaultSeparator,
    {
        self.try_write_into_with_sep(s, Self::DEFAULT_SEPARATOR)
    }
    /// Write into a string with given separator.
    #[inline]
    fn try_write_into_string_with_sep(&self, sep: &[impl Separator]) -> Result<String> {
        let mut s = Vec::new();
        self.try_write_into_with_sep(&mut s, sep)?;
        // What if the string is not valid UTF-8?
        String::from_utf8(s).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
    /// Write into a string.
    #[inline]
    fn try_write_into_string(&self) -> Result<String>
    where
        Self: GetDefaultSeparator,
    {
        self.try_write_into_string_with_sep(Self::DEFAULT_SEPARATOR)
    }
    /// Write into [std::io::Stdout] with given separator.
    #[inline]
    fn try_write_with_sep(&self, sep: &[impl Separator]) -> Result {
        self.try_write_into_with_sep(&mut stdout(), sep)
    }
    /// Write into [std::io::Stdout].
    #[inline]
    fn try_write(&self) -> Result
    where
        Self: GetDefaultSeparator,
    {
        self.try_write_with_sep(Self::DEFAULT_SEPARATOR)
    }
}

impl<T: WriteInto + ?Sized> WriteInto for &T {
    #[inline]
    fn try_write_into_with_sep<S: Write + ?Sized>(
        &self,
        s: &mut S,
        sep: &[impl Separator],
    ) -> Result<()> {
        (*self).try_write_into_with_sep(s, sep)
    }
}

impl<T: WriteInto> WriteInto for Vec<T> {
    #[inline]
    fn try_write_into_with_sep<S: Write + ?Sized>(
        &self,
        s: &mut S,
        sep: &[impl Separator],
    ) -> Result<()> {
        debug_assert_eq!(sep.len(), Self::DIMENSION);
        self.as_slice().try_write_into_with_sep(s, sep)
    }
}

impl<T: WriteInto, const N: usize> WriteInto for [T; N] {
    #[inline]
    fn try_write_into_with_sep<S: Write + ?Sized>(
        &self,
        s: &mut S,
        sep: &[impl Separator],
    ) -> Result<()> {
        self.as_slice().try_write_into_with_sep(s, sep)
    }
}
impl<T: WriteInto> WriteInto for [T] {
    #[inline]
    fn try_write_into_with_sep<S: Write + ?Sized>(
        &self,
        s: &mut S,
        sep: &[impl Separator],
    ) -> Result<()> {
        let (sep, residual) = sep.split_first().expect("Separator count mismatch.");
        WriteInto::try_write_into_with_sep(&self.sep_by_write_into(sep), s, residual)
    }
}
