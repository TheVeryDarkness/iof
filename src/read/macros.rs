#[macro_export]
/// Read a single data item, a [Vec] or a [Mat] from input using [ReadInto].
///
/// - `read!()` reads a single data item from input.
/// - `read!(n)` reads `n` data items from input and stores them in a [Vec].
/// - `read!(m, n)` reads `m * n` data items from input and stores them in a [Mat].
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
macro_rules! read {
    () => {
        $crate::unwrap!($crate::try_read())
    };
    ($dim0:expr $(, $dims:expr)* $(,)?) => {{
        let range = 0usize..$dim0;
        ::std::vec::Vec::<_>::from_iter(range.map(|_| $crate::read!($($dims, )*)))
    }};
}

/// Implement [ReadOneFrom] for given types that already implement [std::str::FromStr].
///
/// Note that all types that are [ReadOneFrom] will also implement [ReadInto] automatically.
///
/// [ReadOneFrom]: crate::ReadOneFrom
/// [ReadInto]: crate::ReadInto
#[macro_export(local_inner_macros)]
macro_rules! impl_read_into_single {
    (char $($tys:ident)*) => {
        impl $crate::ReadOneFrom for char {
            type ParseError = <char as ::std::str::FromStr>::Err;

            fn parse(s: &str) -> Result<char, $crate::ReadOneFromError<Self>> {
                s.parse().map_err(|err| $crate::ReadError::FromStrError(err, s.to_owned(), ::std::any::type_name::<char>()))
            }

            fn try_read_one_from<S: $crate::BufReadExt>(stream: &mut S) -> Result<char, $crate::ReadOneFromError<Self>> {
                <Self as $crate::ReadOneFrom>::try_read_in_char_from(stream)
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
