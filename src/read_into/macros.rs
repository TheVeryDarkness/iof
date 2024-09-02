#[macro_export]
/// Read a single data item, a [Vec] or a [Mat] from input.
///
/// - `read!()` reads a single data item from input.
/// - `read!(n)` reads `n` data items from input and stores them in a [Vec].
/// - `read!(m, n)` reads `m * n` data items from input and stores them in a [Mat].
///
/// [Mat]: crate::Mat
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
        $crate::read()
    };
    ($dim0:expr $(, $dims:expr)* $(,)?) => {{
        let range = 0usize..$dim0;
        ::std::vec::Vec::<_>::from_iter(range.map(|_| $crate::read!($($dims)*)))
    }};
}

/// Implement [ReadInto] for given types that already implement [std::str::FromStr].
///
/// [ReadInto]: crate::ReadInto
#[macro_export]
macro_rules! impl_read_into {
    (char $($tys:ident)*) => {
        impl<B: ::std::io::BufRead> $crate::ReadIntoSingle<char> for $crate::InputStream<B> {
            type Error = $crate::ReadIntoError<<char as ::std::str::FromStr>::Err>;

            fn parse(s: &str) -> Result<char, Self::Error> {
                s.parse().map_err($crate::ReadIntoError::FromStrError)
            }

            fn try_read_one(&mut self) -> Result<char, Self::Error> {
                <Self as $crate::ReadIntoSingle<char>>::try_read_in_char(self)
            }
        }
        $crate::impl_read_into!($($tys)*);
    };
    ($ty:ident $($tys:ident)*) => {
        impl<B: ::std::io::BufRead> $crate::ReadIntoSingle<$ty> for $crate::InputStream<B> {
            type Error = $crate::ReadIntoError<<$ty as ::std::str::FromStr>::Err>;

            fn parse(s: &str) -> Result<$ty, Self::Error> {
                s.parse().map_err($crate::ReadIntoError::FromStrError)
            }
        }
        $crate::impl_read_into!($($tys)*);
    };
    () => {};
}
