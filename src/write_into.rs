use std::{
    borrow::Borrow,
    io::{self, Write},
    ops::DerefMut,
    sync::Mutex,
};

use crate::stdio::STDOUT;

mod impls;
mod macros;

macro_rules! unwrap {
    ($result:expr) => {
        $result.unwrap_or_else(|err| panic!("{err}"))
    };
}

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
        let lock: &Mutex<io::Stdout> = Mutex::borrow(&STDOUT);
        let mut lock = lock.lock().unwrap();
        self.try_write_into(lock.deref_mut())
    }
    /// Unwrapping version of [WriteInto::try_write].
    fn write(&self) {
        unwrap!(self.try_write())
    }
}
