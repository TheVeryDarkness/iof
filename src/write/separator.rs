//! Separator and default separator.
use super::dimension::Dimension;
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

/// Get default separator.
pub trait GetDefaultSeparator {
    /// Separator type.
    type Separator: Separator + 'static;
    /// Default separator.
    ///
    /// ```rust
    /// use iof::{separator::GetDefaultSeparator, Mat, Vec};
    /// assert_eq!(<Vec::<usize> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &[" "]);
    /// assert_eq!(<Vec::<&str> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &[" "]);
    /// assert_eq!(<Vec::<char> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &[""]);
    /// assert_eq!(<Mat::<usize> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &["\n", " "]);
    /// assert_eq!(<Mat::<&str> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &["\n", " "]);
    /// assert_eq!(<Mat::<char> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &["\n", ""]);
    /// ```
    const DEFAULT_SEPARATOR: &'static [Self::Separator];
}

const fn get_separator(dimension: usize, space: bool) -> &'static [&'static str] {
    match (dimension, space) {
        (0, _) => &[],
        (1, true) => &[" "],
        (1, false) => &[""],
        (2, true) => &["\n", " "],
        (2, false) => &["\n", ""],
        // Dimension > 2 is not supported.
        // `unimplemented!()` would cause a compile-time error even if the function is not called,
        // so we use an empty slice instead.
        (_, true) => &[],
        (_, false) => &[],
    }
}

impl<T: Dimension + ?Sized> GetDefaultSeparator for T {
    type Separator = &'static str;
    const DEFAULT_SEPARATOR: &'static [&'static str] = get_separator(T::DIMENSION, T::SPACE);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mat;

    fn check<T: Dimension + ?Sized>(separator: &[&str]) {
        assert_eq!(<T as GetDefaultSeparator>::DEFAULT_SEPARATOR, separator);
        assert_eq!(get_separator(T::DIMENSION, T::SPACE), separator);
    }

    #[test]
    fn check_separator() {
        check::<char>(&[""; 0]);
        check::<Vec<char>>(&[""]);
        check::<Mat<char>>(&["\n", ""]);
        check::<usize>(&[""; 0]);
        check::<Vec<usize>>(&[" "]);
        check::<Mat<usize>>(&["\n", " "]);
    }

    #[test]
    fn check_separator_unimplemented() {
        check::<Vec<Vec<Vec<usize>>>>(&[]);
        check::<Vec<Vec<Vec<char>>>>(&[]);
    }
}
