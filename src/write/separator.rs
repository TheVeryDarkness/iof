//! Separator and default separator.
use std::fmt::Arguments;

/// Position.
///
/// The index of the last item that has been written.
pub type Position = usize;

/// Separator.
pub trait Separator {
    /// Format the separator.
    fn format<R>(&self, _: Position, f: impl FnOnce(Arguments<'_>) -> R) -> R;
}

impl Separator for char {
    #[inline]
    fn format<R>(&self, _: Position, f: impl FnOnce(Arguments<'_>) -> R) -> R {
        f(format_args!("{}", self))
    }
}

impl Separator for str {
    #[inline]
    fn format<R>(&self, _: Position, f: impl FnOnce(Arguments<'_>) -> R) -> R {
        f(format_args!("{}", self))
    }
}

impl Separator for String {
    #[inline]
    fn format<R>(&self, _: Position, f: impl FnOnce(Arguments<'_>) -> R) -> R {
        f(format_args!("{}", self))
    }
}

impl<T: Separator + ?Sized> Separator for &T {
    #[inline]
    fn format<R>(&self, pos: Position, f: impl FnOnce(Arguments<'_>) -> R) -> R {
        T::format(*self, pos, f)
    }
}
