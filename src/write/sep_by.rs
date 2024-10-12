use crate::Separators;

use super::{dimension::Dimension, separator::Separator, WriteInto};
use std::{
    fmt::{self, Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex},
    io::Write,
};

/// Separate items with given separator in [fmt::Display].
///
/// `I` is the iterator type, and it should be [Clone] and [Iterator] for this to work.
///
/// [Clone] is required, as this implementation consumes the iterator.
///
/// All configuration on [fmt::Formatter] is delegated to the item type.
pub struct SepBy<'a, I, S: ?Sized> {
    sep: &'a S,
    iter: I,
}

impl<'a, I: Clone, S: ?Sized> Clone for SepBy<'a, I, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            sep: self.sep,
            iter: self.iter.clone(),
        }
    }
}

impl<'a, I: Iterator + Clone, S: Separator + ?Sized> SepBy<'a, I, S> {
    /// Create a [SepBy].
    #[inline]
    pub fn new(iter: I, sep: &'a S) -> Self {
        Self { sep, iter }
    }
}

macro_rules! impl_for_sep_by {
    ($trait:ident) => {
        impl<'a, I: Iterator + Clone, S: Separator + ?Sized> $trait for SepBy<'a, I, S>
        where
            I::Item: $trait,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut iter = self.iter.clone();
                if let Some(first) = iter.next() {
                    $trait::fmt(&first, f)?;
                }
                for item in iter {
                    self.sep.write_fmt(f)?;
                    $trait::fmt(&item, f)?;
                }
                Ok(())
            }
        }
    };
}

impl_for_sep_by!(Display);
impl_for_sep_by!(Debug);

impl_for_sep_by!(Octal);
impl_for_sep_by!(Binary);
impl_for_sep_by!(LowerHex);
impl_for_sep_by!(UpperHex);
impl_for_sep_by!(Pointer);
impl_for_sep_by!(LowerExp);
impl_for_sep_by!(UpperExp);

impl<I: Iterator<Item = T> + Clone, T: WriteInto, S: Separator + ?Sized> Dimension
    for SepBy<'_, I, S>
{
    const DIMENSION: usize = 0;
    const SPACE: bool = T::SPACE;
}

impl<I: Iterator<Item = T> + Clone, T: WriteInto, S: Separator + ?Sized> WriteInto
    for SepBy<'_, I, S>
{
    #[inline]
    fn try_write_into_with_sep<Stream: Write + ?Sized>(
        &self,
        s: &mut Stream,
        residual: impl Separators,
    ) -> super::Result {
        let mut iter = self.iter.clone();
        if let Some(first) = iter.next() {
            first.try_write_into_with_sep(s, residual)?;
        }
        for item in iter {
            self.sep.write_io(s)?;
            item.try_write_into_with_sep(s, residual)?;
        }
        Ok(())
    }
}
