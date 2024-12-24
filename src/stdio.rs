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
pub(crate) static STDIN: LazyLock<Mutex<InputStream<BufReader<Stdin>>>> =
    LazyLock::new(|| Mutex::new(InputStream::new(BufReader::new(io::stdin()))));

/// Get an exclusive handle to the standard input stream.
///
/// See [io::stdin] and [io::Stdin::lock] for more information.
///
/// # Panics
///
/// This function will panic if the standard input stream is already locked by current thread,
/// or if the standard input stream is poisoned.
#[inline]
pub fn stdin() -> MutexGuard<'static, InputStream<BufReader<Stdin>>> {
    STDIN.lock().unwrap()
}

/// Get an exclusive handle to the standard output stream.
///
/// See [io::stdout] and [io::Stdout::lock] for more information.
#[inline]
pub fn stdout() -> io::StdoutLock<'static> {
    io::stdout().lock()
}
