use crate::{
    array::{array_from_fn, array_try_from_fn},
    mat::Mat,
    stream::InputStream,
};
use from_str::FromStr;
use std::{
    fmt::{self, Debug, Display},
    io::BufRead,
};

pub(super) mod from_str;
pub(super) mod macros;

/// Error during using [ReadInto] or [ReadIntoSingle].
///
/// This error is usually caused by [std::io::Error] or [std::str::FromStr::Err].
pub enum ReadIntoError<E> {
    /// Error during reading from input.
    IOError(std::io::Error),
    /// Error during converting a string to a value, usually caused by calling [std::str::FromStr::from_str].
    FromStrError(E),
}

impl<E> Debug for ReadIntoError<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(error) => Debug::fmt(error, f),
            Self::FromStrError(error) => Debug::fmt(error, f),
        }
    }
}

impl<E> Display for ReadIntoError<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(error) => Display::fmt(error, f),
            Self::FromStrError(error) => Display::fmt(error, f),
        }
    }
}

impl<E> std::error::Error for ReadIntoError<E> where E: std::error::Error {}

macro_rules! unwrap {
    ($result:expr) => {
        $result.unwrap_or_else(|err| panic!("{err}"))
    };
}

/// Read a single data item from input stream.
/// All types that implement this trait also implement [ReadInto].
///
/// # Errors
///
/// - If the input cannot be parsed into `T`, [ReadIntoError::FromStrError] is returned.
/// - If the input is not valid UTF-8, [ReadIntoError::IOError] is returned.
/// - If an I/O error occurs, [ReadIntoError::IOError] is returned.
pub trait ReadIntoSingle<T> {
    /// Errors that come from [ReadInto].
    ///
    /// This is usually [ReadIntoError].
    type Error: std::error::Error;

    /// Read from `self` and parse into `T`.
    ///
    /// ```txt
    /// 1 2 3
    /// ```
    fn try_read_one(&mut self) -> Result<T, Self::Error>;
    /// Unwrapping version of [ReadInto::try_read].
    fn read_one(&mut self) -> T {
        unwrap!(self.try_read_one())
    }
    /// Read an element in the remained line from `self`, parse into `T`.
    ///
    /// ```txt
    /// 1 2 3
    /// ```
    fn try_read_remained_line(&mut self) -> Result<T, Self::Error>;
    /// Unwrapping version of [ReadIntoSingle::try_read_remained_line].
    fn read_remained_line(&mut self) -> T {
        unwrap!(self.try_read_remained_line())
    }
    /// Read an element in a single trimmed line that is not empty from `self`, parse into `T`.
    fn try_read_line(&mut self) -> Result<T, Self::Error>;
    /// Unwrapping version of [ReadIntoSingle::try_read_line].
    fn read_line(&mut self) -> T {
        unwrap!(self.try_read_line())
    }
    /// Read an element in a single non-whitespace character from `self`, parse into `T`.
    fn try_read_char(&mut self) -> Result<T, Self::Error>;
    /// Unwrapping version of [ReadIntoSingle::try_read_char].
    fn read_char(&mut self) -> T {
        unwrap!(self.try_read_char())
    }
    /// Read all remaining elements from `self`.
    fn try_read_all(&mut self) -> impl Iterator<Item = Result<T, Self::Error>>;
    /// Unwrapping version of [ReadIntoSingle::try_read_all].
    fn read_all(&mut self) -> impl Iterator<Item = T> {
        self.try_read_all().map(|res| unwrap!(res))
    }
    /// Read all elements in current line from `self`.
    fn try_read_all_in_line(&mut self) -> impl Iterator<Item = Result<T, Self::Error>>;
    /// Unwrapping version of [ReadIntoSingle::try_read_all_in_line].
    fn read_all_in_line(&mut self) -> impl Iterator<Item = T> {
        self.try_read_all_in_line().map(|res| unwrap!(res))
    }
    /// Read all elements in the remained line from `self`.
    fn try_read_all_in_remained_line(&mut self) -> impl Iterator<Item = Result<T, Self::Error>>;
    /// Unwrapping version of [ReadIntoSingle::try_read_all_in_remained_line].
    fn read_all_in_remained_line(&mut self) -> impl Iterator<Item = T> {
        self.try_read_all_in_remained_line().map(|res| unwrap!(res))
    }
}

impl<T: FromStr, B: BufRead> ReadIntoSingle<T> for InputStream<B> {
    type Error = ReadIntoError<T::Err>;
    fn try_read_one(&mut self) -> Result<T, Self::Error> {
        let res = self
            .consume_string(|s| T::from_str(s))
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
    fn try_read_line(&mut self) -> Result<T, Self::Error> {
        let res = self
            .consume_line(|s| T::from_str(s))
            .map_err(ReadIntoError::IOError)?
            .map_err(ReadIntoError::FromStrError)?;
        Ok(res)
    }
    fn try_read_char(&mut self) -> Result<T, Self::Error> {
        let c = self.consume_char().map_err(ReadIntoError::IOError)?;
        let res = T::from_str(&c.to_string()).map_err(ReadIntoError::FromStrError)?;
        Ok(res)
    }
    fn try_read_all(&mut self) -> impl Iterator<Item = Result<T, Self::Error>> {
        self.consume_all(T::from_str)
            .map(|res| res.map_err(ReadIntoError::FromStrError))
    }
    fn try_read_all_in_line(&mut self) -> impl Iterator<Item = Result<T, Self::Error>> {
        self.consume_strings_in_line()
            .map(T::from_str)
            .map(|res| res.map_err(ReadIntoError::FromStrError))
    }
    fn try_read_all_in_remained_line(&mut self) -> impl Iterator<Item = Result<T, Self::Error>> {
        self.consume_strings_in_remained_line()
            .map(T::from_str)
            .map(|res| res.map_err(ReadIntoError::FromStrError))
    }
}

/// Read data from input stream.
///
/// # Errors
///
/// - If the input cannot be parsed into `T`, [ReadIntoError::FromStrError] is returned.
/// - If the input is not valid UTF-8, [ReadIntoError::IOError] is returned.
/// - If an I/O error occurs, [ReadIntoError::IOError] is returned.
pub trait ReadInto<T> {
    /// Errors that come from [ReadInto].
    ///
    /// This is usually [ReadIntoError].
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
}

impl<T: FromStr, B: BufRead> ReadInto<T> for InputStream<B> {
    type Error = ReadIntoError<T::Err>;
    fn try_read(&mut self) -> Result<T, Self::Error> {
        self.try_read_one()
    }
}

impl<T, B: BufRead, const N: usize> ReadInto<[T; N]> for InputStream<B>
where
    Self: ReadInto<T>,
{
    type Error = <Self as ReadInto<T>>::Error;
    fn try_read(&mut self) -> Result<[T; N], Self::Error> {
        array_try_from_fn(|| self.try_read())
    }
    fn read(&mut self) -> [T; N] {
        array_from_fn(|| self.read())
    }
}

impl<T, B: BufRead, const N: usize> ReadInto<Box<[T; N]>> for InputStream<B>
where
    Self: ReadInto<T>,
{
    type Error = <Self as ReadInto<T>>::Error;
    fn try_read(&mut self) -> Result<Box<[T; N]>, Self::Error> {
        let res = self.try_read_n(N)?.into_boxed_slice().try_into();
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
impl<T, B: BufRead> ReadInto<Vec<T>> for InputStream<B>
where
    Self: ReadIntoSingle<T>,
{
    type Error = <Self as ReadIntoSingle<T>>::Error;
    fn try_read(&mut self) -> Result<Vec<T>, Self::Error> {
        // let len = self.try_read().map_err(Tuple2Error::T1)?;
        self.try_read_all_in_line().collect()
    }
    // Avoid constructing an enum value.
    fn read(&mut self) -> Vec<T> {
        // let len = self.read();
        self.read_all_in_line().collect()
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
        impl<$($t: FromStr, )* S: $(ReadInto<$t> +)* ?Sized> ReadInto<($($t, )*)> for S {
            type Error = $e<$(<S as ReadInto<$t>>::Error, )*>;
            fn try_read(&mut self) -> Result<($($t, )*), Self::Error> {
                Ok(( $(ReadInto::<$t>::try_read(self).map_err($e::$t)?, )* ))
            }
            fn read(&mut self) -> ($($t, )*) {
                // Avoid constructing an enum value.
                ( $(ReadInto::<$t>::read(self), )* )
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
