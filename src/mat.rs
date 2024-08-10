use row_iter::RowIter;
use std::{fmt::Debug, iter::repeat_with, ops::Index};

mod row_iter;
mod tests;

/// A matrix.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Mat<T> {
    inner: Vec<T>,
    m: usize,
    n: usize,
}

impl<T> Mat<T> {
    /// Create a [Mat] from [Vec].
    pub fn from_vec(m: usize, n: usize, vec: Vec<T>) -> Self {
        assert_eq!(vec.len(), m * n);
        Self { m, n, inner: vec }
    }
}

impl<T> Mat<T> {
    /// Create an empty [Mat].
    pub const fn new() -> Self {
        let m = 0;
        let n = 0;
        let inner = Vec::new();
        Self { m, n, inner }
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
        Self { m, n, inner }
    }
}

impl<T> Mat<T> {
    /// Create a [Mat], each of which is constructed using [Default::default].
    pub fn with(m: usize, n: usize, f: impl FnMut() -> T) -> Self {
        let inner = Vec::from_iter(repeat_with(f).take(m * n));
        assert_eq!(inner.len(), m * n);
        Self { m, n, inner }
    }
}

impl<T: Default> Mat<T> {
    /// Create a [Mat], each element of which is constructed using [Default::default].
    pub fn with_default(m: usize, n: usize) -> Self {
        Self::with(m, n, Default::default)
    }
    /// Create a [Mat] that is diagonal matrix.
    ///
    /// The diagonal elements are constructed using `f`, and the other elements are constructed using [Default::default].
    pub fn diagonal_from_fn(n: usize, mut f: impl FnMut(usize) -> T) -> Self {
        let mut inner = Vec::with_capacity(n * n);
        for i in 0..n {
            for j in 0..n {
                inner.push(if i == j { f(j) } else { T::default() });
            }
        }
        Self { m: n, n, inner }
    }
}

impl<T: Default + Clone> Mat<T> {
    /// Create a [Mat] that is scalar matrix.
    ///
    /// The diagonal elements are clones of `elem`, and the other elements are constructed using [Default::default].
    pub fn scalar(n: usize, elem: T) -> Self {
        let mut inner = Vec::with_capacity(n * n);
        for i in 0..n {
            for j in 0..n {
                inner.push(if i == j { elem.clone() } else { T::default() });
            }
        }
        Self { m: n, n, inner }
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
        debug_assert_eq!(self.inner.len(), self.n * self.m);
        self.m
    }
    /// Columns count.
    pub fn len_columns(&self) -> usize {
        self.n
    }
    /// First row.
    pub fn first_row(&self) -> Option<&[T]> {
        if self.len_rows() == 0 {
            None
        } else {
            let n = self.len_columns();
            self.inner.get(0..n)
        }
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
    /// Convert `self` into a [RowIter].
    pub fn iter(&self) -> RowIter<'_, T> {
        RowIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a Mat<T> {
    type Item = &'a [T];

    type IntoIter = RowIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Debug> Debug for Mat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Clone> Mat<T> {
    /// Transpose.
    pub fn transpose(&self) -> Self {
        let mut inner = Vec::with_capacity(self.inner.capacity());
        let n = self.len_columns();
        let m = self.len_rows();
        for j in 0..n {
            for i in 0..m {
                inner.push(self.inner[i * n + j].clone());
            }
        }
        let n = self.len_rows();
        let m = self.len_columns();
        Self { inner, n, m }
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for Mat<T> {
    fn from(array: [[T; N]; M]) -> Self {
        let n = N;
        let m = M;
        let inner = array.into_iter().flat_map(|row| row.into_iter()).collect();
        Self { inner, n, m }
    }
}
impl<T: PartialEq> PartialEq<Vec<Vec<T>>> for Mat<T> {
    fn eq(&self, other: &Vec<Vec<T>>) -> bool {
        if self.len_rows() != other.len() {
            return false;
        }
        for (row, other_row) in self.iter().zip(other.iter()) {
            if row != other_row.as_slice() {
                return false;
            }
        }
        true
    }
}
impl<T: PartialEq, const M: usize, const N: usize> PartialEq<[[T; N]; M]> for Mat<T> {
    fn eq(&self, other: &[[T; N]; M]) -> bool {
        if self.len_rows() != M || self.len_columns() != N {
            return false;
        }
        for i in 0..M {
            for j in 0..N {
                if self[i][j] != other[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl<T, const N: usize> FromIterator<[T; N]> for Mat<T> {
    fn from_iter<I: IntoIterator<Item = [T; N]>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut iter = iter.peekable();
        if let Some(first) = iter.peek() {
            let n = first.len();
            let mut inner = Vec::new();
            let mut m = 0;
            for row in iter {
                assert_eq!(row.len(), n);
                inner.extend(row);
                m += 1;
            }
            Self::from_vec(m, n, inner)
        } else {
            Self::new()
        }
    }
}

impl<'a, T: Clone + 'a> FromIterator<&'a [T]> for Mat<T> {
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

impl<'a, T: Clone + 'a, const N: usize> FromIterator<&'a [T; N]> for Mat<T> {
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
