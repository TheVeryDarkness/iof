use std::{fmt, fmt::Write};

type Result = fmt::Result;

/// Format trait with default formatting.
///
/// This is similar to [fmt::Display], but we need this to avoid conflicting.
#[deprecated]
pub trait Display: fmt::Display {
    /// Write into a [Write].
    fn fmt(&self, f: &mut impl Write) -> Result;
}

/// Implement [Display] for given types that already implements [fmt::Display].
#[macro_export]
macro_rules! impl_display {
    ($($ty:ty)*) => {
        $(
            impl $crate::Display for $ty {
                fn fmt(&self, f: &mut impl ::std::fmt::Write) -> ::std::fmt::Result {
                    ::std::write!(f, "{}", self)
                }
            }
        )*
    };
}

impl_display!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
    bool
    char str String
);

impl<T: Display + ?Sized> Display for &T {
    fn fmt(&self, f: &mut impl Write) -> Result {
        Display::fmt(*self, f)
    }
}
