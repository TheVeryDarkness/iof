//! A port of the `ascii` module from Rust's standard library with a little bit of extra functionality.
//!
//! See [issue 110998](https://github.com/rust-lang/rust/issues/110998). Switch to the standard
//! library's `ascii` module when it becomes stable.
pub(crate) mod char;
pub(crate) mod string;

pub use char::Char;
pub use string::String;
