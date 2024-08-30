use crate::{mat::Mat, stdio::STDOUT};
use display::Display;
use std::{fmt, fmt::Write, ops::DerefMut};

pub(crate) mod display;

macro_rules! unwrap {
    ($result:expr) => {
        $result.unwrap_or_else(|err| panic!("{err}"))
    };
}

/// Write into a stream.
///
/// - All types that implement [display::Display] also implement this.
/// - [Vec<T>] and `[T]` where `T` implements [display::Display] also implements this.
///   They write each item separated by a space.
/// - [Mat<T>] where `T` implements [display::Display] also implements this.
///   They write each row separated by a newline, and each item in a row separated by a space.
pub trait WriteInto {
    /// Write into a stream.
    fn try_write_into<S: Write>(&self, s: &mut S) -> fmt::Result;
    /// Unwrapping version of [WriteInto::try_write_into].
    fn write_into<S: Write>(&self, s: &mut S) {
        unwrap!(self.try_write_into(s))
    }
    /// Write into a string.
    fn try_write_into_string(&self) -> Result<String, fmt::Error> {
        let mut s = String::new();
        self.try_write_into(&mut s)?;
        // What if the string is not valid UTF-8?
        Ok(s)
    }
    /// Unwrapping version of [WriteInto::try_write_into_string].
    fn write_into_string(&self) -> String {
        unwrap!(self.try_write_into_string())
    }
    /// Write into [std::io::Stdout].
    fn try_write(&self) -> fmt::Result {
        STDOUT.with(|lock| self.try_write_into(lock.borrow_mut().deref_mut()))
    }
    /// Unwrapping version of [WriteInto::try_write].
    fn write(&self) {
        unwrap!(self.try_write())
    }
}

impl<T: Display + ?Sized> WriteInto for T {
    fn try_write_into<S: Write>(&self, s: &mut S) -> fmt::Result {
        Display::fmt(self, s)
    }
}

impl<T: WriteInto> WriteInto for Vec<T> {
    fn try_write_into<S: Write>(&self, s: &mut S) -> fmt::Result {
        self.as_slice().try_write_into(s)
    }
}

impl<T: WriteInto, const N: usize> WriteInto for [T; N] {
    fn try_write_into<S: Write>(&self, s: &mut S) -> fmt::Result {
        self.as_slice().try_write_into(s)
    }
}

impl<T: WriteInto> WriteInto for [T] {
    fn try_write_into<S: Write>(&self, s: &mut S) -> fmt::Result {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            first.try_write_into(s)?;
        }
        for item in iter {
            s.write_str(" ")?;
            item.try_write_into(s)?
        }
        Ok(())
    }
}

impl<T: Display> WriteInto for Mat<T> {
    fn try_write_into<S: Write>(&self, s: &mut S) -> fmt::Result {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            first.try_write_into(s)?;
        }
        for row in iter {
            s.write_str("\n")?;
            row.try_write_into(s)?
        }
        Ok(())
    }
}
