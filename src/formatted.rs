use crate::sep_by;

/// [std::fmt::Display] with given separator.
pub trait SepBy: IntoIterator {
    /// Create a [sep_by::SepBy] with given separator.
    fn sep_by(self, sep: &'_ str) -> sep_by::SepBy<'_, Self::IntoIter>;
}

impl<I: IntoIterator> SepBy for I {
    fn sep_by(self, sep: &'_ str) -> sep_by::SepBy<'_, Self::IntoIter> {
        super::sep_by::SepBy::new(self.into_iter(), sep)
    }
}
