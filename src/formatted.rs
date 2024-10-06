use crate::write::{sep_by, separator::Separator};

/// Create an object that implement [core::fmt::Display] using given separator.
///
/// Pass an iterator and several separators.
///
/// # Examples
///
/// ```rust
/// use iof::sep_by;
/// let v = vec![1, 2, 3];
/// let s = format!("{}", sep_by!(v.iter(), ", "));
/// assert_eq!(s, "1, 2, 3");
/// ```
#[macro_export(local_inner_macros)]
macro_rules! sep_by {
    ($iter:expr, $sep:expr, $($residual:expr),+ $(,)?) => {
        $crate::SepBy::sep_by(::std::iter::IntoIterator::into_iter($iter).map(|iter| $crate::sep_by!(iter, $($residual, )+)), &$sep)
    };
    ($iter:expr, $sep:expr $(,)?) => {
        $crate::SepBy::sep_by($iter, &$sep)
    };
}

/// [std::fmt::Display] with given separator.
///
/// Note that this is a trait, and you can use it with any type that implements [IntoIterator] and whose [IntoIterator::IntoIter] implements [Clone].
///
/// # Examples
///
/// ```rust
/// use iof::SepBy;
/// let v = vec![1, 2, 3];
/// let s = format!("{}", v.sep_by(", "));
/// assert_eq!(s, "1, 2, 3");
/// ```
pub trait SepBy: IntoIterator
where
    <Self as IntoIterator>::IntoIter: Clone,
{
    /// Create an iterator that implement [core::fmt::Display] using given separator.
    fn sep_by<S: Separator + ?Sized>(self, sep: &'_ S) -> sep_by::SepBy<'_, Self::IntoIter, S>;

    /// Create an iterator that implement [WriteInto](crate::WriteInto) using given separator.
    fn sep_by_write_into<S: Separator + ?Sized>(
        self,
        sep: &'_ S,
    ) -> sep_by::SepBy<'_, Self::IntoIter, S>
    where
        Self::Item: crate::WriteInto;
}

impl<I: IntoIterator> SepBy for I
where
    I::IntoIter: Clone,
{
    fn sep_by<S: Separator + ?Sized>(self, sep: &'_ S) -> sep_by::SepBy<'_, Self::IntoIter, S> {
        sep_by::SepBy::new(self.into_iter(), sep)
    }

    fn sep_by_write_into<S: Separator + ?Sized>(
        self,
        sep: &'_ S,
    ) -> sep_by::SepBy<'_, Self::IntoIter, S>
    where
        Self::Item: crate::WriteInto,
    {
        let iter = sep_by::SepBy::new(self.into_iter(), sep);
        fn check_impl_write_into<T: crate::WriteInto>(_: &T) {}
        check_impl_write_into(&iter);
        iter
    }
}
