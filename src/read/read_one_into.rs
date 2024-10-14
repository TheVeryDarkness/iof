use crate::{stream, unwrap, BufReadExt, ReadOneFrom, ReadOneFromError};

use super::fmt::Default;

/// The opposite of [ReadOneFrom].
pub trait ReadOneInto<T> {
    /// Errors that come from [ReadOneFrom].
    ///
    /// This is usually [ReadError].
    ///
    /// [ReadError]: crate::ReadError
    type Error: std::error::Error + From<stream::error::StreamError>;

    /// Read one from `self` and parse into `T`.
    fn try_read_one(&mut self) -> Result<T, Self::Error>;

    /// Read an element in a single non-whitespace character from `self`, parse into `T`.
    fn try_read_in_char(&mut self) -> Result<T, Self::Error>;

    /// Read an element in the remained line from `self`, parse into `T`.
    fn try_read_in_line_trimmed(&mut self) -> Result<T, Self::Error>;

    /// Read an element in a single trimmed line that is not empty from `self`, parse into `T`.
    fn try_read_in_line_some_trimmed(&mut self) -> Result<T, Self::Error>;

    /// Read all remaining elements from `stream`.
    fn try_read_all(&mut self) -> Result<Vec<T>, Self::Error>;

    /// Read all elements in current line from `self`.
    fn try_read_any_in_line(&mut self) -> Result<Vec<T>, Self::Error>;

    /// Read all elements in a non-empty line from `self`.
    fn try_read_some_in_line(&mut self) -> Result<Vec<T>, Self::Error>;

    /// Unwrap the result of [ReadOneInto::try_read_one].
    #[inline]
    #[track_caller]
    fn read_one(&mut self) -> T {
        unwrap!(self.try_read_one())
    }

    /// Unwrap the result of [ReadOneInto::try_read_in_char].
    #[inline]
    #[track_caller]
    fn read_in_char(&mut self) -> T {
        unwrap!(self.try_read_in_char())
    }

    /// Unwrap the result of [ReadOneInto::try_read_in_line_trimmed].
    #[inline]
    #[track_caller]
    fn read_in_line_trimmed(&mut self) -> T {
        unwrap!(self.try_read_in_line_trimmed())
    }

    /// Unwrap the result of [ReadOneInto::try_read_in_line_some_trimmed].
    #[inline]
    #[track_caller]
    fn read_in_line_some_trimmed(&mut self) -> T {
        unwrap!(self.try_read_in_line_some_trimmed())
    }

    /// Unwrap the result of [ReadOneInto::try_read_all].
    #[inline]
    #[track_caller]
    fn read_all(&mut self) -> Vec<T> {
        unwrap!(self.try_read_all())
    }

    /// Unwrap the result of [ReadOneInto::try_read_any_in_line].
    #[inline]
    #[track_caller]
    fn read_any_in_line(&mut self) -> Vec<T> {
        unwrap!(self.try_read_any_in_line())
    }

    /// Unwrap the result of [ReadOneInto::try_read_some_in_line].
    #[inline]
    #[track_caller]
    fn read_some_in_line(&mut self) -> Vec<T> {
        unwrap!(self.try_read_some_in_line())
    }
}

impl<T: ReadOneFrom, U: BufReadExt> ReadOneInto<T> for U {
    type Error = ReadOneFromError<T>;

    #[inline]
    fn try_read_one(&mut self) -> Result<T, Self::Error> {
        T::try_read_one_from(self, &Default)
    }

    #[inline]
    fn try_read_in_char(&mut self) -> Result<T, Self::Error> {
        T::try_read_in_char_from(self, &Default)
    }

    #[inline]
    fn try_read_in_line_trimmed(&mut self) -> Result<T, Self::Error> {
        T::try_read_in_line_trimmed_from(self, &Default)
    }

    #[inline]
    fn try_read_in_line_some_trimmed(&mut self) -> Result<T, Self::Error> {
        T::try_read_in_line_some_trimmed_from(self, &Default)
    }

    #[inline]
    fn try_read_all(&mut self) -> Result<Vec<T>, Self::Error> {
        T::try_read_all_from(self, &Default)
    }

    #[inline]
    fn try_read_any_in_line(&mut self) -> Result<Vec<T>, Self::Error> {
        T::try_read_any_in_line_from(self, &Default)
    }

    #[inline]
    fn try_read_some_in_line(&mut self) -> Result<Vec<T>, Self::Error> {
        T::try_read_some_in_line_from(self, &Default)
    }
}
