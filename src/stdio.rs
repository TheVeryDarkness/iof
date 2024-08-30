//! Standard input/output streams.
//!
//! This module provides utilities for reading from standard input and writing to standard output.
//!
//! Calling functions in this module will lock the standard input/output streams, so it is not
//! recommended to use this module in a multi-threaded environment.
use crate::stream::InputStream;
use std::{
    cell::RefCell,
    io::{StdinLock, StdoutLock},
};

pub(crate) mod read_into;

thread_local! {
    pub(crate) static STDIN: RefCell<InputStream<StdinLock<'static>>> = RefCell::new(InputStream::new(std::io::stdin().lock()));
    pub(crate) static STDOUT: RefCell<StdoutLock<'static>> = RefCell::new(std::io::stdout().lock());
}
