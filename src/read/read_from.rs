use super::read_one_from::ReadOneFrom;
use crate::{array::array_try_from_fn, mat::Mat, BufReadExt, ReadError};

/// The error type for [ReadFrom].
pub type ReadFromError<T> = ReadError<<T as ReadFrom>::ParseError>;

/// Read data from input stream.
///
/// # Errors
///
/// - If the input cannot be parsed into `T`, [ReadError::FromStrError] is returned.
/// - If the input is not valid UTF-8, [ReadError::IOError] is returned.
/// - If an I/O error occurs, [ReadError::IOError] is returned.
///
/// [ReadError]: crate::ReadError
/// [ReadError::FromStrError]: crate::ReadError::FromStrError
/// [ReadError::IOError]: crate::ReadError::IOError
pub trait ReadFrom: Sized {
    /// Errors that come from [ReadOneFrom].
    type ParseError: std::error::Error;

    /// Read from `self` and parse into `Self`.
    fn try_read_from<S: BufReadExt>(stream: &mut S) -> Result<Self, ReadFromError<Self>>;
    /// Read `n` elements from `self`, parse into `Self` and aggregate them into a single [Vec].
    fn try_read_n_from<S: BufReadExt>(
        stream: &mut S,
        n: usize,
    ) -> Result<Vec<Self>, ReadFromError<Self>> {
        let mut res = Vec::with_capacity(n);
        for _ in 0..n {
            res.push(Self::try_read_from(stream)?);
        }
        Ok(res)
    }
    /// Read `m * n` elements from `self`, parse into `Self` and aggregate them into a single [Mat].
    fn try_read_m_n_from<S: BufReadExt>(
        stream: &mut S,
        m: usize,
        n: usize,
    ) -> Result<Mat<Self>, ReadFromError<Self>> {
        let mut res = Mat::with_capacity(m);
        for _ in 0..m {
            res.push(Self::try_read_n_from(stream, n)?);
        }
        Ok(res)
    }
}

impl<T: ReadOneFrom> ReadFrom for T {
    type ParseError = <Self as ReadOneFrom>::ParseError;

    fn try_read_from<S: BufReadExt>(stream: &mut S) -> Result<T, ReadFromError<Self>> {
        Self::try_read_one_from(stream)
    }
}

impl<T: ReadFrom, const N: usize> ReadFrom for [T; N] {
    type ParseError = <T as ReadFrom>::ParseError;

    fn try_read_from<S: BufReadExt>(stream: &mut S) -> Result<[T; N], ReadFromError<Self>> {
        array_try_from_fn(|| T::try_read_from(stream))
    }
}

impl<T: ReadFrom, const N: usize> ReadFrom for Box<[T; N]> {
    type ParseError = <T as ReadFrom>::ParseError;

    fn try_read_from<S: BufReadExt>(stream: &mut S) -> Result<Box<[T; N]>, ReadFromError<Self>> {
        let res = T::try_read_n_from(stream, N)?.into_boxed_slice().try_into();
        let res = unsafe { res.unwrap_unchecked() };
        Ok(res)
    }
}

/// Read several data items in a line from input stream.
///
/// Such as:
///
/// ```txt
/// 1 2 3
/// ```
impl<T: ReadOneFrom> ReadFrom for Vec<T> {
    type ParseError = <T as ReadOneFrom>::ParseError;
    fn try_read_from<S: BufReadExt>(stream: &mut S) -> Result<Vec<T>, ReadFromError<Self>> {
        T::try_read_some_in_line_from(stream)
    }
}
