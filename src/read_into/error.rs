use std::fmt::{self, Debug, Display};

/// Error during using [ReadInto] or [ReadIntoOne].
///
/// This error is usually caused by [std::io::Error] or [std::str::FromStr::Err].
///
/// [ReadInto]: crate::ReadInto
/// [ReadIntoOne]: crate::ReadIntoOne
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

impl<E> From<std::io::Error> for ReadIntoError<E> {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
