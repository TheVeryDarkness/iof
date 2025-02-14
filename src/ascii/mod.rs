//! A port of the `ascii` module from Rust's standard library with a little bit of extra functionality.
//!
//! See [issue 110998](https://github.com/rust-lang/rust/issues/110998).
pub(crate) mod char;
pub(crate) mod string;

pub use char::Char;
pub use string::String;
