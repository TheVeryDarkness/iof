use super::STDIN;
use crate::BufReadExt;
use std::io::Error;

/// Read a line from standard input.
pub fn get_line() -> Result<String, Error> {
    STDIN.lock().unwrap().try_get_line().map(ToOwned::to_owned)
}

/// Read a non-empty line from standard input.
pub fn get_line_some() -> Result<String, Error> {
    STDIN
        .lock()
        .unwrap()
        .try_get_line_some()
        .map(ToOwned::to_owned)
}
