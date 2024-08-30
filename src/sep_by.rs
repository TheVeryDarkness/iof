use std::fmt::{self, Binary, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};

/// Separate items with given separator in [fmt::Display].
///
/// `I` is the iterator type, and it should be [Clone] and [Iterator] for this to work.
///
/// [Clone] is required, as this implementation consumes the iterator.
///
/// All configuration on [fmt::Formatter] is delegated to the item type.
#[derive(Debug, Clone)]
pub struct SepBy<'a, I> {
    sep: &'a str,
    iter: I,
}

impl<'a, I: Iterator + Clone> SepBy<'a, I> {
    /// Create a [SepBy].
    pub fn new(iter: I, sep: &'a str) -> Self {
        Self { sep, iter }
    }
}

macro_rules! impl_for_sep_by {
    ($trait:ident) => {
        impl<'a, I: Iterator + Clone> $trait for SepBy<'a, I>
        where
            I::Item: $trait,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut iter = self.iter.clone();
                if let Some(first) = iter.next() {
                    $trait::fmt(&first, f)?;
                }
                for item in iter {
                    f.write_str(self.sep)?;
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
