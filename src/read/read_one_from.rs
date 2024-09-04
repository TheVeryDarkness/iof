use super::iter::{ReadAll, ReadAllIn};
use crate::{BufReadExt, ReadError};

/// The error type for [ReadOneFrom].
pub type ReadOneFromError<T> = ReadError<<T as ReadOneFrom>::ParseError>;

/// Read a single data item from input stream.
///
/// All types that implement this trait also implement [ReadFrom].
///
/// # Errors
///
/// - If the input cannot be parsed into `T`, [ReadError::FromStrError] is returned.
/// - If the input is not valid UTF-8, [ReadError::IOError] is returned.
/// - If an I/O error occurs, [ReadError::IOError] is returned.
///
/// [ReadFrom]: crate::ReadFrom
/// [ReadError]: crate::ReadError
/// [ReadError::FromStrError]: crate::ReadError::FromStrError
/// [ReadError::IOError]: crate::ReadError::IOError
pub trait ReadOneFrom: Sized {
    /// Errors that come from parsing.
    type ParseError: std::error::Error;

    /// Parse a string into `Self`.
    fn parse(s: &str) -> Result<Self, ReadError<Self::ParseError>>;

    /// Read from `stream` and parse into `Self`.
    fn try_read_one_from<S: BufReadExt>(
        stream: &mut S,
    ) -> Result<Self, ReadError<Self::ParseError>> {
        let s = stream.try_get_string_some()?;
        Self::parse(s)
    }

    /// Read an element in a single non-whitespace character from `stream`, parse into `Self`.
    fn try_read_in_char_from<S: BufReadExt>(
        stream: &mut S,
    ) -> Result<Self, ReadError<Self::ParseError>> {
        let s = stream.try_get_non_ws()?;
        Self::parse(s.encode_utf8(&mut [0; 4]))
    }

    /// Read an element in the remained line from `stream`, parse into `Self`.
    fn try_read_in_line_trimmed_from<S: BufReadExt>(
        stream: &mut S,
    ) -> Result<Self, ReadError<Self::ParseError>> {
        let s = stream.try_get_line_trimmed()?.trim_start();
        Self::parse(s)
    }

    /// Read an element in a single trimmed line that is not empty from `stream`, parse into `Self`.
    fn try_read_in_line_some_trimmed_from<S: BufReadExt>(
        stream: &mut S,
    ) -> Result<Self, ReadError<Self::ParseError>> {
        let s = stream.try_get_line_some_trimmed()?.trim_start();
        Self::parse(s)
    }

    /// Read all remaining elements from `stream` into a [Vec] of `Self`.
    fn try_read_all_from<S: BufReadExt>(
        stream: &mut S,
    ) -> Result<Vec<Self>, ReadError<Self::ParseError>> {
        ReadAll::<S, Self>::new(stream).collect()
    }

    /// Read all elements in current line from `stream` into a [Vec] of `Self`.
    fn try_read_any_in_line_from<S: BufReadExt>(
        stream: &mut S,
    ) -> Result<Vec<Self>, ReadError<Self::ParseError>> {
        ReadAllIn::<Self>::new(stream.try_get_line_trimmed()?).collect()
    }

    /// Read all elements in a non-empty line from `stream` into a [Vec] of `Self`.
    fn try_read_some_in_line_from<S: BufReadExt>(
        stream: &mut S,
    ) -> Result<Vec<Self>, ReadError<Self::ParseError>> {
        ReadAllIn::<Self>::new(stream.try_get_line_some_trimmed()?).collect()
    }
}