//! Dimension markers.

/// A trait for types with a dimension.
pub trait Dimension {
    /// Dimension.
    ///
    /// Currently this is only a marker,
    /// but it can be used after feature `generic_const_exprs` is stable.
    /// See <https://github.com/rust-lang/rust/issues/60551>
    const DIMENSION: usize;
    /// Need space between every two items?
    const SPACE: bool;
}

impl<T: Dimension + ?Sized> Dimension for &T {
    const DIMENSION: usize = T::DIMENSION;
    const SPACE: bool = T::SPACE;
}
impl<T: Dimension + ?Sized> Dimension for Box<T> {
    const DIMENSION: usize = T::DIMENSION;
    const SPACE: bool = T::SPACE;
}
impl<T: Dimension + ?Sized> Dimension for std::rc::Rc<T> {
    const DIMENSION: usize = T::DIMENSION;
    const SPACE: bool = T::SPACE;
}
impl<T: Dimension + ?Sized> Dimension for std::sync::Arc<T> {
    const DIMENSION: usize = T::DIMENSION;
    const SPACE: bool = T::SPACE;
}

// Implementation for higher-dimension types.
impl<T: Dimension> Dimension for Vec<T> {
    const DIMENSION: usize = T::DIMENSION + 1;
    const SPACE: bool = T::SPACE;
}
impl<T: Dimension> Dimension for [T] {
    const DIMENSION: usize = T::DIMENSION + 1;
    const SPACE: bool = T::SPACE;
}
impl<T: Dimension, const N: usize> Dimension for [T; N] {
    const DIMENSION: usize = T::DIMENSION + 1;
    const SPACE: bool = T::SPACE;
}
