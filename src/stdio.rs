//! Standard input/output streams.
//!
//! This module provides utilities for reading from standard input and writing to standard output.
//!
//! Calling functions in this module will lock the standard input/output streams, so it is not
//! recommended to use this module in a multi-threaded environment.
use crate::InputStream;
use std::{
    io::{stdin, BufReader, Stdin},
    sync::{LazyLock, Mutex},
};

pub(crate) mod read_into;

/// Standard input stream.
pub static STDIN: LazyLock<Mutex<InputStream<BufReader<Stdin>>>> =
    LazyLock::new(|| Mutex::new(InputStream::new(BufReader::new(stdin()))));
