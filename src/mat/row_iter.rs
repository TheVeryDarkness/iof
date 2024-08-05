use super::Mat;

/// Iterator over all rows.
pub struct RowIter<'a, T> {
    mat: &'a Mat<T>,
    i: usize,
}

impl<'a, T> RowIter<'a, T> {
    /// Create a [RowIter] from a [Mat].
    pub const fn new(mat: &'a Mat<T>) -> Self {
        let i = 0;
        Self { mat, i }
    }
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.mat.n;
        let i = self.i;
        let inner = &self.mat.inner;
        let m = self.mat.len_rows();
        if i < m {
            let row = &inner[i * n..(i + 1) * n];
            self.i += 1;
            Some(row)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remained = self.mat.len_rows() - self.i;
        (remained, Some(remained))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.mat.len_rows()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.mat.last_row()
    }
}
