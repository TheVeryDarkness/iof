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
#[macro_export(local_inner_macros)]
macro_rules! show {
    ($($expr:expr),* $(,)? ; $($opt:ident=$val:expr),* $(,)?) => {
        unwrap!(|| -> ::std::io::Result<()> {
            $crate::Writer::new()
                $(.$opt($val))*
                $(.write(&$expr)?)*
                .finish()?;
            Ok(())
        }())
    };
    ($($expr:expr),* $(,)?) => {
        unwrap!(|| -> ::std::io::Result<()> {
            $crate::Writer::new()
                $(.write(&$expr)?)*
                .finish()?;
            Ok(())
        }())
    };
}

/// Implement [WriteInto] for given types that already implements [std::fmt::Display].
///
/// [WriteInto]: crate::WriteInto
#[macro_export(local_inner_macros)]
macro_rules! impl_write_into {
    ($($ty:ty)*) => {
        $(
            impl $crate::WriteOneInto for $ty {
                fn try_write_one_into<S: ::std::io::Write + ?::std::marker::Sized>(&self, s: &mut S) -> ::std::io::Result<()> {
                    ::std::write!(s, "{}", self)
                }
            }
        )*
    };
}
