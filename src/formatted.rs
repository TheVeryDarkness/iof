use crate::write::{sep_by, separator::Separator};

/// Create an object using given separator.
///
/// Pass an iterator and several separators.
///
/// # Examples
///
/// ```rust
/// use iof::sep_by;
/// let v = vec![1, 2, 3];
/// let s = format!("{}", sep_by!(&v, ", "));
/// assert_eq!(s, "1, 2, 3");
///
/// let v = [[1, 2], [3, 4]];
/// let s = format!("{}", sep_by!(&v, "\n", ", "));
/// assert_eq!(s, "1, 2\n3, 4");
///
/// let v: [[f64; 2]; 2] = [[1.0 / 3.0, 2.0 / 3.0], [3.0 / 7.0, 4.0 / 7.0]];
/// let s = format!("{:.5}", sep_by!(&v, "\n", ", "));
/// assert_eq!(s, "0.33333, 0.66667\n0.42857, 0.57143");
/// ```
#[macro_export]
macro_rules! sep_by {
    ($iter:expr, $sep:expr, $($residual:expr),+ $(,)?) => {
        $crate::SepBy::sep_by(::std::iter::IntoIterator::into_iter($iter).map(|iter| $crate::sep_by!(iter, $($residual, )+)), &$sep)
    };
    ($iter:expr, $sep:expr $(,)?) => {
        $crate::SepBy::sep_by($iter, &$sep)
    };
}

/// Create a new object using given iterator and separator.
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
    /// Create an object that implement [core::fmt::Display] using given iterator and separator.
    fn sep_by<S: Separator + ?Sized>(self, sep: &'_ S) -> sep_by::SepBy<'_, Self::IntoIter, S>;

    /// Create an object that implement [WriteInto](crate::WriteInto) using given iterator and separator.
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
    #[inline]
    fn sep_by<S: Separator + ?Sized>(self, sep: &'_ S) -> sep_by::SepBy<'_, Self::IntoIter, S> {
        sep_by::SepBy::new(self.into_iter(), sep)
    }

    #[inline]
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
