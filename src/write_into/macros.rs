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
#[macro_export]
macro_rules! show {
    (; end=$end:expr, sep=$sep:expr) => {
        $crate::WriteInto::write(&$end);
    };
    ($expr:expr $(, $res:expr)*; end=$end:expr, sep=$sep:expr) => {
        $crate::WriteInto::write(&$expr);
        $(
            $crate::WriteInto::write(&$sep);
            $crate::WriteInto::write(&$res);
        )*
        $crate::WriteInto::write(&$end);
    };
    ($expr:expr $(, $res:expr)*; sep=$sep:expr, end=$end:expr) => {
        $crate::show!($expr $(, $res)*; end=$end, sep=$sep);
    };
    ($expr:expr $(, $res:expr)*; end=$end:expr) => {
        $crate::show!($expr $(, $res)*; end=$end, sep=" ");
    };
    ($expr:expr $(, $res:expr)*; sep=$sep:expr) => {
        $crate::show!($expr $(, $res)*; end="\n", sep=$sep);
    };
    ($expr:expr $(, $res:expr)* $(;)?) => {
        $crate::show!($expr $(, $res)*; end="\n", sep="");
    };

    (; end=$end:expr) => {
        $crate::WriteInto::write(&$end);
    };
    (; sep=$sep:expr, end=$end:expr) => {
        $crate::show!(; end=$end, sep=$sep);
    };
    (; end=$end:expr) => {
        $crate::show!(; end=$end, sep=" ");
    };
    (; sep=$sep:expr) => {
        $crate::show!(; end="\n", sep=$sep);
    };
    ($(;)?) => {
        $crate::show!(; end="\n", sep="");
    };
}
/// Implement [WriteInto] for given types that already implements [std::fmt::Display].
///
/// [WriteInto]: crate::WriteInto
#[macro_export]
macro_rules! impl_write_into {
    ($($ty:ty)*) => {
        $(
            impl $crate::WriteSingleInto for $ty {
                fn try_write_single_into<S: ::std::io::Write>(&self, s: &mut S) -> ::std::io::Result<()> {
                    ::std::write!(s, "{}", self)
                }
            }
        )*
    };
}
