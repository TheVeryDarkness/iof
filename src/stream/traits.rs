use super::{error::StreamError, EOL, WHITE};
use std::mem::transmute;

/// Extension trait for [BufRead].
///
/// It provides a way to read:
///
/// - A single non-ASCII-whitespace character ([BufReadExt::try_get_non_ws]),
/// - A single ASCII-white-space-separated string ([BufReadExt::try_get_string]),
/// - A single non-empty line ([BufReadExt::try_get_line_some]),
/// - Or just the remained line ([BufReadExt::try_get_line]).
///
/// ASCII whitespace characters here are `' '`, `'\t'`, `'\n'`, and `'\r'`.
///
/// [BufRead]: std::io::BufRead
pub trait BufReadExt {
    /// Get a single character.
    fn try_get(&mut self) -> Result<char, StreamError>;
    /// Peek a single character.
    fn try_peek(&mut self) -> Result<char, StreamError>;

    /// Get a single character if it is in `pattern`.
    fn try_get_if(&mut self, pattern: &[char]) -> Result<Option<char>, StreamError>;

    /// Get a single non-`skipped` character.
    #[inline]
    fn try_get_non(&mut self, skipped: &[char]) -> Result<char, StreamError> {
        loop {
            let c = self.try_get()?;
            if !skipped.contains(&c) {
                return Ok(c);
            }
        }
    }

    /// Get a single non-ASCII-whitespace character.
    #[inline]
    fn try_get_non_ws(&mut self) -> Result<char, StreamError> {
        self.try_get_non(&WHITE)
    }

    /// Skip a single character.
    #[inline]
    fn try_skip_any(&mut self) -> Result<(), StreamError> {
        self.try_get().map(|_| ())
    }

    /// Skip a single character if it is in `skipped`.
    ///
    /// Returns `true` if a character is skipped.
    #[inline]
    fn try_skip(&mut self, skipped: &[char]) -> Result<bool, StreamError> {
        self.try_get_if(skipped).map(|x| x.is_some())
    }

    /// Go to the next line if the remaining part are end of line characters.
    fn try_skip_eol(&mut self) -> Result<(), StreamError>;

    /// Skip all characters in `skipped`.
    fn try_skip_all(&mut self, skipped: &[char]) -> Result<usize, StreamError>;

    /// Skip a single ASCII-whitespace character.
    #[inline]
    fn try_skip_ws(&mut self) -> Result<bool, StreamError> {
        self.try_skip(&WHITE)
    }

    /// Skip all ASCII-whitespace characters.
    #[inline]
    fn try_skip_all_ws(&mut self) -> Result<usize, StreamError> {
        self.try_skip_all(&WHITE)
    }

    /// Read until a character in `pattern` is found or end of line.
    fn try_get_until_in_line(&mut self, pattern: &[char]) -> Result<&str, StreamError>;

    /// Get a single ASCII-whitespace-separated string.
    /// It won't contain the trailing ASCII-white-space characters.
    /// If current line is empty or all ASCII-white-spaces, it will read a new line.
    fn try_get_string_some(&mut self) -> Result<&str, StreamError>;

    /// Get a single line. The trailing newline will be consumed and trimmed.
    #[inline]
    fn try_get_line(&mut self) -> Result<&str, StreamError> {
        let line = self.try_get_until_in_line(&[])?;
        Ok(line.trim_end_matches(EOL))
    }

    /// Get a single line. The trailing white spaces will be consumed and trimmed.
    #[inline]
    fn try_get_line_trimmed(&mut self) -> Result<&str, StreamError> {
        let line = self.try_get_line()?;
        Ok(line.trim_end_matches(WHITE))
    }

    /// Get a single not-empty line. The trailing newline will be consumed and trimmed.
    ///
    /// Repeat reading a new line if current line is empty.
    #[inline]
    fn try_get_line_some(&mut self) -> Result<&str, StreamError> {
        loop {
            let line: &str = self.try_get_line()?;
            let line: &str = unsafe { transmute(line) };
            if !line.is_empty() {
                return Ok(line);
            }
        }
    }

    /// Get a single not-empty line.  The trailing white spaces will be consumed and trimmed.
    ///
    /// Repeat reading a new line if current line is empty.
    #[inline]
    fn try_get_line_some_trimmed(&mut self) -> Result<&str, StreamError> {
        loop {
            let line = self.try_get_line_some()?.trim_end_matches(WHITE);
            let line: &str = unsafe { transmute(line) };
            if !line.is_empty() {
                return Ok(line);
            }
        }
    }
}

impl<S: ?Sized + BufReadExt> BufReadExt for &mut S {
    fn try_get(&mut self) -> Result<char, StreamError> {
        (**self).try_get()
    }

    fn try_peek(&mut self) -> Result<char, StreamError> {
        (**self).try_peek()
    }

    fn try_get_if(&mut self, pattern: &[char]) -> Result<Option<char>, StreamError> {
        (**self).try_get_if(pattern)
    }

    fn try_skip_eol(&mut self) -> Result<(), StreamError> {
        (**self).try_skip_eol()
    }

    fn try_skip_all(&mut self, skipped: &[char]) -> Result<usize, StreamError> {
        (**self).try_skip_all(skipped)
    }

    fn try_get_until_in_line(&mut self, pattern: &[char]) -> Result<&str, StreamError> {
        (**self).try_get_until_in_line(pattern)
    }

    fn try_get_string_some(&mut self) -> Result<&str, StreamError> {
        (**self).try_get_string_some()
    }
}
