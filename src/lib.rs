#![deny(missing_docs, rust_2021_compatibility, rust_2018_idioms)]
//! A utility library from input/output.
use std::{
    fmt::{self, Debug},
    io::BufRead,
    marker::PhantomData,
    str::{FromStr, Utf8Error},
    usize,
};

#[derive(Debug)]
/// An error type for reading from buffer into specified type `T`.
pub enum Error<T: FromStr + Debug> {
    /// An error from [FromStr::from_str].
    ParseError(T::Err, String),
    /// An error from [std::str::from_utf8].
    InvalidEncoding(Utf8Error),
    /// Found too few elements when reading multiple elements from the buffer.
    TooFewElements(usize, std::io::Error),
}
impl<T: FromStr + Debug> fmt::Display for Error<T>
where
    T::Err: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err, string) => write!(f, "{err} during parsing {string:?}"),
            Self::InvalidEncoding(err) => write!(f, "{err}"),
            Self::TooFewElements(i, err) => {
                write!(f, "{err} (at {i})")
            }
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

fn read_until<'r, R: BufRead + ?Sized, T: FromStr + Debug>(
    reader: &'r mut R,
    buf: &mut Vec<u8>,
    separator: u8,
    i: usize,
) -> Result<T, Error<T>> {
    reader
        .read_until(separator, buf)
        .map_err(|err| Error::TooFewElements(i, err))?;
    let part = buf.strip_suffix(&[separator]).unwrap_or(&buf);
    let frag = std::str::from_utf8(&part).map_err(|err| Error::InvalidEncoding(err))?;
    let res = T::from_str(frag).map_err(|err| Error::ParseError(err, frag.to_owned()))?;
    Ok(res)
}

/// An iterator for reading separated elements from Buffer.
pub struct Splitted<'r, R: ?Sized, T> {
    reader: &'r mut R,
    buf: Vec<u8>,
    separator: u8,
    i: usize,
    phantom: PhantomData<T>,
}

impl<'r, R: BufRead + ?Sized, T: FromStr + Debug> Iterator for Splitted<'r, R, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let res = read_until(self.reader, &mut self.buf, self.separator, self.i).ok();
        if res.is_some() {
            self.i += 1;
        }
        res
    }
}

/// Reading in specified format.
pub trait Formatted: BufRead {
    /// Iterate over multiple elements that are separated by a given separator.
    fn parse_by_sep_iter<T: FromStr + Debug>(&mut self, separator: u8) -> Splitted<'_, Self, T> {
        Splitted {
            buf: Vec::new(),
            i: 0,
            reader: self,
            separator,
            phantom: PhantomData,
        }
    }
    /// Read multiple elements that are separated by a given separator into specified container.
    fn parse_by_sep<C: FromIterator<T>, T: FromStr + Debug>(&mut self, separator: u8) -> C {
        self.parse_by_sep_iter(separator).collect()
    }
    /// Read multiple elements that are separated by a space into specified container.
    fn parse_by_space<C: FromIterator<T>, T: FromStr + Debug>(&mut self) -> C {
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
        for i in 0..n {
            let elem = read_until(self, &mut buf, separator, i)?;
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
