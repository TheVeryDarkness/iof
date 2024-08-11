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
    formatted::SepBy,
    mat::Mat,
    read_into::{
        read, read_array, read_m_n, read_n, read_tuple, try_read, try_read_array, try_read_m_n,
        try_read_n, try_read_tuple, ReadInto, ReadIntoError,
    },
    stream::InputStream,
};

mod formatted;
mod mat;
mod read_into;
mod sep_by;
mod stdio;
mod stream;
