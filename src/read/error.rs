use crate::stream::{self, MSG_EOF, MSG_EOL};
use std::fmt::{self, Debug, Display};

/// Error during using [ReadInto] or [ReadOneFrom].
///
/// This error is usually caused by [std::io::Error] or [std::str::FromStr::Err].
///
/// [ReadInto]: crate::ReadInto
/// [ReadOneFrom]: crate::ReadOneFrom
#[derive(Debug)]
pub enum ReadError<E> {
    /// Error during reading from input.
    IOError(std::io::Error),
    /// Unexpected end of file.
    EOF,
    /// Unexpected end of line.
    EOL,
    /// Error during converting a string to a value, usually caused by calling [std::str::FromStr::from_str].
    FromStrError(E, String, &'static str),
}

impl<E> Display for ReadError<E>
where
    E: std::error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IOError(error) => Display::fmt(error, f),
            Self::EOF => f.write_str(MSG_EOF),
            Self::EOL => f.write_str(MSG_EOL),
            Self::FromStrError(error, s, name) => {
                write!(
                    f,
                    "Error during converting a string {:?} to a value of `{}`: ",
                    s, name,
                )?;
                Display::fmt(error, f)
            }
        }
    }
}

impl<E> std::error::Error for ReadError<E> where E: std::error::Error {}

impl<E> From<stream::error::StreamError> for ReadError<E> {
    fn from(error: stream::error::StreamError) -> Self {
        match error {
            stream::error::StreamError::IOError(e) => Self::IOError(e),
            stream::error::StreamError::Eof => Self::EOF,
            stream::error::StreamError::Eol => Self::EOL,
        }
    }
}
