use super::{fmt::Default, read_from::ReadFromError};
use crate::{unwrap, BufReadExt, Mat, ReadFrom};

/// The opposite of [ReadFrom].
pub trait ReadInto<T>: BufReadExt {
    /// Errors that come from [ReadOneFrom].
    ///
    /// This is usually [ReadError].
    ///
    /// [ReadOneFrom]: crate::ReadOneFrom
    /// [ReadError]: crate::ReadError
    type Error: std::error::Error;

    /// Read from `self` and parse into `Self`.
    fn try_read(&mut self) -> Result<T, Self::Error>;

    /// Read `n` elements from `self`, parse into `Self` and aggregate them into a single [Vec].
    fn try_read_n(&mut self, n: usize) -> Result<Vec<T>, Self::Error>;

    /// Read `m * n` elements from `self`, parse into `Self` and aggregate them into a single [Mat].
    fn try_read_m_n(&mut self, m: usize, n: usize) -> Result<Mat<T>, Self::Error>;

    /// Unwrap the result of [ReadInto::try_read].
    #[inline]
    #[track_caller]
    fn read(&mut self) -> T {
        unwrap!(self.try_read())
    }

    /// Unwrap the result of [ReadInto::try_read_n].
    #[inline]
    #[track_caller]
    fn read_n(&mut self, n: usize) -> Vec<T> {
        unwrap!(self.try_read_n(n))
    }

    /// Unwrap the result of [ReadInto::try_read_m_n].
    #[inline]
    #[track_caller]
    fn read_m_n(&mut self, m: usize, n: usize) -> Mat<T> {
        unwrap!(self.try_read_m_n(m, n))
    }
}

impl<T: BufReadExt, U> ReadInto<U> for T
where
    U: ReadFrom,
{
    type Error = ReadFromError<U>;

    #[inline]
    fn try_read(&mut self) -> Result<U, Self::Error> {
        U::try_read_from(self, Default::new())
    }

    #[inline]
    fn try_read_n(&mut self, n: usize) -> Result<Vec<U>, Self::Error> {
        U::try_read_n_from(self, n, Default::new())
    }

    #[inline]
    fn try_read_m_n(&mut self, m: usize, n: usize) -> Result<Mat<U>, Self::Error> {
        U::try_read_m_n_from(self, m, n, Default::new())
    }
}
