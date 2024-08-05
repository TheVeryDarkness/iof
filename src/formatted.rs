use crate::sep_by;

/// [std::fmt::Display] with given separator.
pub trait SepBy: IntoIterator {
    /// Create a [sep_by::SepBy] with given separator.
    fn sep_by<'a>(self, sep: &'a str) -> sep_by::SepBy<'a, Self::IntoIter>;
}

impl<I: IntoIterator> SepBy for I {
    fn sep_by<'a>(self, sep: &'a str) -> sep_by::SepBy<'a, Self::IntoIter> {
        super::sep_by::SepBy::new(self.into_iter(), sep)
    }
}
