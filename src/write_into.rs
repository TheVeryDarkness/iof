use crate::{mat::Mat, stdio::STDOUT, SepBy};
use std::{io, io::Write, ops::DerefMut};

pub(crate) mod display;

macro_rules! unwrap {
    ($result:expr) => {
        $result.unwrap_or_else(|err| panic!("{err}"))
    };
}

type Result<T = ()> = io::Result<T>;

/// Write into a stream.
///
/// - All types that implement [display::Display] also implement this.
/// - [Vec<T>] and `[T]` where `T` implements [display::Display] also implements this.
///   They write each item separated by a space.
/// - [Mat<T>] where `T` implements [display::Display] also implements this.
///   They write each row separated by a newline, and each item in a row separated by a space.
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
        Ok(String::from_utf8(s).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?)
    }
    /// Unwrapping version of [WriteInto::try_write_into_string].
    fn write_into_string(&self) -> String {
        unwrap!(self.try_write_into_string())
    }
    /// Write into [std::io::Stdout].
    fn try_write(&self) -> Result {
        STDOUT.with(|lock| self.try_write_into(lock.borrow_mut().deref_mut()))
    }
    /// Unwrapping version of [WriteInto::try_write].
    fn write(&self) {
        unwrap!(self.try_write())
    }
}

/// Implement [WriteInto] for given types that already implements [std::fmt::Display].
#[macro_export]
macro_rules! impl_write_into {
    ($($ty:ty)*) => {
        $(
            impl $crate::WriteInto for $ty {
                fn try_write_into<S: Write>(&self, s: &mut S) -> ::std::io::Result<()> {
                    ::std::write!(s, "{}", self)
                }
            }
        )*
    };
}

impl_write_into!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
    bool
    char str String
);

impl<T: WriteInto + ?Sized> WriteInto for &T {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result {
        WriteInto::try_write_into(*self, s)
    }
}

impl<T: WriteInto> WriteInto for Vec<T> {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result {
        self.as_slice().try_write_into(s)
    }
}

impl<T: WriteInto, const N: usize> WriteInto for [T; N] {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result {
        self.as_slice().try_write_into(s)
    }
}

impl<T> WriteInto for [T]
where
    T: WriteInto,
{
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result {
        WriteInto::try_write_into(&self.sep_by(" "), s)
    }
}

impl<T: WriteInto> WriteInto for Mat<T> {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result {
        fn row_sep_by<T: WriteInto>(
            row: &[T],
        ) -> crate::sep_by::SepBy<'_, std::slice::Iter<'_, T>> {
            row.iter().sep_by(" ")
        }
        self.iter().map(row_sep_by).sep_by("\n").try_write_into(s)
    }
}
