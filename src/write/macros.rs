/// Write the given expression into [standard output](std::io::Stdout) using [WriteInto].
///
/// You can configure the writer using the following options:
///
/// - `sep`: Separator between values. Default is `" "`.
/// - `end`: End of the output. Default is `"\n"`.
/// - `buf`: Buffer to write into. Default is [standard output](crate::stdout).
///
/// ```rust
#[doc = include_str!("../../examples/doc_macro_show.rs")]
/// ```
///
/// [WriteInto]: crate::WriteInto
#[macro_export]
macro_rules! show {
    ($expr:expr $(, $opt:ident=$val:expr)* $(,)? => $buf:expr) => {
        unwrap!(|| -> ::std::io::Result<()> {
            $crate::Writer::new()
                $(.$opt(&$val))*
                .write(&$expr, &mut $buf)?;
            Ok(())
        }())
    };
    ($expr:expr $(, $opt:ident=$val:expr)* $(,)?) => {
        $crate::unwrap!(|| -> ::std::io::Result<()> {
            $crate::Writer::new()
                $(.$opt(&$val))*
                .write(&$expr, &mut $crate::stdout())?;
            Ok(())
        }())
    };
}

/// Implement [WriteInto] for given types that already implements [std::fmt::Display].
///
/// [WriteInto]: crate::WriteInto
#[macro_export]
macro_rules! impl_for_single {
    ($($ty:ty)*) => {
        $(
            impl $crate::WriteInto for $ty {
                fn try_write_into_with_sep<S: ::std::io::Write + ?::std::marker::Sized>(&self, s: &mut S, _sep: &[impl Separator]) -> ::std::io::Result<()> {
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
