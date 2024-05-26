use std::{
    fmt::{self, Debug},
    io::BufRead,
    str::{FromStr, Utf8Error},
    usize,
};

#[derive(Debug)]
pub enum Error<T: FromStr + Debug> {
    ParseError(T::Err, String),
    InvalidEncoding(Utf8Error),
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

pub trait Formatted: BufRead {
    fn read_multiple_separated_by_space<T: FromStr + Debug>(
        &mut self,
        n: usize,
    ) -> Result<Vec<T>, Error<T>> {
        let mut res = Vec::with_capacity(n);
        let mut buf = Vec::new();
        for i in 0..n {
            self.read_until(b' ', &mut buf)
                .map_err(|err| Error::TooFewElements(i, err))?;
            let integer_part = buf.strip_suffix(b" ").unwrap_or(&buf);
            let fragment =
                std::str::from_utf8(&integer_part).map_err(|err| Error::InvalidEncoding(err))?;
            res.push(
                T::from_str(fragment).map_err(|err| Error::ParseError(err, fragment.to_owned()))?,
            );
            buf.clear();
        }
        Ok(res)
    }
}

impl<T: BufRead> Formatted for T {}
