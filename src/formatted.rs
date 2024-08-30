use crate::sep_by;

/// [std::fmt::Display] with given separator.
///
/// # Examples
///
/// ```rust
/// use iof::SepBy;
/// let v = vec![1, 2, 3];
/// let s = format!("{}", v.sep_by(", "));
/// assert_eq!(s, "1, 2, 3");
/// ```
pub trait SepBy: IntoIterator {
    /// Create an iterator that implement [core::fmt::Display] using given separator.
    fn sep_by(self, sep: &'_ str) -> sep_by::SepBy<'_, Self::IntoIter>;
}

impl<I: IntoIterator> SepBy for I
where
    I::IntoIter: Clone,
{
    fn sep_by(self, sep: &'_ str) -> sep_by::SepBy<'_, Self::IntoIter> {
        super::sep_by::SepBy::new(self.into_iter(), sep)
    }
}
