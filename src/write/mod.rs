use crate::{stdout, unwrap, Mat, SepBy};
use std::io::{self, Write};

mod impls;
mod macros;

type Result<T = ()> = io::Result<T>;

/// Write an element into a stream.
///
/// Most types that implement [Display] also implement this.
///
/// [Display]: std::fmt::Display
pub trait WriteOneInto {
    /// Separator between items.
    const SEP_ITEM: &'static str = " ";
    /// Separator between lines.
    const SEP_LINE: &'static str = "\n";
    /// Write into a stream.
    fn try_write_one_into<S: Write>(&self, s: &mut S) -> Result;
    /// Unwrapping version of [WriteOneInto::try_write_one_into].
    fn write_one_into<S: Write>(&self, s: &mut S) {
        unwrap!(self.try_write_one_into(s))
    }
}

impl<T: WriteOneInto + ?Sized> WriteOneInto for &T {
    fn try_write_one_into<S: Write>(&self, s: &mut S) -> Result {
        (*self).try_write_one_into(s)
    }
}

/// Write into a stream.
///
/// - Most types that implement [std::fmt::Display] also implement this.
/// - [Vec] and `[T]` where `T` implements [std::fmt::Display] also implements this.
///   They write each item separated by a space.
/// - [Mat] where `T` implements [std::fmt::Display] also implements this.
///   They write each row separated by a newline, and each item in a row separated by a space.
///
/// [Mat]: crate::Mat
pub trait WriteInto {
    /// Write into a stream.
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result;
    /// Unwrapping version of [WriteInto::try_write_into].
    fn write_into<S: Write>(&self, s: &mut S) {
        unwrap!(self.try_write_into(s))
    }
    /// Write into a string.
    fn try_write_into_string(&self) -> Result<String> {
        let mut s = Vec::new();
        self.try_write_into(&mut s)?;
        // What if the string is not valid UTF-8?
        String::from_utf8(s).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
    /// Unwrapping version of [WriteInto::try_write_into_string].
    fn write_into_string(&self) -> String {
        unwrap!(self.try_write_into_string())
    }
    /// Write into [std::io::Stdout].
    fn try_write(&self) -> Result {
        self.try_write_into(&mut stdout())
    }
    /// Unwrapping version of [WriteInto::try_write].
    fn write(&self) {
        unwrap!(self.try_write())
    }
}

impl<T: WriteOneInto> WriteInto for T {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result<()> {
        self.try_write_one_into(s)
    }
}

impl<T: WriteOneInto> WriteInto for Vec<T> {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result<()> {
        self.as_slice().try_write_into(s)
    }
}

impl<T: WriteOneInto, const N: usize> WriteInto for [T; N] {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result<()> {
        self.as_slice().try_write_into(s)
    }
}
impl<T: WriteOneInto> WriteInto for [T] {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result<()> {
        WriteInto::try_write_into(&self.sep_by(T::SEP_ITEM), s)
    }
}

impl<T: WriteOneInto> WriteInto for Mat<T> {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result<()> {
        self.iter()
            .map(|row| row.iter().sep_by(T::SEP_ITEM))
            .sep_by(T::SEP_LINE)
            .try_write_into(s)
    }
}

impl<T: WriteOneInto, const M: usize, const N: usize> WriteInto for [[T; N]; M] {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result<()> {
        self.iter()
            .map(|row| row.iter().sep_by(T::SEP_ITEM))
            .sep_by(T::SEP_LINE)
            .try_write_into(s)
    }
}
