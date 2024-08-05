use crate::{
    mat::Mat,
    stdio::STDIN,
    stream::{InputStream, RealAll},
};
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

macro_rules! unwrap {
    ($result:expr) => {
        $result.unwrap_or_else(|err| panic!("{err}"))
    };
}

/// Read data from input stream.
pub trait ReadInto<T> {
    /// Errors that come from [ReadInto].
    type Error: std::error::Error;
    /// Read from `self` and parse into `T`.
    fn try_read(&mut self) -> Result<T, Self::Error>;
    /// Unwrapping version of [ReadInto::try_read].
    fn read(&mut self) -> T {
        unwrap!(self.try_read())
    }
    /// Read `n` elements from `self`, parse into `T` and aggregate them into a single [Vec].
    fn try_read_n(&mut self, n: usize) -> Result<Vec<T>, Self::Error> {
        let mut res = Vec::new();
        for _ in 0..n {
            res.push(self.try_read()?);
        }
        Ok(res)
    }
    /// Unwrapping version of [ReadInto::try_read_n].
    fn read_n(&mut self, n: usize) -> Vec<T> {
        unwrap!(self.try_read_n(n))
    }
    /// Read `m * n` elements from `self`, parse into `T` and aggregate them into a single [Mat].
    fn try_read_m_n(&mut self, m: usize, n: usize) -> Result<Mat<T>, Self::Error> {
        let mut res = Vec::new();
        for _ in 0..m {
            for _ in 0..n {
                res.push(self.try_read()?);
            }
        }
        Ok(Mat::from_vec(m, n, res))
    }
    /// Unwrapping version of [ReadInto::try_read_n].
    fn read_m_n(&mut self, m: usize, n: usize) -> Mat<T> {
        unwrap!(self.try_read_m_n(m, n))
    }
    /// Read all remaining elements from `self`.
    fn read_all(&mut self) -> RealAll<'_, Self, T> {
        RealAll::new(self)
    }
}

impl<T: FromStr, B: BufRead> ReadInto<T> for InputStream<B>
where
    T::Err: std::error::Error,
{
    type Error = ReadIntoError<T>;
    fn try_read(&mut self) -> Result<T, Self::Error> {
        let res = self
            .consume_string(|s| T::from_str(s))
            .map_err(ReadIntoError::IOError)?
            .map_err(ReadIntoError::FromStrError)?;
        Ok(res)
    }
}

/// Read from [std::io::Stdin] and parse into `T`.
pub fn try_read<T: FromStr>() -> Result<T, ReadIntoError<T>>
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().try_read())
}

/// Unwrapping version of [try_read].
pub fn read<T: FromStr>() -> T
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().read())
}

/// Read `n` elements from [std::io::Stdin] and parse into `Vec<T>`.
pub fn try_read_n<T: FromStr>(n: usize) -> Result<Vec<T>, ReadIntoError<T>>
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().try_read_n(n))
}

/// Unwrapping version of [try_read_n].
pub fn read_n<T: FromStr>(n: usize) -> Vec<T>
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().read_n(n))
}

/// Read `n` elements from [std::io::Stdin] and parse into `Vec<T>`.
pub fn try_read_m_n<T: FromStr>(m: usize, n: usize) -> Result<Mat<T>, ReadIntoError<T>>
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().try_read_m_n(m, n))
}

/// Unwrapping version of [try_read_n].
pub fn read_m_n<T: FromStr>(m: usize, n: usize) -> Mat<T>
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().read_m_n(m, n))
}
