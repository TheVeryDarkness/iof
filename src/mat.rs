/// A matrix with `m` rows and `n` columns.
///
/// # Examples
///
/// ```rust
/// use iof::Mat;
/// let mat = Mat::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
/// assert_eq!(mat[0], [1, 2, 3]);
/// assert_eq!(mat[1], [4, 5, 6]);
/// ```
pub type Mat<T> = Vec<Vec<T>>;
