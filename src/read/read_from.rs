use super::read_one_from::ReadOneFrom;
use crate::{array::array_try_from_fn, mat::Mat, BufReadExt, ReadError};
use std::fmt::{self, Display};

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

macro_rules! impl_read_into_for_tuple {
    ($e:ident $($t:ident)*) => {
        #[derive(Debug)]
        pub enum $e<$($t, )* > {
            $($t($t), )*
        }
        impl<$($t: std::error::Error, )* > Display for $e<$($t, )* > {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self { $( Self::$t(err) => Display::fmt(err, f), )* }
            }
        }
        impl<$($t: std::error::Error, )* > std::error::Error for $e<$($t, )* > {}
        impl<$($t: ReadFrom, )*> ReadFrom for ( $($t, )* ) {
            type ParseError = $e<$(<$t as ReadFrom>::ParseError, )*>;
            fn try_read_from<S: BufReadExt>(stream: &mut S) -> Result<($($t, )*), ReadFromError<Self>> {
                Ok((
                    $(
                        <$t as ReadFrom>::try_read_from(stream).map_err(|err| match err {
                            ReadFromError::<$t>::IOError(e) => ReadFromError::<Self>::IOError(e),
                            ReadFromError::<$t>::EOF => ReadFromError::<Self>::EOF,
                            ReadFromError::<$t>::EOL => ReadFromError::<Self>::EOL,
                            ReadFromError::<$t>::FromStrError(e, s, n) => ReadFromError::<Self>::FromStrError($e::$t(e), s, n),
                        })?,
                    )*
                ))
            }
        }
    };
}

impl_read_into_for_tuple!(Tuple1Error T1);
impl_read_into_for_tuple!(Tuple2Error T1 T2);
impl_read_into_for_tuple!(Tuple3Error T1 T2 T3);
impl_read_into_for_tuple!(Tuple4Error T1 T2 T3 T4);
impl_read_into_for_tuple!(Tuple5Error T1 T2 T3 T4 T5);
impl_read_into_for_tuple!(Tuple6Error T1 T2 T3 T4 T5 T6);
impl_read_into_for_tuple!(Tuple7Error T1 T2 T3 T4 T5 T6 T7);
impl_read_into_for_tuple!(Tuple8Error T1 T2 T3 T4 T5 T6 T7 T8);
impl_read_into_for_tuple!(Tuple9Error T1 T2 T3 T4 T5 T6 T7 T8 T9);
impl_read_into_for_tuple!(Tuple10Error T1 T2 T3 T4 T5 T6 T7 T8 T9 T10);
impl_read_into_for_tuple!(Tuple11Error T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11);
impl_read_into_for_tuple!(Tuple12Error T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12);
