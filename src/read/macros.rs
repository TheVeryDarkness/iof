/// Read a single data item, a [Vec] or a [Mat] from input using [ReadInto].
///
/// The intended grammar is:
///
/// ```rust,ignore
/// $($dims:expr),* $(,)? $(; src = $src:expr)? $(; fmt = $fmt:expr)?
/// ```
///
/// - `read!()` reads a single data item from input.
/// - `read!(n)` reads `n` data items from input and stores them in a [Vec].
/// - `read!(m, n)` reads `m * n` data items from input and stores them in a [Mat],
///   
///   which consists of `m` [Vec]s, each containing `n` data items.
///
/// And the reader will always respect your size hint, and if you pass a size larger that the actual data in current line, it will go on to the next line and read as many as the size hint.
///
/// [Mat]: crate::Mat
/// [ReadInto]: crate::ReadInto
///
/// # Example
///
/// ```rust,no_run
/// use iof::read;
/// let a: usize = read!();
/// let b: Vec<usize> = read!(3);
/// let c: Vec<Vec<usize>> = read!(2, 3);
/// ```
///
/// # Notes
///
/// This macro accepts even higher dimensions, such as `read!(m, n, o, p)`,
/// but as this creates a nested [Vec], this may cause performance concerns.
///
/// What's more, you can pass a dynamic value to `read!` like `read!(m, f())`,
/// which can create a nested [Vec] with a non-uniform length.
///
/// Given the input:
///
/// ```txt
#[doc = include_str!("../../examples/doc_macro_read.txt")]
/// ```
///
/// ```rust,no_run
#[doc = include_str!("../../examples/doc_macro_read.rs")]
/// ```
///
/// Also, you can specify the source (as long as it implements [BufReadExt](crate::BufReadExt)) and format (as long as it implements [Format]) for reading.
///
/// [BufReadExt]: crate::BufReadExt
/// [Format]: crate::fmt::Format
///
/// ```rust
/// use iof::{fmt::csv, read, InputStream, Mat};
///
/// let a: usize = read!(; src = InputStream::new(b"42".as_slice()));
/// assert_eq!(a, 42);
///
/// let b: Vec<usize> = read!(3; src = InputStream::new(b"1 2 3".as_slice()));
/// assert_eq!(b, [1, 2, 3]);
///
/// let b: Vec<usize> = read!(3; src = InputStream::new(b"1, 2, 3".as_slice()); fmt = csv());
/// assert_eq!(b, [1, 2, 3]);
///
/// let s = b"010\n101";
///
/// let b: Mat<char> = read!(2, 3; src = InputStream::new(s.as_slice()); skip = [' ', ',', ';', '\n', '\r']);
/// assert_eq!(b, [['0', '1', '0'], ['1', '0', '1']]);
///
/// let s = b"1,2,3;4,5,6\r\n";
///
/// let b: Mat<usize> = read!(2, 3; src = InputStream::new(s.as_slice()); skip = [' ', ',', ';', '\r', '\n']);
/// assert_eq!(b, [[1, 2, 3], [4, 5, 6]]);
///
/// let b: Mat<usize> = read!(2, 3; src = InputStream::new(s.as_slice()); skip = " \t,;\r\n".chars());
/// assert_eq!(b, [[1, 2, 3], [4, 5, 6]]);
/// ```
#[macro_export]
macro_rules! read {
    (@ $(,)?; src = $src:expr; fmt = $fmt:expr) => {
        $crate::unwrap!($crate::ReadFrom::try_read_from($src, $fmt))
    };
    (@ $dim0:expr $(, $dims:expr)* $(,)?; src = $src:expr; fmt = $fmt:expr) => {{
        let range = 0usize..$dim0;
        ::std::vec::Vec::<_>::from_iter(range.map(|_| $crate::read!(@ $($dims, )* ; src = $src ; fmt = $fmt)))
    }};
    ($(,)? $(; src = $src:expr)? $(; fmt = $fmt:expr)?) => {{
        let src = $crate::argument_or_default!($(&mut $src)?, &mut *$crate::stdin());
        let fmt = $crate::argument_or_default!($(&$fmt)?, &$crate::fmt::Default::new());
        $crate::unwrap!($crate::ReadFrom::try_read_from(src, fmt))
    }};
    ($dim0:expr $(, $dims:expr)* $(,)? $(; src = $src:expr)? $(; fmt = $fmt:expr)?) => {{
        let range = 0usize..$dim0;
        let src = $crate::argument_or_default!($(&mut $src)?, &mut *$crate::stdin());
        let fmt = $crate::argument_or_default!($(&$fmt)?, &$crate::fmt::Default::new());
        ::std::vec::Vec::<_>::from_iter(range.map(|_| $crate::read!(@ $($dims, )*; src = src; fmt = fmt)))
    }};
    ($(,)? $(; src = $src:expr)? ; skip = $skip:expr) => {{
        let src = $crate::argument_or_default!($(&mut $src)?, &mut *$crate::stdin());
        let fmt = &$crate::fmt::skip($skip);
        $crate::unwrap!($crate::ReadFrom::try_read_from(src, fmt))
    }};
    ($dim0:expr $(, $dims:expr)* $(,)? $(; src = $src:expr)? ; skip = $skip:expr) => {{
        let range = 0usize..$dim0;
        let src = $crate::argument_or_default!($(&mut $src)?, &mut *$crate::stdin());
        let fmt = &$crate::fmt::skip($skip);
        ::std::vec::Vec::<_>::from_iter(range.map(|_| $crate::read!(@ $($dims, )*; src = src; fmt = fmt)))
    }};
}

/// Implement [ReadOneFrom] for given types that already implement [std::str::FromStr].
///
/// The intended grammar is:
///
/// ```rust,ignore
/// $($ty:ty $(=> $accept:pat)?)*
/// ```
///
/// Note that all types that are [ReadOneFrom] will also implement [ReadInto] automatically.
///
/// [ReadOneFrom]: crate::ReadOneFrom
/// [ReadInto]: crate::ReadInto
#[macro_export]
macro_rules! impl_read_one_from_for_from_str {
    ($($ty:ty)+) => {
        $(
            impl $crate::ReadOneFrom for $ty {
                type ParseError = <Self as ::core::str::FromStr>::Err;

                #[inline]
                fn parse(s: &::core::primitive::str) -> Result<Self, $crate::ReadOneFromError<Self>> {
                    s.parse().map_err(|err| $crate::ReadError::FromStrError(err, s.to_owned(), ::core::any::type_name::<Self>()))
                }
            }
        )*
    };
    ($($ty:ty)+ => $accept:expr) => {
        $(
            impl $crate::ReadOneFrom for $ty {
                type ParseError = <Self as ::core::str::FromStr>::Err;

                #[inline]
                fn accept() -> impl $crate::ext::Pattern<Item = ::core::primitive::char> {
                    $accept
                }

                #[inline]
                fn parse(s: &::core::primitive::str) -> Result<Self, $crate::ReadOneFromError<Self>> {
                    s.parse().map_err(|err| $crate::ReadError::FromStrError(err, s.to_owned(), ::core::any::type_name::<Self>()))
                }
            }
        )*
    };
}
