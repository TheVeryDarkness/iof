/// Write the given expression into [standard output](std::io::Stdout) using [WriteInto].
///
/// The intended grammar is:
///
/// ```ignore
/// show!($expr:expr $(, sep=$sep:expr)? $(, end=$end:expr)? $(,)? $(=> $buf:expr)?)
/// ```
///
/// You can configure the writer using the following options:
///
/// - `sep`: Separator between values. Default is `" "`. Provide an instance of [Separators] to use custom separators, and if it has mismatched dimensions, it may use the default separator (if there is one) or panic.
/// - `end`: End of the output. Default is `"\n"`. Provide a string to use a custom end.
/// - `buf`: Buffer to write into. Default is [standard output](crate::stdout). Provide a mutable reference to a buffer that implements [std::io::Write] to write into it.
///
/// ```rust
#[doc = include_str!("../../examples/doc_macro_show.rs")]
/// ```
///
/// [WriteInto]: crate::WriteInto
/// [Separators]: crate::Separators
#[macro_export]
macro_rules! show {
    ($expr:expr $(, sep=$sep:expr)? $(, end=$end:expr)? $(,)? $(=> $buf:expr)?) => {
        $crate::unwrap!(|| -> ::std::io::Result<()> {
            $crate::write(
                &$expr,
                &mut $crate::argument_or_default!($($buf)?, $crate::stdout()),
                $crate::argument_or_default!($(&$sep)?, $crate::DefaultSeparator),
                $crate::argument_or_default!($(&$end)?, "\n"),
            )?;
            Ok(())
        }())
    };
    ($expr:expr, end=$end:expr $(, sep=$sep:expr)? $(,)? $(=> $buf:expr)?) => {
        $crate::unwrap!(|| -> ::std::io::Result<()> {
            $crate::write(
                &$expr,
                &mut $crate::argument_or_default!($($buf)?, $crate::stdout()),
                $crate::argument_or_default!($(&$sep)?, $crate::DefaultSeparator),
                &$end,
            )?;
            Ok(())
        }())
    };
}

/// Return the given expression or the default value.
#[macro_export]
macro_rules! argument_or_default {
    ($arg:expr, $default:expr $(,)?) => {
        $arg
    };
    (, $default:expr $(,)?) => {
        $default
    };
}

/// Implement [WriteInto] for given types that already implements [std::fmt::Display].
///
/// [WriteInto]: crate::WriteInto
#[macro_export]
macro_rules! impl_write_into_for_display {
    ($($ty:ty)*) => {
        $(
            impl $crate::WriteInto for $ty {
                #[inline]
                fn try_write_into_with_sep<S: ::std::io::Write + ?::std::marker::Sized>(&self, s: &mut S, _sep: impl $crate::Separators) -> ::std::io::Result<()> {
                    ::std::write!(s, "{}", self)
                }
            }
            impl $crate::dimension::Dimension for $ty {
                const DIMENSION: usize = 0;
                const SPACE: bool = true;
            }
        )*
    };
}
