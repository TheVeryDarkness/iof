use super::ranked::Rank;
use std::io;

/// Separator.
pub trait Separator: std::fmt::Debug {
    /// Write the separator.
    fn write<S: io::Write + ?Sized>(&self, s: &mut S) -> io::Result<()>;
}

impl Separator for str {
    fn write<S: io::Write + ?Sized>(&self, s: &mut S) -> io::Result<()> {
        s.write_all(self.as_bytes())
    }
}

impl<T: Separator + ?Sized> Separator for &T {
    fn write<S: io::Write + ?Sized>(&self, s: &mut S) -> io::Result<()> {
        <T as Separator>::write(*self, s)
    }
}

/// Get default separator.
pub trait GetDefaultSeparator {
    /// Separator type.
    type Separator: Separator + 'static;
    /// Default separator.
    ///
    /// ```rust
    /// use iof::*;
    /// assert_eq!(<Vec::<usize> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &[" "]);
    /// assert_eq!(<Vec::<&str> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &[" "]);
    /// assert_eq!(<Vec::<char> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &[""]);
    /// assert_eq!(<Mat::<usize> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &["\n", " "]);
    /// assert_eq!(<Mat::<&str> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &["\n", " "]);
    /// assert_eq!(<Mat::<char> as GetDefaultSeparator>::DEFAULT_SEPARATOR, &["\n", ""]);
    /// ```
    const DEFAULT_SEPARATOR: &'static [Self::Separator];
}

// impl<T0: Rank0 + ?Sized> GetDefaultSeparator for T0 {
//     type Separator = &'static str;
//     const DEFAULT_SEPARATOR: &'static [&'static str] = &[];
// }

// macro_rules! impl_rank1 {
//     ($ty:ty, $($tt:tt)*) => {
//         impl<T0: Rank0, $($tt)*> GetDefaultSeparator for $ty {
//             type Separator = &'static str;
//             const DEFAULT_SEPARATOR: &'static [&'static str] = &[" "];
//         }
//     };
// }

// impl_rank1!(Vec<T0>,);
// impl_rank1!([T0; N], const N: usize);
// impl_rank1!([T0],);

// macro_rules! impl_rank2 {
//     ($ty:ty, $($tt:tt)*) => {
//         impl<T0: Rank0, $($tt)*> GetDefaultSeparator for $ty {
//             type Separator = &'static str;
//             const DEFAULT_SEPARATOR: &'static [&'static str] = &[" ", "\n"];
//         }
//     };
// }

// impl_rank2!(Vec<Vec<T0>>, );
// impl_rank2!([Vec<T0>], );
// impl_rank2!([Vec<T0>; N], const N: usize);

// impl_rank2!(Vec<[T0; M]>, const M: usize);
// impl_rank2!([[T0; M]], const M: usize);
// impl_rank2!([[T0; M]; N], const M: usize, const N: usize);

// impl_rank2!(Vec<&[T0]>, );
// impl_rank2!([&[T0]], );
// impl_rank2!([&[T0]; N], const N: usize);

const fn get_rank(rank: usize, space: bool) -> &'static [&'static str] {
    match (rank, space) {
        (0, _) => &[],
        (1, true) => &[" "],
        (1, false) => &[""],
        (2, true) => &["\n", " "],
        (2, false) => &["\n", ""],
        // Rank > 2 is not supported.
        // `unimplemented!()` would cause a compile-time error,
        // so we use an empty slice instead.
        (_, true) => &[],
        (_, false) => &[],
    }
}

impl<T: Rank + ?Sized> GetDefaultSeparator for T {
    type Separator = &'static str;
    const DEFAULT_SEPARATOR: &'static [&'static str] = get_rank(T::RANK, T::SPACE);
}
