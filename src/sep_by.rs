use std::fmt;

/// Separate an iterator with given separator.
pub struct SepBy<'a, I> {
    sep: &'a str,
    iter: I,
}

impl<'a, I> SepBy<'a, I> {
    /// Create a [SepBy].
    pub fn new(iter: I, sep: &'a str) -> Self {
        Self { sep, iter }
    }
}

impl<'a, I: Iterator + Clone> fmt::Display for SepBy<'a, I>
where
    I::Item: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.iter.clone();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }
        for item in iter {
            write!(f, "{}{}", self.sep, item)?;
        }
        Ok(())
    }
}
