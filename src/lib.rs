#![forbid(missing_docs)]
#![forbid(rust_2021_compatibility, rust_2018_idioms, future_incompatible)]
#![forbid(unused_imports, unused_qualifications, unused_results)]
#![forbid(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::cargo
)]
#![forbid(clippy::should_panic_without_expect, clippy::incompatible_msrv)]
//! A utility library for reading data from input
//! and writting data from output.
//!
//! # Input
//!
//! Some lower-level functions are provided to read a single data item from input:
//!
//! - [read_single<T>()] (or [try_read_single<T>()]) reads a ASCII-whitespace-separated fragment of string,
//!   and converts it to a value of `T`.
//!
//!   The fragment is a string that does not contain any ASCII whitespace characters, and
//!   ASCII whitespace characters here are defined as `[' ', '\t', '\n', '\r']`.
//!
//!   For example, given the input below:
//!
//!   ```txt
//!   1 2,3
//!   3 4;sa
//!   ```
//!
//!   If you call [read_single<T>()] for 4 times, it will read 4 string fragments `1`, `2,3`, `3`, and `4;sa`.
//!
//! - [read_line<T>()] (or [try_read_line<T>()]) reads a non-empty line of string from input,
//!   trims the trailing newline, and converts it to a value of `T`.
//!
//!   If all characters in remained part of current line are ASCII whitespace characters,
//!   it will discard current line and read one more line, otherwise it will return remained part of current line.
//!
//!   For example, given the input below:
//!
//!   ```txt
//!   1 2 3
//!   4 5 6
//!   ```
//!
//!   If you call [read_line<T>()] for 2 times, it will read `1 2 3` and `4 5 6` as two lines of string.
//!
//!   And given the input below:
//!
//!   ```txt
//!   1 2 3
//!   ```
//!
//!   If you call [read_single<T>()] once and [read_line<T>()] once, they will read `1` and `2 3` respectively.
//!
//!   If you call [read_line<T>()] once more, it will panic because there is no non-empty line remained.
//!
//!   Again, given the input below:
//!
//!   ```txt
//!   1 2 3
//!   4 5 6
//!   ```
//!
//!   If you call [read_single<T>()] for 3 times and [read_line<T>()] for 1 time, they will read `1`, `2`, `3`, and `4 5 6` respectively.
//! - [read_remained_line<T>()] (or [try_read_remained_line<T>()]) reads the remained line from input regardless of whether it is empty or not,
//!   trims the trailing newline, and converts it to a value of `T`.
//!
//!   For example, given the input below:
//!
//!   ```txt
//!   1 2 3
//!   4 5 6
//!   ```
//!
//!   If you call [read_single<T>] for 3 times and [read_remained_line<T>] for 1 time, they will read `1`, `2`, `3`, and an empty string respectively.
//!
//!   If you call [read_remained_line<T>] once more, it will still read an empty string.
//!
//! - [read_char<T>] (or [try_read_char<T>]) reads a single non-ASCII-whitespace character from input and converts it to a value of `T`.
//!
//!   For example, given the input below:
//!
//!   ```txt
//!   1 2 3
//!   ```
//!
//!   If you call [read_char] for 3 times, it will read `1`, `2`, and `3` as three characters.
//!
//! These functions are implemented for types that implement [ReadIntoSingle] trait. Currently, the following types implement [ReadIntoSingle] trait:
//!
//! - [String];
//! - [char];
//! - [u8], [u16], [u32], [u64], [u128], [usize];
//! - [i8], [i16], [i32], [i64], [i128], [isize];
//! - [f32], [f64];
//! - [bool];
//! - [std::num::NonZeroU8], [std::num::NonZeroU16], [std::num::NonZeroU32], [std::num::NonZeroU64], [std::num::NonZeroU128], [std::num::NonZeroUsize];
//! - [std::num::NonZeroI8], [std::num::NonZeroI16], [std::num::NonZeroI32], [std::num::NonZeroI64], [std::num::NonZeroI128], [std::num::NonZeroIsize];
//! - ...
//!
//! Some higher-level functions are provided to read data sequence (a single item is also a sequnce) from input:
//!
//! - [read<T>()] (or [try_read()]) reads a single sequence from input and converts it to a value of `T`.
//! - [read_n<T>(n)] (or [try_read_n<T>(n)]) reads `n`` sequences from input and converts them to a value of [Vec<T>].
//! - [read_m_n<T>(m, n)] (or [try_read_m_n<T>(m, n)]) reads `m * n` sequences from input and converts them to a value of [Mat<T>].
//!
//! These functions are implemented for types that implement [ReadInto] trait. Currently, the following types implement [ReadInto] trait:
//!
//! - All types that implement [ReadIntoSingle] trait;
//! - `[T; N]` where `T` implements [ReadInto] trait;
//! - `Box<[T; N]>` where `T` implements [ReadInto] trait.
//! - Tuple types, e.g., `(T1, T2, ..., Tn)`, where `Ti` implements [ReadInto] trait and `n` is neither 0 nor more than 12.
//! - ...
//!
//! ## Complex Examples for Input
//!
//! Function [read] is the simplest way to read data from [standard input](std::io::Stdin).
//!
//! Given the input below:
//!
//! ```txt
//! 42
//! Hello, World!
//! 1 2 3 4
//! 1 2
//! 3 4
//! ```
//!
//! Code below reads the input and stores it in variables:
//!
//! ```rust,no_run
//! use iof::read;
//!
//! // Read a single integer from input, whose type is inferred from the context.
//! let n: u32 = read();
//! assert_eq!(n, 42);
//!
//! // Read a string from input.
//! let s: String = read();
//! assert_eq!(s, "Hello, World!");
//!
//! // Read an array of integers from input.
//! let arr: [u32; 4] = read();
//! assert_eq!(arr, [1, 2, 3, 4]);
//!
//! // Read a nested array of integers from input.
//! let arr: [[u32; 2]; 2] = read();
//! assert_eq!(arr, [[1, 2], [3, 4]]);
//! ```
//!
//! # Output
//!
//! Some lower-level functions are provided to write a data sequence with customizing format to output:
//!
//! - [SepBy::sep_by()] writes a sequence of data items, which implements [IntoIterator], to output with a separator.
//!
//!   There won't be any separator before the first item or after the last item.
//!
//!   For example, given the code below:
//!
//!   ```rust
//!   use iof::SepBy;
//!   let v = vec![1, 2, 3];
//!   let s = format!("{}", v.sep_by(", "));
//!   assert_eq!(s, "1, 2, 3");
//!   ```

pub use {
    formatted::SepBy,
    mat::Mat,
    read_into::{from_str::FromStr, ReadInto, ReadIntoError, ReadIntoSingle},
    stdio::read_into::*,
    stream::{InputStream, OutputStream},
    write_into::{display::Display, WriteInto},
};

mod array;
mod formatted;
mod mat;
mod read_into;
mod sep_by;
mod stdio;
mod stream;
mod write_into;
