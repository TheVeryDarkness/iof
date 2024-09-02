use crate::{
    array::{array_from_fn, array_try_from_fn},
    mat::Mat,
    stream::InputStream,
    BufReadExt,
};
use std::{
    fmt::{self, Debug, Display},
    io::BufRead,
};

pub(super) mod error;
mod impls;
mod macros;

/// Unwrap a result or panic with the error message.
#[macro_export(local_inner_macros)]
macro_rules! unwrap {
    ($result:expr) => {
        $result.unwrap_or_else(|err| ::std::panic!("{err}"))
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
///
/// [ReadIntoError]: crate::ReadIntoError
/// [ReadIntoError::FromStrError]: crate::ReadIntoError::FromStrError
/// [ReadIntoError::IOError]: crate::ReadIntoError::IOError
pub trait ReadIntoOne<T>: BufReadExt {
    /// Errors that come from [ReadInto].
    ///
    /// This is usually [ReadIntoError].
    ///
    /// [ReadIntoError]: crate::ReadIntoError
    type Error: std::error::Error + From<std::io::Error>;

    /// Parse a string into `T`.
    fn parse(s: &str) -> Result<T, Self::Error>;

    /// Read from `self` and parse into `T`.
    ///
    /// ```txt
    /// 1 2 3
    /// ```
    fn try_read_one(&mut self) -> Result<T, Self::Error> {
        let s = self.try_get_string()?;
        Self::parse(s)
    }
    /// Unwrapping version of [ReadInto::try_read].
    fn read_one(&mut self) -> T {
        unwrap!(self.try_read_one())
    }
    /// Read an element in the remained line from `self`, parse into `T`.
    ///
    /// ```txt
    /// 1 2 3
    /// ```
    ///
    /// The above example will read `1 2 3`.
    fn try_read_in_line_trimmed(&mut self) -> Result<T, Self::Error> {
        let s = self.try_get_line_trimmed()?.trim_start();
        Self::parse(s)
    }
    /// Unwrapping version of [ReadIntoOne::try_read_in_line_trimmed].
    fn read_in_line_trimmed(&mut self) -> T {
        unwrap!(self.try_read_in_line_trimmed())
    }
    /// Read an element in a single trimmed line that is not empty from `self`, parse into `T`.
    ///
    /// ```txt
    /// 1 2 3
    /// ```
    ///
    /// ```txt
    ///
    /// 1 2 3
    /// ```
    ///
    /// Both examples will read `1 2 3`.
    fn try_read_in_line_some_trimmed(&mut self) -> Result<T, Self::Error> {
        let s = self.try_get_line_some_trimmed()?.trim_start();
        Self::parse(s)
    }
    /// Unwrapping version of [ReadIntoOne::try_read_in_line_some_trimmed].
    fn read_in_line_some_trimmed(&mut self) -> T {
        unwrap!(self.try_read_in_line_some_trimmed())
    }
    /// Read an element in a single non-whitespace character from `self`, parse into `T`.
    fn try_read_in_char(&mut self) -> Result<T, Self::Error> {
        let s = self.try_get_non_ws()?;
        Self::parse(s.encode_utf8(&mut [0; 4]))
    }
    /// Unwrapping version of [ReadIntoOne::try_read_in_char].
    fn read_in_char(&mut self) -> T {
        unwrap!(self.try_read_in_char())
    }
    /// Read all remaining elements from `self`.
    fn try_read_all(&mut self) -> Result<Vec<T>, Self::Error> {
        self.try_get_all().map(Self::parse).collect()
    }
    /// Unwrapping version of [ReadIntoOne::try_read_all].
    fn read_all(&mut self) -> Vec<T> {
        unwrap!(self.try_read_all())
    }
    /// Read all elements in current line from `self`.
    fn try_read_all_in_line(&mut self) -> Result<Vec<T>, Self::Error> {
        self.try_get_all_in_line()?.map(Self::parse).collect()
    }
    /// Unwrapping version of [ReadIntoOne::try_read_all_in_line].
    fn read_all_in_line(&mut self) -> Vec<T> {
        unwrap!(self.try_read_all_in_line())
    }
    /// Read all elements in a non-empty line from `self`.
    fn try_read_all_in_line_some(&mut self) -> Result<Vec<T>, Self::Error> {
        self.try_get_all_in_line_some()?.map(Self::parse).collect()
    }
    /// Unwrapping version of [ReadIntoOne::try_read_all_in_line_some].
    fn read_all_in_line_some(&mut self) -> Vec<T> {
        unwrap!(self.try_read_all_in_line_some())
    }
}

/// Read data from input stream.
///
/// # Errors
///
/// - If the input cannot be parsed into `T`, [ReadIntoError::FromStrError] is returned.
/// - If the input is not valid UTF-8, [ReadIntoError::IOError] is returned.
/// - If an I/O error occurs, [ReadIntoError::IOError] is returned.
///
/// [ReadIntoError]: crate::ReadIntoError
/// [ReadIntoError::FromStrError]: crate::ReadIntoError::FromStrError
/// [ReadIntoError::IOError]: crate::ReadIntoError::IOError
pub trait ReadInto<T>: BufReadExt {
    /// Errors that come from [ReadInto].
    ///
    /// This is usually [ReadIntoError].
    ///
    /// [ReadIntoError]: crate::ReadIntoError
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
        let mut res = Mat::with_capacity(m);
        for _ in 0..m {
            res.push(self.try_read_n(n)?);
        }
        Ok(res)
    }
    /// Unwrapping version of [ReadInto::try_read_n].
    fn read_m_n(&mut self, m: usize, n: usize) -> Mat<T> {
        unwrap!(self.try_read_m_n(m, n))
    }
}

impl<T, B: BufRead> ReadInto<T> for InputStream<B>
where
    Self: ReadIntoOne<T>,
{
    type Error = <Self as ReadIntoOne<T>>::Error;
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
    Self: ReadIntoOne<T>,
{
    type Error = <Self as ReadIntoOne<T>>::Error;
    fn try_read(&mut self) -> Result<Vec<T>, Self::Error> {
        self.try_read_all_in_line()
    }
    // Avoid constructing an enum value.
    fn read(&mut self) -> Vec<T> {
        self.read_all_in_line()
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
        impl<$($t, )* S: $(ReadInto<$t> +)* ?Sized> ReadInto<($($t, )*)> for S where
            $(S: ReadInto<$t>, )*
        {
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
