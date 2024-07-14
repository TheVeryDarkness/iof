use std::{iter::repeat_with, ops::Index};

mod row_iter;

#[derive(Default)]
pub struct Mat<T> {
    inner: Vec<T>,
    n: usize,
}

impl<T> Mat<T> {
    /// Create a [Mat] from [Vec].
    pub fn from_vec(m: usize, n: usize, vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), m * n);
        Self { n, inner: vec }
    }
}

impl<T> Mat<T> {
    /// Create an empty [Mat].
    pub const fn new() -> Self {
        let n = 0;
        let inner = Vec::new();
        Self { n, inner }
    }
}

impl<T: Clone> Mat<T> {
    /// Create a [Mat], each of which is a clone of `value`.
    pub fn with_clone(m: usize, n: usize, value: T) -> Self {
        Self::with(m, n, || value.clone())
    }
}
impl<T: Copy> Mat<T> {
    /// Create a [Mat], each row of which is a copy of `row`.
    pub fn with_repeat(m: usize, row: Vec<T>) -> Self {
        let n = row.len();
        let inner = row.repeat(m);
        assert_eq!(inner.len(), m * n);
        Self { n, inner }
    }
}

impl<T> Mat<T> {
    /// Create a [Mat], each of which is constructed using [Default::default].
    pub fn with(m: usize, n: usize, f: impl FnMut() -> T) -> Self {
        let inner = Vec::from_iter(repeat_with(f).take(m * n));
        assert_eq!(inner.len(), m * n);
        Self { n, inner }
    }
}

impl<T: Default> Mat<T> {
    /// Create a [Mat], each of which is constructed using [Default::default].
    pub fn with_default(m: usize, n: usize) -> Self {
        Self::with(m, n, Default::default)
    }
}

impl<T> Index<usize> for Mat<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[(index * self.n)..((index + 1) * self.n)]
    }
}

impl<T> Mat<T> {
    /// Rows count.
    pub fn len_rows(&self) -> usize {
        self.inner.len() / self.n
    }
    /// Columns count.
    pub fn len_columns(&self) -> usize {
        self.n
    }
    /// Last row.
    pub fn last_row(&self) -> Option<&[T]> {
        let n = self.len_columns();
        let m = self.len_rows();
        if let Some(m_1) = m.checked_sub(1) {
            Some(&self.inner[m_1 * n..m * n])
        } else {
            None
        }
    }
}

impl<'a, T: Clone> FromIterator<&'a [T]> for Mat<T> {
    fn from_iter<I: IntoIterator<Item = &'a [T]>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut iter = iter.peekable();
        if let Some(first) = iter.peek() {
            let n = first.len();
            let mut inner = Vec::new();
            let mut m = 0;
            for row in iter {
                assert_eq!(row.len(), n);
                inner.extend_from_slice(row);
                m += 1;
            }
            Self::from_vec(m, n, inner)
        } else {
            Self::new()
        }
    }
}

impl<'a, T: Clone, const N: usize> FromIterator<&'a [T; N]> for Mat<T> {
    fn from_iter<I: IntoIterator<Item = &'a [T; N]>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let n = N;
        let mut inner = Vec::new();
        let mut m = 0;
        for row in iter {
            // assert_eq!(row.len(), n);
            inner.extend_from_slice(row);
            m += 1;
        }
        Self::from_vec(m, n, inner)
    }
}
