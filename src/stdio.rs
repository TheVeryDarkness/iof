//! Standard input/output streams.
//!
//! This module provides utilities for reading from standard input and writing to standard output.
//!
//! Calling functions in this module will lock the standard input/output streams, so it is not
//! recommended to use this module in a multi-threaded environment.
use crate::InputStream;
use std::{
    io::{self, BufReader, Stdin},
    sync::{LazyLock, Mutex, MutexGuard},
};

pub(crate) mod read_into;
pub(crate) mod stream;

/// Standard input stream.
pub static STDIN: LazyLock<Mutex<InputStream<BufReader<Stdin>>>> =
    LazyLock::new(|| Mutex::new(InputStream::new(BufReader::new(io::stdin()))));

/// Get a handle to the standard input stream.
pub fn stdin() -> MutexGuard<'static, InputStream<BufReader<Stdin>>> {
    STDIN.lock().unwrap()
}

/// Get a handle to the standard output stream.
pub fn stdout() -> io::StdoutLock<'static> {
    io::stdout().lock()
}
