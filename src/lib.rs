#![forbid(missing_docs, rust_2021_compatibility, rust_2018_idioms)]
//! A utility library from input/output.
use std::{
    fmt::{self, Debug},
    io::BufRead,
    mem::take,
    str::{FromStr, Utf8Error},
    string::FromUtf8Error,
    usize,
};

#[derive(Debug)]
/// An error type for reading from buffer into specified type `T`.
pub enum ReadError {
    /// Failed to interpret a sequence of [u8] as a string.. See [std::str::from_utf8].
    Utf8Error(Utf8Error),
    /// Failed to convert a [String] from a UTF-8 byte vector. See [std::string::String::from_utf8].
    FromUtf8Error(FromUtf8Error),
    /// I/O operations failure.
    IOError(std::io::Error),
}
impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Utf8Error(err) => write!(f, "{err}"),
            Self::FromUtf8Error(err) => write!(f, "{err}"),
            Self::IOError(err) => write!(f, "{err}"),
        }
    }
}
impl std::error::Error for ReadError {}

#[derive(Debug)]
/// An error type for reading from buffer into specified type `T`.
pub enum Error<T: FromStr + Debug> {
    /// An error from [FromStr::from_str].
    ParseError(T::Err, String),
    /// An error that occurs during reading.
    ReadError(ReadError),
}
impl<T: FromStr + Debug> fmt::Display for Error<T>
where
    T::Err: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err, string) => write!(f, "{err} during parsing {string:?}"),
            Self::ReadError(err) => write!(f, "{err}"),
        }
    }
}
impl<T: FromStr + Debug> std::error::Error for Error<T> where T::Err: fmt::Display + fmt::Debug {}

#[cfg(not(feature = "auto_unwrap"))]
mod util {
    pub type Result<T, E> = std::result::Result<T, E>;
}
#[cfg(feature = "auto_unwrap")]
mod util {
    pub type Result<T, E> = T;
}

use util::Result;

fn read_until<'r, R: BufRead + ?Sized>(
    reader: &'r mut R,
    buf: &mut Vec<u8>,
    separator: u8,
) -> Result<String, ReadError> {
    reader
        .read_until(separator, buf)
        .map_err(|err| ReadError::IOError(err))?;
    if buf.ends_with(&[separator]) {
        buf.pop();
    }
    let res = String::from_utf8(take(buf)).map_err(|err| ReadError::FromUtf8Error(err))?;
    Ok(res)
}

/// An iterator for reading separated elements from Buffer.
pub struct Separated<'r, R: ?Sized> {
    reader: &'r mut R,
    buf: Vec<u8>,
    separator: u8,
}

impl<'r, R: BufRead + ?Sized> Iterator for Separated<'r, R> {
    type Item = Result<String, ReadError>;
    fn next(&mut self) -> Option<Self::Item> {
        match read_until(self.reader, &mut self.buf, self.separator) {
            Ok(s) => {
                if s.is_empty() {
                    None
                } else {
                    Some(Ok(s))
                }
            }
            Err(err) => Some(Err(err)),
        }
    }
}

type SperatedParsed<'b, B, T> =
    std::iter::Map<Separated<'b, B>, fn(Result<String, ReadError>) -> Result<T, Error<T>>>;

/// Reading in specified format.
pub trait Formatted: BufRead {
    /// Iterate over multiple elements that are separated by a given separator.
    fn parse_by_sep_iter<T: FromStr + Debug>(
        &mut self,
        separator: u8,
    ) -> SperatedParsed<'_, Self, T> {
        Separated {
            buf: Vec::new(),
            reader: self,
            separator,
        }
        .map(|s| {
            let s = s.map_err(|err| Error::ReadError(err))?;
            let res = T::from_str(s.as_str()).map_err(|err| Error::ParseError(err, s))?;
            Ok(res)
        })
    }
    /// Read multiple elements that are separated by a given separator into specified container.
    fn parse_by_sep<C: FromIterator<T>, T: FromStr + Debug>(
        &mut self,
        separator: u8,
    ) -> Result<C, Error<T>> {
        self.parse_by_sep_iter(separator).collect()
    }
    /// Read multiple elements that are separated by a space into specified container.
    fn parse_by_space<C: FromIterator<T>, T: FromStr + Debug>(&mut self) -> Result<C, Error<T>> {
        self.parse_by_sep(b' ')
    }
    /// Read multiple elements that are separated by a given separator into a [Vec].
    fn parse_vec_by_sep<T: FromStr + Debug>(
        &mut self,
        n: usize,
        separator: u8,
    ) -> Result<Vec<T>, Error<T>> {
        let mut res = Vec::with_capacity(n);
        let mut buf = Vec::new();
        for _ in 0..n {
            let elem = read_until(self, &mut buf, separator).map_err(Error::ReadError)?;
            let elem = T::from_str(&elem).map_err(|err| Error::ParseError(err, elem))?;
            res.push(elem);
            buf.clear();
        }
        Ok(res)
    }
    /// Read multiple elements that are separated by a space into a [Vec].
    fn parse_vec_by_space<T: FromStr + Debug>(&mut self, n: usize) -> Result<Vec<T>, Error<T>> {
        self.parse_vec_by_sep(n, b' ')
    }
}

impl<T: BufRead> Formatted for T {}

/// Read from stdin.
#[macro_export]
macro_rules! read_once {
    ($src:expr, $ty:ty) => {};
}

/// Read from stdin.
#[macro_export]
macro_rules! read {
    ($src:expr, $($ty:ty),* $(,)?) => {
        {
            ( $($crate::scan_once!($src, $ty), )* )
        }
    };
}
