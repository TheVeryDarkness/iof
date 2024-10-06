//! Rank markers.

/// A trait for types with a dimension.
pub trait Rank {
    /// Dimension.
    ///
    /// Currently this is only a marker,
    /// but it can be used after feature `generic_const_exprs` is stable.
    /// See <https://github.com/rust-lang/rust/issues/60551>
    const RANK: usize;
    /// Need space between every two items?
    const SPACE: bool;
}

impl<T: Rank + ?Sized> Rank for &T {
    const RANK: usize = T::RANK;
    const SPACE: bool = T::SPACE;
}

// Implementation for higher-rank types.
impl<T: Rank> Rank for Vec<T> {
    const RANK: usize = T::RANK + 1;
    const SPACE: bool = T::SPACE;
}
impl<T: Rank> Rank for [T] {
    const RANK: usize = T::RANK + 1;
    const SPACE: bool = T::SPACE;
}
impl<T: Rank, const N: usize> Rank for [T; N] {
    const RANK: usize = T::RANK + 1;
    const SPACE: bool = T::SPACE;
}
