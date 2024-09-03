/// Write the given expression into [standard output](std::io::Stdout) using [WriteInto].
///
/// ```rust
/// use iof::show;
///
/// show!(42);
/// show!(42, "Hello, World!");
/// show!(42, "Hello, World!", [1, 2, 3, 4]);
/// show!(42, "Hello, World!", [1, 2, 3, 4], [[1, 2], [3, 4]]);
/// show!(42, "Hello, World!", [1, 2, 3, 4], [[1, 2], [3, 4]]; sep=", ");
/// show!(42, "Hello, World!", [1, 2, 3, 4], [[1, 2], [3, 4]]; sep=", ", end="!");
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
