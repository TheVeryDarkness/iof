/// Write the given expression into [standard output](std::io::Stdout) using [WriteInto].
///
/// [WriteInto]: crate::WriteInto
#[macro_export]
macro_rules! show {
    ($expr:expr) => {
        $crate::WriteInto::write(&$expr)
    };
    ($expr:expr, end=$end:expr) => {
        $crate::WriteInto::write(&$expr);
        $crate::WriteInto::write(&$end);
    };
}
/// Implement [WriteInto] for given types that already implements [std::fmt::Display].
///
/// [WriteInto]: crate::WriteInto
#[macro_export]
macro_rules! impl_write_into {
    ($($ty:ty)*) => {
        $(
            impl $crate::WriteInto for $ty {
                fn try_write_into<S: ::std::io::Write>(&self, s: &mut S) -> ::std::io::Result<()> {
                    ::std::write!(s, "{}", self)
                }
            }

            impl $crate::WriteInto for ::std::vec::Vec<$ty> {
                fn try_write_into<S: ::std::io::Write>(&self, s: &mut S) -> ::std::io::Result<()> {
                    self.as_slice().try_write_into(s)
                }
            }

            impl<const N: usize> $crate::WriteInto for [$ty; N] {
                fn try_write_into<S: ::std::io::Write>(&self, s: &mut S) -> ::std::io::Result<()> {
                    self.as_slice().try_write_into(s)
                }
            }

            impl $crate::WriteInto for [$ty] {
                fn try_write_into<S: ::std::io::Write>(&self, s: &mut S) -> ::std::io::Result<()> {
                    use $crate::SepBy;
                    WriteInto::try_write_into(&self.sep_by(" "), s)
                }
            }

            impl $crate::WriteInto for $crate::Mat<$ty> {
                fn try_write_into<S: ::std::io::Write>(&self, s: &mut S) -> ::std::io::Result<()> {
                    use $crate::SepBy;
                    self.iter()
                        .map(|row| row.iter().sep_by(" "))
                        .sep_by("\n")
                        .try_write_into(s)
                }
            }

        )*
    };
}
