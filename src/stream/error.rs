use std::fmt::Display;

use super::{MSG_EOF, MSG_EOL};

/// Error type for the `Stream` module.
#[derive(Debug)]
pub enum StreamError {
    IOError(std::io::Error),
    EOF,
    EOL,
}

impl Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => Display::fmt(e, f),
            Self::EOF => f.write_str(MSG_EOF),
            Self::EOL => f.write_str(MSG_EOL),
        }
    }
}

impl std::error::Error for StreamError {}

impl From<std::io::Error> for StreamError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}
