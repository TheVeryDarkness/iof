#![forbid(missing_docs, rust_2021_compatibility, rust_2018_idioms)]
#![forbid(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::cargo
)]
//! A utility library for reading integers, floating numbers and strings from input/output.

pub use {
    mat::Mat,
    read_into::{
        read, read_m_n, read_n, try_read, try_read_m_n, try_read_n, ReadInto, ReadIntoError,
    },
    stream::InputStream,
};

mod mat;
mod read_into;
mod stdio;
mod stream;
