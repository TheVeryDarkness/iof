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

// static LOCKED: RwLock<bool> = RwLock::new(false);

// struct LockGuard<T>(T);

// impl<T> LockGuard<T> {
//     fn new(inner: T) -> Self {
//         {
//             let mut guard = LOCKED.write().unwrap();
//             assert_eq!(*guard, false);
//             *guard = true;
//         }
//         LockGuard(inner)
//     }
// }

// impl<T> Drop for LockGuard<T> {
//     fn drop(&mut self) {
//         let mut guard = LOCKED.write().unwrap();
//         assert_eq!(*guard, true);
//         *guard = false;
//     }
// }

// impl<T> Deref for LockGuard<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> DerefMut for LockGuard<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

thread_local! {
    pub(crate) static STDIN: RefCell<InputStream<StdinLock<'static>>> = {
        RefCell::new(InputStream::new(std::io::stdin().lock()))
    };
    pub(crate) static STDOUT: RefCell<StdoutLock<'static>> = RefCell::new(std::io::stdout().lock());
}
