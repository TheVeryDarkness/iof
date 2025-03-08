use super::{
    fmt::Format,
    iter::{ReadAll, ReadAllIn},
};
use crate::{
    ext::{Pattern, PatternError},
    stream::{error::StreamError, ext::Any, traits::BufReadExtWithFormat},
    BufReadExt, ReadError,
};
use std::any::type_name;

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

    /// Check if a character is acceptable.
    #[inline]
    fn accept() -> impl Pattern<Item = char> {
        Any::new()
    }

    /// Read from `stream` and parse into `Self`.
    #[inline]
    fn try_read_one_from<F: Format, S: BufReadExt>(
        stream: &mut S,
        format: F,
    ) -> Result<Self, ReadError<Self::ParseError>> {
        let s = stream
            .try_get_string_some(format.skip(), Self::accept())
            .map_err(|error| match error {
                PatternError::Extra(StreamError::IOError(e)) => ReadError::IOError(e),
                PatternError::Extra(StreamError::Eof) => ReadError::EOF,
                PatternError::Extra(StreamError::Eol) => ReadError::EOL,
                PatternError::UnexpectedChar(c) => {
                    ReadError::UnexpectedChar(c, type_name::<Self>())
                } // PatternError::Unfulfilled(s) => ReadError::Unfulfilled(s),
            })?;
        Self::parse(s)
    }

    /// Read an element in a single non-whitespace character from `stream`, parse into `Self`.
    #[inline]
    fn try_read_in_char_from<F: Format, S: BufReadExt>(
        stream: &mut S,
        format: F,
    ) -> Result<Self, ReadError<Self::ParseError>> {
        let s = stream.try_get_non_skipped(format.skip())?;
        Self::parse(s.encode_utf8(&mut [0; 4]))
    }

    /// Read an element in the remained line from `stream`, parse into `Self`.
    #[inline]
    fn try_read_in_line_trimmed_from<F: Format, S: BufReadExt>(
        stream: &mut S,
        format: F,
    ) -> Result<Self, ReadError<Self::ParseError>> {
        let s = stream.try_get_line_trimmed(format.skip())?.trim_start();
        Self::parse(s)
    }

    /// Read an element in a single trimmed line that is not empty from `stream`, parse into `Self`.
    #[inline]
    fn try_read_in_line_some_trimmed_from<F: Format, S: BufReadExt>(
        stream: &mut S,
        format: F,
    ) -> Result<Self, ReadError<Self::ParseError>> {
        let s = stream
            .try_get_line_some_trimmed(format.skip())?
            .trim_start();
        Self::parse(s)
    }

    /// Read all remaining elements from `stream` into a [Vec] of `Self`.
    #[inline]
    fn try_read_all_from<F: Format, S: BufReadExt>(
        stream: &mut S,
        format: F,
    ) -> Result<Vec<Self>, ReadError<Self::ParseError>> {
        ReadAll::<F, S, Self>::new(stream, format).collect()
    }

    /// Read all elements in current line from `stream` into a [Vec] of `Self`.
    #[inline]
    fn try_read_any_in_line_from<F: Format, S: BufReadExt>(
        stream: &mut S,
        format: F,
    ) -> Result<Vec<Self>, ReadError<Self::ParseError>> {
        ReadAllIn::<F, Self>::new(stream.try_get_line_trimmed(format.skip())?, format).collect()
    }

    /// Read all elements in a non-empty line from `stream` into a [Vec] of `Self`.
    #[inline]
    fn try_read_some_in_line_from<F: Format, S: BufReadExt>(
        stream: &mut S,
        format: F,
    ) -> Result<Vec<Self>, ReadError<Self::ParseError>> {
        ReadAllIn::<F, Self>::new(stream.try_get_line_some_trimmed(format.skip())?, format)
            .collect()
    }
}
