#![forbid(missing_docs, rust_2021_compatibility, rust_2018_idioms)]
//! A utility library for reading integers, floating numbers and strings from input/output.

pub use {
    read_into::{read, ReadInto, ReadIntoError},
    stream::InputStream,
    Vec,
};

mod mat;
mod read_into;
mod stdio;
mod stream;
