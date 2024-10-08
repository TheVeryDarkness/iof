/// Read a single data item, a [Vec] or a [Mat] from input using [ReadInto].
///
/// - `read!()` reads a single data item from input.
/// - `read!(n)` reads `n` data items from input and stores them in a [Vec].
/// - `read!(m, n)` reads `m * n` data items from input and stores them in a [Mat],
///   
///   which consists of `m` [Vec]s, each containing `n` data items.
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
/// ```rust,no_run
#[doc = include_str!("../../examples/doc_macro_read.rs")]
/// ```
///
/// Also, you can specify the source and locale for reading.
///
/// ```rust,no_run
/// use iof::{InputStream, locale, read};
/// let a: usize = read!(; src = InputStream::new(b"42".as_slice()));
/// assert_eq!(a, 42);
///
/// let b: Vec<usize> = read!(3; src = InputStream::new(b"1 2 3".as_slice()));
/// assert_eq!(b, vec![1, 2, 3]);
///
/// let b: Vec<usize> = read!(3; src = InputStream::new(b"1, 2, 3".as_slice()); loc = locale::CSV);
/// assert_eq!(b, vec![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! read {
    (@; src = $src:expr; loc = $loc:expr) => {
        $crate::unwrap!($crate::ReadFrom::try_read_from($src, $loc))
    };
    (@ $dim0:expr $(, $dims:expr)* $(,)?; src = $src:expr; loc = $loc:expr) => {{
        let range = 0usize..$dim0;
        ::std::vec::Vec::<_>::from_iter(range.map(|_| $crate::read!(@ $($dims, )* ; src = $src ; loc = $loc)))
    }};
    ($(; src = $src:expr)? $(; loc = $loc:expr)?) => {{
        let src = $crate::argument_or_default!($(&mut $src)?, &mut *$crate::stdin());
        let loc = $crate::argument_or_default!($(&$loc)?, &$crate::locale::ASCII);
        $crate::unwrap!($crate::ReadFrom::try_read_from(src, loc))
    }};
    ($dim0:expr $(, $dims:expr)* $(,)? $(; src = $src:expr)? $(; loc = $loc:expr)?) => {{
        let range = 0usize..$dim0;
        let src = $crate::argument_or_default!($(&mut $src)?, &mut *$crate::stdin());
        let loc = $crate::argument_or_default!($(&$loc)?, &$crate::locale::ASCII);
        ::std::vec::Vec::<_>::from_iter(range.map(|_| $crate::read!(@ $($dims, )*; src = src; loc = loc)))
    }};
}

/// Implement [ReadOneFrom] for given types that already implement [std::str::FromStr].
///
/// Note that all types that are [ReadOneFrom] will also implement [ReadInto] automatically.
///
/// [ReadOneFrom]: crate::ReadOneFrom
/// [ReadInto]: crate::ReadInto
#[macro_export]
macro_rules! impl_read_into_single {
    (char $($tys:ident)*) => {
        impl $crate::ReadOneFrom for char {
            type ParseError = <char as ::std::str::FromStr>::Err;

            fn parse(s: &str) -> Result<char, $crate::ReadOneFromError<Self>> {
                s.parse().map_err(|err| $crate::ReadError::FromStrError(err, s.to_owned(), ::std::any::type_name::<char>()))
            }

            fn try_read_one_from<L: $crate::locale::Locale, S: $crate::BufReadExt>(stream: &mut S, locale: &L) -> Result<char, $crate::ReadOneFromError<Self>> {
                <Self as $crate::ReadOneFrom>::try_read_in_char_from(stream, locale)
            }
        }
        $crate::impl_read_into_single!($($tys)*);
    };
    ($ty:ident $($tys:ident)*) => {
        impl $crate::ReadOneFrom for $ty {
            type ParseError = <$ty as ::std::str::FromStr>::Err;

            fn parse(s: &str) -> Result<$ty, $crate::ReadOneFromError<Self>> {
                s.parse().map_err(|err| $crate::ReadError::FromStrError(err, s.to_owned(), ::std::any::type_name::<$ty>()))
            }
        }
        $crate::impl_read_into_single!($($tys)*);
    };
    () => {};
}
