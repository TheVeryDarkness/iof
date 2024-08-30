use super::Mat;
use std::{fmt::Debug, iter::FusedIterator};

/// Iterator over all rows.
#[derive(Copy)]
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

impl<'a, T> Clone for RowIter<'a, T> {
    fn clone(&self) -> Self {
        Self {
            mat: self.mat,
            i: self.i,
        }
    }
}

impl<'a, T: Debug> Debug for RowIter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, T> ExactSizeIterator for RowIter<'a, T> {
    fn len(&self) -> usize {
        self.mat.len_rows() - self.i
    }
}

impl<'a, T> FusedIterator for RowIter<'a, T> {}
