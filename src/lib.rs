#![forbid(missing_docs)]
#![forbid(rust_2021_compatibility, rust_2018_idioms, future_incompatible)]
#![warn(
    unused_imports,
    unused_qualifications,
    unused_results,
    unused_comparisons
)]
#![forbid(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::cargo
)]
#![forbid(clippy::should_panic_without_expect, clippy::incompatible_msrv)]
//! A utility library for reading data from input and writting data to output.
//!
//! # In Short
//!
//! ## [read!]
//!
//! You can use [read!] macro to read a single data item, a [Vec] or a [Mat] from input.
//!
//! - `read!()` reads a single data item from input.
//! - `read!(n)` reads `n` data items from input and stores them in a [Vec].
//! - `read!(m, n)` reads `m * n` data items from input and stores them in a [Mat].
//!
//! Given the input below:
//!
//! ```txt
//! 42 abc def
//! 0 0.3 lmn
//! 1 2 3
//! 1 2 3
//! 4 5 6
//! .@/#$
//! !@#!@
//! *&@:,
//! ```
//!
//! ```rust,no_run
//! use iof::{read, Mat};
//!
//! // Read a single integer from input.
//! let n: u32 = read!();  
//! assert_eq!(n, 42);
//!
//! // Read a single string from input.
//! let n: String = read!();
//! assert_eq!(n, "abc");
//!
//! // Read a tuple from input.
//! let (l, m, n): (u32, f64, String) = read!();
//! assert_eq!(l, 0);
//! assert_eq!(m, 0.3);
//! assert_eq!(n, "lmn");
//!
//! // Read a vector of charcters from input.
//! let v: Vec<char> = read!();
//! assert_eq!(v, ['d', 'e', 'f']);
//!  
//! // Read a vector of integers from input.
//! let v: Vec<u32> = read!(3);
//! assert_eq!(v, [1, 2, 3]);
//!
//! // Read a matrix of integers from input.
//! let m: Mat<u32> = read!(2, 3);
//! assert_eq!(m, [[1, 2, 3], [4, 5, 6]]);
//!
//! // Read a matrix of characters from input.
//! let m: Mat<char> = read!(3, 5);
//! assert_eq!(m, [['.', '@', '/', '#', '$'], ['!', '@', '#', '!', '@'], ['*', '&', '@', ':', ',']]);
//! ```
//!
//! ## [get_line] and [get_line_some]
//!
//! You can use [get_line] functions to read a line of string from current position of cursor in [standard input](std::io::Stdin) to the end of the line.
//!
//! Given the input below:
//!
//! ```txt
//! 42
//! abc
//! ```
//!
//! ```rust,no_run
//! use iof::{get_line, read};
//!
//! // Read a single string from input.
//! let s: String = read!();
//! assert_eq!(s, "42");
//!
//! // Read a line of string from input.
//! let s: String = get_line();
//! assert_eq!(s, "");
//!
//! // Read a line of string from input.
//! let s: String = get_line();
//! assert_eq!(s, "abc");
//! ```
//!
//! *You may have noticed that the [get_line] function is similar to the [`input`](https://docs.python.org/zh-cn/3/library/functions.html#input) function in Python and [`std::get_line`](https://zh.cppreference.com/w/cpp/string/basic_string/getline) in cpp.*
//!
//! Sometimes you may want to ensure that the line is not empty. You can use [get_line_some] functions to read a non-empty line of string from the position of next non-whitespace character to the end of the line.
//!
//! Given the input below:
//!
//! ```txt
//! 42
//! abc
//! ```
//!
//! ```rust,no_run
//! use iof::{read, get_line_some};
//!
//! // Read a single string from input.
//! let s: String = read();
//! assert_eq!(s, "42");
//!
//! // Read a non-empty line of string from input.
//! let s: String = get_line_some();
//! assert_eq!(s, "abc");
//! ```
//!
//! See [Cursor](#cursor) for more details.
//!
//! ## [show!]
//!
//! You can use [show!] macro to write a single data item, a [Vec] or a [Mat] to output.
//!
//! - `show!(e)` writes a single data item `e` to output.
//! - `show!(e1, e2, ...)` writes several data items `e1, e2, ...` to output. They are separated by a space.
//! - `show!(e1, e2, ...; sep=", ")` writes several data items `e1, e2, ...` to output. They are separated by a comma and a space, as specified in the `sep` parameter.
//! - `show!(e1, e2, ...; sep=", ", end="!")` writes several data items `e1, e2, ...` to output. They are separated by a comma and a space, as specified in the `sep` parameter, and ended with an exclamation mark, as specified in the `end` parameter.
//!
//! Note that all parameter are optional and placed after a semicolon, and the order of parameters does not matter. The default value of `sep` is a space (`' '`), and the default value of `end` is a newline (`'\n'`).
//!
//! *You may have noticed that the `show!` macro is similar to the [`print`](https://docs.python.org/zh-cn/3/library/functions.html#print) function in Python.*
//!
//! # Input
//!
//! ## [ReadInto]
//!
//! Some higher-level functions are provided to read data sequence (a single item is also a sequnce) from input:
//!
//! - [`read<T>()`] (or [`try_read<T>()`]) reads a single sequence from input and converts it to a value of `T`.
//! - [`read_n<T>(n)`] (or [`try_read_n<T>(n)`]) reads `n`` sequences from input and converts them to a value of [Vec].
//! - [`read_m_n<T>(m, n)`] (or [`try_read_m_n<T>(m, n)`]) reads `m * n` sequences from input and converts them to a value of [`Mat<T>`].
//!
//! These functions are implemented for types that implement [ReadInto] trait. Currently, the following types implement [ReadInto] trait:
//!
//! - All types that implement [ReadIntoOne] trait;
//! - `[T; N]` where `T` implements [ReadInto] trait;
//! - `Box<[T; N]>` where `T` implements [ReadInto] trait.
//! - Tuple types, e.g., `(T1, T2, ..., Tn)`, where `Ti` implements [ReadInto] trait and `n` is neither 0 nor more than 12.
//! - ...
//!
//! ## [ReadIntoOne]
//!
//! Some lower-level functions are provided to read a single data item from input:
//!
//! - [`read_one<T>()`] (or [`try_read_one<T>()`]) reads a ASCII-whitespace-separated fragment of string (for [char], it reads a single non-ASCII-whitespace character instead),
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
//!   If you call [`read_one<String>()`] for 4 times, it will read 4 string fragments `1`, `2,3`, `3`, and `4;sa`.
//!
//! - [`read_in_line_some_trimmed<T>()`] (or [`try_read_in_line_some_trimmed<T>()`]) reads a non-empty line of string from input,
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
//!   If you call [`read_in_line_some_trimmed<T>()`] for 2 times, it will read `1 2 3` and `4 5 6` as two lines of string.
//!
//!   And given the input below:
//!
//!   ```txt
//!   1 2 3
//!   ```
//!
//!   If you call [`read_one<T>()`] once and [`read_in_line_some_trimmed<T>()`] once, they will read `1` and `2 3` respectively.
//!
//!   If you call [`read_in_line_some_trimmed<T>()`] once more, it will panic because there is no non-empty line remained.
//!
//!   Again, given the input below:
//!
//!   ```txt
//!   1 2 3
//!   4 5 6
//!   ```
//!
//!   If you call [`read_one<T>()`] for 3 times and [`read_in_line_some_trimmed<T>()`] for 1 time, they will read `1`, `2`, `3`, and `4 5 6` respectively.
//! - [`read_in_line_trimmed<T>()`] (or [`read_in_line_trimmed<T>()`]) reads the remained line from input regardless of whether it is empty or not,
//!   trims the trailing newline, and converts it to a value of `T`.
//!
//!   For example, given the input below:
//!
//!   ```txt
//!   1 2 3
//!   4 5 6
//!   ```
//!
//!   If you call [`read_one<T>`] for 3 times and [`read_in_line_trimmed<T>`] for 1 time, they will read `1`, `2`, `3`, and an empty string respectively.
//!
//!   If you call [`read_in_line_trimmed<T>`] once more, it will still read an empty string.
//!
//! - [`read_in_char<T>`] (or [`try_read_in_char<T>`]) reads a single non-ASCII-whitespace character from input and converts it to a value of `T`.
//!
//!   For example, given the input below:
//!
//!   ```txt
//!   1 2 3
//!   ```
//!
//!   If you call [`read_in_char`] for 3 times, it will read `1`, `2`, and `3` as three characters.
//!
//! - [`read_all<T>()`] (or [`try_read_all<T>()`]) reads all remaining data items from input and converts them to a value of [Vec].
//! - [`read_all_in_line<T>()`] (or [`try_read_all_in_line<T>()`]) reads all data items in current line from input and converts them to a value of [Vec].
//! - [`read_all_in_line_some<T>()`] (or [`try_read_all_in_line_some<T>()`]) reads all data items in the next non-empty line from input and converts them to a value of [Vec].
//!
//! These functions are implemented for types that implement [ReadIntoOne] trait. Currently, the following types in [std] (or [core]) implement [ReadIntoOne] trait:
//!
//! - [String];
//! - [char] (but it has different behavior from other types);
//! - [u8], [u16], [u32], [u64], [u128], [usize];
//! - [i8], [i16], [i32], [i64], [i128], [isize];
//! - [f32], [f64];
//! - [bool];
//! - [NonZeroU8], [NonZeroU16], [NonZeroU32], [NonZeroU64], [NonZeroU128], [NonZeroUsize];
//! - [NonZeroI8], [NonZeroI16], [NonZeroI32], [NonZeroI64], [NonZeroI128], [NonZeroIsize];
//! - ...
//!
//! And you can implement [ReadIntoOne] trait for your own types by implementing [ReadIntoOne::parse] method. For [FromStr] types, you can use the macro [impl_read_into_single!].
//!
//! [FromStr]: std::str::FromStr
//!
//! ## Complex Examples for Input
//!
//! Function [read()] is the simplest way to read data from [standard input](std::io::Stdin).
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
//! // Read a single integer from input.
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
//! ## [SepBy]
//!
//! Some lower-level functions are provided to write a data sequence with customizing format to output:
//!
//! - [SepBy::sep_by()] writes a sequence of data items, which implements [IntoIterator], to output with a separator.
//!
//!   There won't be any separator before the first item or after the last item.
//!
//!   For example:
//!
//!   ```rust
//!   use iof::SepBy;
//!   let v = vec![1, 2, 3];
//!   let s = format!("{}", v.sep_by(", "));
//!   assert_eq!(s, "1, 2, 3");
//!   ```
//!
//! ## [WriteInto]
//!
//! Some higher-level functions are provided to write data sequence with default format to output:
//!
//! - [WriteInto::write()] (or [WriteInto::try_write()]) writes to [standard output](std::io::Stdout) with default format.
//! - [WriteInto::write_into()] (or [WriteInto::try_write_into()]) writes to given buffer that implements [std::io::Write] with default format.
//! - [WriteInto::write_into_string()] (or [WriteInto::try_write_into_string()]) writes to a new string with default format.
//!   
//! The default format is defined as follows:
//!
//! - For types that implement [Display] trait (but we only implement [WriteInto] for a part of types that implement [Display]), it uses [Display::fmt] method;
//!    - For [String], it writes the string as is;
//!    - For [char], it writes the character as is;
//!    - For [u8], [u16], [u32], [u64], [u128], [usize], [i8], [i16], [i32], [i64], [i128], [isize], [f32], [f64], [bool], [NonZeroU8], [NonZeroU16], [NonZeroU32], [NonZeroU64], [NonZeroU128], [NonZeroUsize], [NonZeroI8], [NonZeroI16], [NonZeroI32], [NonZeroI64], [NonZeroI128], [NonZeroIsize], it writes the value as is in decimal format;
//! - For `[T]`, `[T; N]` and [Vec] where `T` implements [WriteInto] trait, it writes each item in the vector with a space as separator;
//! - For [Mat] where `T` implements [WriteInto] trait, it writes each row in the matrix with a newline as separator, and writes each item in a row with a space as separator;
//! - For all `&T` where `T` implements [WriteInto] trait, it writes the value as is.
//!
//! [Display]: std::fmt::Display
//! [Display::fmt]: std::fmt::Display::fmt
//!
//! ## [WriteOneInto]
//!
//! Some lower-level functions are provided to write a single data item to output:
//!
//! - [WriteOneInto::write_one_into()] (or [WriteOneInto::try_write_one_into()]) writes to given buffer that implements [std::io::Write].
//!
//! [NonZeroU8]: std::num::NonZeroU8
//! [NonZeroU16]: std::num::NonZeroU16
//! [NonZeroU32]: std::num::NonZeroU32
//! [NonZeroU64]: std::num::NonZeroU64
//! [NonZeroU128]: std::num::NonZeroU128
//! [NonZeroUsize]: std::num::NonZeroUsize
//! [NonZeroI8]: std::num::NonZeroI8
//! [NonZeroI16]: std::num::NonZeroI16
//! [NonZeroI32]: std::num::NonZeroI32
//! [NonZeroI64]: std::num::NonZeroI64
//! [NonZeroI128]: std::num::NonZeroI128
//! [NonZeroIsize]: std::num::NonZeroIsize
//!
//! # Warning
//!
//! ## Concurrency
//!
//! Take care when using this library in a multi-threaded environment, as the standard input/output streams are shared among all threads. See [Stdin] and [Stdout] for more details.
//!
//! [Stdin]: std::io::Stdin
//! [Stdout]: std::io::Stdout
//!
//! ## Cursor
//!
//! For character streams, it's sometimes a little tricky to determine the position of the cursor. For example, given the input below:
//!
//! ```txt
//! 1 2 3
//! 4 5 6
//! ```
//!
//! If you call [`read_one<String>`] for 3 times and [`read_in_line_trimmed<String>`] for 1 time, they will read `1`, `2`, `3`, and an empty string respectively. Therefore it's generally unrecommended to use [`read_in_line_trimmed<String>()`] and similar functions that read a possibly empty line of string without specifying the number of data items to read.
//!
//! In general, every call to a read function that consume a data item will consume the input up to the next whitespace character (but whitespaces after the data item will not be consumed), and every call to a read function that reads a line will consume the input up to the next newline character (and then the cursor will be at the beginning of the next line).
pub use {
    crate as iof,
    formatted::SepBy,
    mat::Mat,
    read_into::{error::ReadIntoError, ReadInto, ReadIntoOne},
    stdio::{read_into::*, stdin, stdout, stream::*},
    stream::{BufReadExt, InputStream},
    write_into::{WriteInto, WriteOneInto},
};

mod array;
mod formatted;
mod mat;
mod read_into;
mod sep_by;
mod stdio;
mod stream;
mod write_into;
