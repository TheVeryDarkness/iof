#![forbid(missing_docs, rust_2021_compatibility, rust_2018_idioms)]
#![forbid(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::cargo
)]
#![forbid(clippy::should_panic_without_expect, clippy::incompatible_msrv)]
//! A utility library for reading integers, floating numbers and strings from input/output.

pub use {
    formatted::SepBy,
    mat::Mat,
    read_into::{ReadInto, ReadIntoError, ReadIntoSingle},
    stdio::read_into::*,
    stream::InputStream,
};

mod array;
mod formatted;
mod mat;
mod read_into;
mod sep_by;
mod stdio;
mod stream;
