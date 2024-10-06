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
#![forbid(rustdoc::all)]

//! A utility library for reading data from input and writing data to output.
//!
//! # Principles
//!
//! - **Simple**: You can read and write data with a single line of code.
//! - **Flexible**: You can customize the format of data if the default format does not meet your needs.
//! - **Efficient**: You can read and write data with minimal overhead.
//! - **Safe**: You can read and write data without worrying about buffer overflow or other security issues.
//! - **Easy to Learn**: You can read and write data with a similar interface to Python3 and C++.
//! - **Extensible**: You can implement your own types to read and write data.
//! - **Compatible**: You can read and write data with types that implement [std::fmt::Display] and [std::str::FromStr].
//! - **Human Readable**: You can read and write data in a human-readable format.
//!
//!   For types whose representation is longer than 1 character, the default separator is a space; otherwise, it is an empty string, which means that there is no separator.
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
#![doc = include_str!("../examples/doc_read.txt")]
//! ```
//!
//! ```rust,no_run
#![doc = include_str!("../examples/doc_read.rs")]
//! ```
//!
//! ## [get_line] and [get_line_some]
//!
//! You can use [get_line] functions to read a line of string from current position of cursor in [standard input](std::io::Stdin) to the end of the line.
//!
//! Given the input below:
//!
//! ```txt
#![doc = include_str!("../examples/doc_get_line.txt")]
//! ```
//!
//! ```rust,no_run
#![doc = include_str!("../examples/doc_get_line.rs")]
//! ```
//!
//! *You may have noticed that the [get_line] function is similar to the [`input`](https://docs.python.org/zh-cn/3/library/functions.html#input) function in Python3 and [`std::get_line`](https://zh.cppreference.com/w/cpp/string/basic_string/getline) in C++.*
//!
//! Sometimes you may want to ensure that the line is not empty. You can use [get_line_some] functions to read a non-empty line of string from the position of next non-whitespace character to the end of the line.
//!
//! Given the input below:
//!
//! ```txt
#![doc = include_str!("../examples/doc_get_line_some.txt")]
//! ```
//!
//! ```rust,no_run
#![doc = include_str!("../examples/doc_get_line_some.rs")]
//! ```
//!
//! See [Cursor](#cursor) for more details.
//!
//! ## [show!]
//!
//! You can use [show!] macro to write something to output.
//!
//! - `show!(e)` writes `e` to output. They are formatted with default format.
//! - `show!([a, b], sep = [", "])` writes `[a, b]` like a [`Vec`] to output. They are separated by a comma and a space, as specified in the `sep` parameter.
//! - `show!([[a, b], [c, d]], sep = ["\n", " "])` writes `e` like a [`Mat`] to output. Rows of them are separated by LF, and columns are separated by a space, as specified in the `sep` parameter.
//!
//! Also, you can append a `=>` and a buffer to write to a buffer instead of [standard output](std::io::Stdout), such as `show!(e => buf)` and `show!([a, b], sep = [", "] => buf)`.
//!
//! Note that all parameters are optional and placed after a comma, and the order of parameters does not matter. The default value of `sep` and the default value of `end` are from the [GetDefaultSeparator] trait. See [Separator](#separator) for more details.
//!
//! *You may have noticed that the `show!` macro is similar to the [`print`](https://docs.python.org/zh-cn/3/library/functions.html#print) function in Python.*
//!
//! Code below writes the output to [standard output](std::io::Stdout):
//!
//! ```rust
#![doc = include_str!("../examples/doc_show.rs")]
//! ```
//!
//! And code above generates the output below:
//!
//! ```txt
#![doc = include_str!("../examples/doc_show.txt")]
//! ```
//!
//! [GetDefaultSeparator]: crate::separator::GetDefaultSeparator
//! [Separator]: crate::separator::Separator
//!
//! # Input
//!
//! ## [ReadInto]
//!
//! Some higher-level functions are provided to read data sequence (a single item is also a sequence) from input:
//!
//! - [`read<T>()`](read()) (or [`try_read<T>()`](try_read())) reads a single sequence from input and converts it to a value of `T`.
//! - [`read_n<T>(n)`](read_n()) (or [`try_read_n<T>(n)`](try_read_n())) reads `n` sequences from input and converts them to a value of [Vec].
//! - [`read_m_n<T>(m, n)`](read_m_n()) (or [`try_read_m_n<T>(m, n)`](try_read_m_n())) reads `m * n` sequences from input and converts them to a value of [`Mat<T>`].
//!
//! These functions are implemented for types that implement [ReadInto] trait. Currently, the following types implement [ReadInto] trait:
//!
//! - All types that implement [ReadOneFrom] trait;
//! - `[T; N]` where `T` implements [ReadInto] trait;
//! - `Box<[T; N]>` where `T` implements [ReadInto] trait.
//! - Tuple types, e.g., `(T1, T2, ..., Tn)`, where `Ti` implements [ReadInto] trait and `n` is neither 0 nor more than 12.
//! - ...
//!
//! ## [ReadOneFrom]
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
//! - [`read_any_in_line<T>()`] (or [`try_read_any_in_line<T>()`]) reads all data items in current line from input and converts them to a value of [Vec].
//! - [`read_some_in_line<T>()`] (or [`try_read_some_in_line<T>()`]) reads all data items in the next non-empty line from input and converts them to a value of [Vec].
//!
//! These functions are implemented for types that implement [ReadOneFrom] trait. Currently, the following types in [std] (or [core]) implement [ReadOneFrom] trait:
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
//! And you can implement [ReadOneFrom] trait for your own types by implementing [ReadOneFrom::parse] method. For [FromStr] types, you can use the macro [impl_read_into_single!].
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
#![doc = include_str!("../examples/doc_fn_read.txt")]
//! ```
//!
//! Code below reads the input and stores it in variables:
//!
//! ```rust,no_run
#![doc = include_str!("../examples/doc_fn_read.rs")]
//! ```
//!
//! # Output
//!
//! ## [SepBy] and [sep_by!]
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
//!   use iof::{sep_by, SepBy};
//!   use std::collections::{BTreeMap, BTreeSet};
//!
//!   let v = vec![1, 2, 3];
//!   let s = v.sep_by(", ").to_string();
//!   assert_eq!(s, "1, 2, 3");
//!
//!   let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
//!   let s = sep_by!(v, "\n", ", ");
//!   // Above line is equivalent to:
//!   // let s = v.map(|e| e.sep_by("\n")).sep_by(" ");
//!   assert_eq!(s.to_string(), "1, 2, 3\n4, 5, 6");
//!
//!   let v = BTreeSet::from_iter([3, 1, 2, 4]);
//!   let s = v.sep_by(", ").to_string();
//!   assert_eq!(s, "1, 2, 3, 4");
//!
//!   let v = BTreeMap::from_iter([(3, "w"), (1, "x"), (2, "y"), (4, "z")]);
//!   let s = v.iter().map(|(k, v)| format!("{} -> {}", k, v)).sep_by("\n").to_string();
//!   assert_eq!(s, "1 -> x\n2 -> y\n3 -> w\n4 -> z");
//!   ```
//!
//!   Note that the iterator must implement [Clone] trait to use the [SepBy] trait.
//!
//! ## [WriteInto]
//!
//! Some higher-level functions are provided to write data sequence with default format to output:
//!
//! - [WriteInto::try_write()] writes to [standard output](std::io::Stdout) with default format.
//! - [WriteInto::try_write_into()] writes to given buffer that implements [std::io::Write] with default format.
//! - [WriteInto::try_write_into_string()] writes to a new string with default format.
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
//! ## Separator
//!
//! The [separator] is a string that separates data items. It can be a single character, a string, or a slice of strings.
//!
//! The default separator from [GetDefaultSeparator] is defined as follows:
//!
//! - For all types whose dimension is 0, it uses `[]`;
//! - For all types whose dimension is 1 and `T` must be separated by a space, it uses `[" "]`;
//! - For all types whose dimension is 1 and `T` need not be separated by a space, it uses `[""]`;
//! - For all types whose dimension is 2 and `T` must be separated by a space, it uses `["\n", " "]`;
//! - For all types whose dimension is 2 and `T` need not be separated by a space, it uses `["\n", ""]`.
//! - ...
//!
//! The dimension of a type is the number of dimensions of the data sequence. For example, the dimension of a primitive type `T` is 0, the dimension of [`Vec<T>`] is 1, and the dimension of [`Mat<T>`] is 2.
//!
//! # Notes
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
//! For character streams, The cursor is the position of the next character to be read. It is at the beginning of the input stream initially, and it moves forward as data items are read.
//!
//! In general, every call to a read function that consume a data item will consume the input up to the next whitespace character (but white spaces after the data item will not be consumed), and every call to a read function that reads a line will consume the input up to the next newline character (and then the cursor will be at the beginning of the next line).
//!
//! It's sometimes a little tricky to determine the position of the cursor. For example, given the input below:
//!
//! ```txt
//! 1 2 3
//! 4 5 6
//! ```
//!
//! If you call [`read_one<String>()`] for 3 times and [`read_in_line_trimmed<String>()`] for 1 time, they will read `1`, `2`, `3`, and an empty string respectively. Therefore it's generally unrecommended to use [`read_in_line_trimmed<String>()`] and similar functions that read a possibly empty line of string without specifying the number of data items to read.
pub use {
    crate as iof,
    formatted::SepBy,
    mat::Mat,
    read::{
        error::ReadError,
        read_from::{ReadFrom, ReadFromError},
        read_into::ReadInto,
        read_one_from::{ReadOneFrom, ReadOneFromError},
        read_one_into::ReadOneInto,
    },
    stdio::{read_into::*, stdin, stdout, stream::*},
    stream::{input_stream::InputStream, traits::BufReadExt},
    write::{dimension, separator, writer::Writer, WriteInto},
    Vec,
};

mod array;
mod formatted;
mod mat;
mod read;
mod stdio;
mod stream;
mod write;

/// Unwrap a result or panic with the error message.
#[macro_export]
macro_rules! unwrap {
    ($result:expr) => {
        $result.unwrap_or_else(|err| ::std::panic!("{err}"))
    };
}
