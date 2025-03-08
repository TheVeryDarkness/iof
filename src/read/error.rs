use crate::stream::{error::StreamError, MSG_EOF, MSG_EOL};
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
    /// Error during matching a pattern.
    UnexpectedChar(String, &'static str),
    // /// Unfulfilled pattern.
    // Unfulfilled(String),
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
                    "error during converting a string {s:?} to a value of `{name}`: ",
                )?;
                Display::fmt(error, f)
            }
            Self::UnexpectedChar(s, t) => write!(f, "found unexpected character at the end of the string {s:?} during converting it to a value of {t:?}"),
            // Self::Unfulfilled(s) => write!(f, "unfulfilled pattern in {s:?}"),
        }
    }
}

impl<E> std::error::Error for ReadError<E> where E: std::error::Error {}

impl<E> From<StreamError> for ReadError<E> {
    #[inline]
    fn from(error: StreamError) -> Self {
        match error {
            StreamError::IOError(e) => Self::IOError(e),
            StreamError::Eof => Self::EOF,
            StreamError::Eol => Self::EOL,
        }
    }
}

// impl<E> From<PatternError<StreamError>> for ReadError<E> {
//     #[inline]
//     fn from(error: PatternError<StreamError>) -> Self {
//         match error {
//             PatternError::Extra(StreamError::IOError(e)) => Self::IOError(e),
//             PatternError::Extra(StreamError::Eof) => Self::EOF,
//             PatternError::Extra(StreamError::Eol) => Self::EOL,
//             PatternError::UnexpectedChar(c) => Self::UnexpectedChar(c),
//             // PatternError::Unfulfilled(s) => Self::Unfulfilled(s),
//         }
//     }
// }
