use super::{MSG_EOF, MSG_EOL};
use std::fmt::Display;

/// Error type for the `Stream` module.
#[derive(Debug)]
pub enum StreamError {
    IOError(std::io::Error),
    Eof,
    Eol,
}

impl Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => Display::fmt(e, f),
            Self::Eof => f.write_str(MSG_EOF),
            Self::Eol => f.write_str(MSG_EOL),
        }
    }
}

impl std::error::Error for StreamError {}

impl From<std::io::Error> for StreamError {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
