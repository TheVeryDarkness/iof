use crate::stdio::STDIN;
use std::{
    fmt::{Debug, Display},
    io::BufRead,
    str::FromStr,
};

/// Error during using [ReadInto].
pub enum ReadIntoError<T: FromStr> {
    /// Error during reading from input.
    IOError(std::io::Error),
    /// Error during calling [FromStr::from_str].
    FromStrError(T::Err),
}

impl<T: FromStr> Debug for ReadIntoError<T>
where
    T::Err: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(error) => Debug::fmt(error, f),
            Self::FromStrError(error) => Debug::fmt(error, f),
        }
    }
}

impl<T: FromStr> Display for ReadIntoError<T>
where
    T::Err: std::error::Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(error) => Display::fmt(error, f),
            Self::FromStrError(error) => Display::fmt(error, f),
        }
    }
}

impl<T: FromStr> std::error::Error for ReadIntoError<T> where T::Err: std::error::Error {}

/// Read data from input stream.
pub trait ReadInto<T> {
    /// Errors that come from [ReadInto].
    type Error: std::error::Error;
    /// Read from `self` and parse into given type.
    fn try_read(&mut self) -> Result<T, Self::Error> {
        self.try_read_with(&mut String::new())
    }
    /// Read from `self` with given buffer and parse into given type.
    fn try_read_with(&mut self, buf: &mut String) -> Result<T, Self::Error>;
    /// Read from `self` and parse into given type.
    fn read(&mut self) -> T {
        self.try_read().unwrap_or_else(|err| panic!("{err}"))
    }
}

impl<T: FromStr, B: BufRead> ReadInto<T> for B
where
    T::Err: std::error::Error,
{
    type Error = ReadIntoError<T>;
    fn try_read_with(&mut self, buf: &mut String) -> Result<T, Self::Error> {
        buf.clear();
        self.read_line(buf).map_err(ReadIntoError::IOError)?;
        let res = T::from_str(buf).map_err(ReadIntoError::FromStrError)?;
        Ok(res)
    }
}

/// Read from [std::io::Stdin] and parse into `T`.
pub fn read<T: FromStr>() -> T
where
    T::Err: std::error::Error,
{
    STDIN.with_borrow_mut(|lock| lock.read())
}
