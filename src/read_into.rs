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
        let mut res = Vec::with_capacity(n);
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
        let mut res = Vec::with_capacity(m * n);
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
    /// Read `N` elements from `self`, parse into `T` and aggregate them into a single [std::array].
    ///
    /// Use [std::array::try_from_fn] if it's stabilized.
    fn try_read_array<const N: usize>(&mut self) -> Result<Box<[T; N]>, Self::Error> {
        let res = self.try_read_n(N)?.into_boxed_slice().try_into();
        let res = unsafe { res.unwrap_unchecked() };
        Ok(res)
    }
    /// Unwrapping version of [ReadInto::try_read_array].
    fn read_array<const N: usize>(&mut self) -> Box<[T; N]> {
        unwrap!(self.try_read_array())
    }
    /// Read several elements from `self`, parse into `T` and aggregate them into a single tuple.
    fn try_read_tuple<U: MonoTuple<T, Self>>(&mut self) -> Result<U, Self::Error> {
        MonoTuple::read_from(self)
    }
    /// Unwrapping version of [ReadInto::try_read_tuple].
    fn read_tuple<U: MonoTuple<T, Self>>(&mut self) -> U {
        unwrap!(self.try_read_tuple())
    }
    /// Read an element in the remained line from `self`, parse into `T`.
    fn try_read_remained_line(&mut self) -> Result<T, Self::Error>;
    /// Unwrapping version of [ReadInto::try_read_remained_line].
    fn read_remained_line(&mut self) -> T {
        unwrap!(self.try_read_remained_line())
    }
    /// Read an element in a single trimmed line from `self`, parse into `T`.
    fn try_read_line(&mut self) -> Result<T, Self::Error>;
    /// Unwrapping version of [ReadInto::try_read_line].
    fn read_line(&mut self) -> T {
        unwrap!(self.try_read_line())
    }
    /// Read all remaining elements from `self`.
    fn read_all(&mut self) -> RealAll<'_, Self, T> {
        RealAll::new(self)
    }
}

/// For all tuple types, all of whose elements is the same.
pub trait MonoTuple<T, S: ReadInto<T> + ?Sized>: Sized {
    fn read_from(stream: &mut S) -> Result<Self, S::Error>;
}

macro_rules! impl_mono {
    ($($ty:ty)*) => {
        impl<T, S: ReadInto<T> + ?Sized> MonoTuple<T, S> for ( $($ty, )* ) {
            fn read_from(stream: &mut S) -> Result<Self, S::Error> {
                Ok(( $(ReadInto::<$ty>::try_read(stream)?, )* ))
            }
        }
    };
}

impl_mono!(T);
impl_mono!(T T);
impl_mono!(T T T);
impl_mono!(T T T T);
impl_mono!(T T T T T);
impl_mono!(T T T T T T);
impl_mono!(T T T T T T T);
impl_mono!(T T T T T T T T);
impl_mono!(T T T T T T T T T);
impl_mono!(T T T T T T T T T T);
impl_mono!(T T T T T T T T T T T);
impl_mono!(T T T T T T T T T T T T);

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
    fn try_read_line(&mut self) -> Result<T, Self::Error> {
        let res = self
            .consume_line(|s| T::from_str(s))
            .map_err(ReadIntoError::IOError)?
            .map_err(ReadIntoError::FromStrError)?;
        Ok(res)
    }
    fn try_read_remained_line(&mut self) -> Result<T, Self::Error> {
        let res = self
            .consume_remained_line(|s| T::from_str(s))
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

/// Read `n` elements from [std::io::Stdin] and parse into `Vec<T>`.
pub fn try_read_array<T: FromStr, const N: usize>() -> Result<Box<[T; N]>, ReadIntoError<T>>
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().try_read_array())
}

/// Unwrapping version of [try_read_array].
pub fn read_array<T: FromStr, const N: usize>() -> Box<[T; N]>
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().read_array())
}

/// Read `n` elements from [std::io::Stdin] and parse into `Vec<T>`.
pub fn try_read_tuple<T: FromStr, U: MonoTuple<T, InputStream<std::io::StdinLock<'static>>>>(
) -> Result<U, ReadIntoError<T>>
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().try_read_tuple())
}

/// Unwrapping version of [try_read_tuple].
pub fn read_tuple<T: FromStr, U: MonoTuple<T, InputStream<std::io::StdinLock<'static>>>>() -> U
where
    T::Err: std::error::Error,
{
    STDIN.with(|lock| lock.borrow_mut().read_tuple())
}
