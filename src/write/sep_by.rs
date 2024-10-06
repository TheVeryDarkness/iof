use super::{ranked::Rank, separator::Separator, WriteInto};
use std::{
    fmt::{self, Binary, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex},
    io::Write,
};

/// Separate items with given separator in [fmt::Display].
///
/// `I` is the iterator type, and it should be [Clone] and [Iterator] for this to work.
///
/// [Clone] is required, as this implementation consumes the iterator.
///
/// All configuration on [fmt::Formatter] is delegated to the item type.
#[derive(Debug, Clone)]
pub struct SepBy<'a, I, S: ?Sized> {
    sep: &'a S,
    iter: I,
}

impl<'a, I: Iterator + Clone, S: Separator + ?Sized> SepBy<'a, I, S> {
    /// Create a [SepBy].
    pub fn new(iter: I, sep: &'a S) -> Self {
        Self { sep, iter }
    }
}

macro_rules! impl_for_sep_by {
    ($trait:ident) => {
        impl<'a, I: Iterator + Clone, S: Separator + ?Sized> $trait for SepBy<'a, I, S>
        where
            I::Item: $trait,
            S: Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut iter = self.iter.clone();
                if let Some(first) = iter.next() {
                    $trait::fmt(&first, f)?;
                }
                for item in iter {
                    Display::fmt(&self.sep, f)?;
                    $trait::fmt(&item, f)?;
                }
                Ok(())
            }
        }
    };
}

impl_for_sep_by!(Display);

impl_for_sep_by!(Octal);
impl_for_sep_by!(Binary);
impl_for_sep_by!(LowerHex);
impl_for_sep_by!(UpperHex);
impl_for_sep_by!(Pointer);
impl_for_sep_by!(LowerExp);
impl_for_sep_by!(UpperExp);

impl<I: Iterator<Item = T> + Clone, T: WriteInto, S: Separator + ?Sized> Rank for SepBy<'_, I, S> {
    const RANK: usize = 0;
    const SPACE: bool = T::SPACE;
}

impl<I: Iterator<Item = T> + Clone, T: WriteInto, S: Separator + ?Sized> WriteInto
    for SepBy<'_, I, S>
{
    fn try_write_into_with_sep<Stream: Write + ?Sized>(
        &self,
        s: &mut Stream,
        residual: &[impl Separator],
    ) -> super::Result {
        let mut iter = self.iter.clone();
        if let Some(first) = iter.next() {
            first.try_write_into_with_sep(s, residual)?;
        }
        for item in iter {
            self.sep.write(s)?;
            item.try_write_into_with_sep(s, residual)?
        }
        Ok(())
    }
}
