use crate::sep_by;
use std::{fmt, fmt::Write};

type Result = fmt::Result;

/// Format trait with default formatting.
///
/// This is similar to [fmt::Display], but we need this to avoid conflicting.
pub trait Display {
    /// Write into a [Write].
    fn fmt(&self, f: &mut impl Write) -> Result;
}

macro_rules! impl_display {
    ($($ty:ty)*) => {
        $(
            impl Display for $ty {
                fn fmt(&self, f: &mut impl Write) -> Result {
                    write!(f, "{}", self)
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
        (*self).fmt(f)
    }
}

impl<'a, I: Clone + Iterator> Display for sep_by::SepBy<'a, I>
where
    I::Item: fmt::Display,
{
    fn fmt(&self, f: &mut impl Write) -> Result {
        write!(f, "{}", self)
    }
}
