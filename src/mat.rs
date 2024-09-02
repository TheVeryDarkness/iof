/// A matrix with `m` rows and `n` columns.
///
/// # Examples
///
/// Create a matrix with two rows and three columns:
///
/// ```rust
/// use iof::Mat;
/// let mat = Mat::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
/// assert_eq!(mat[0], [1, 2, 3]);
/// assert_eq!(mat[1], [4, 5, 6]);
/// ```
///
/// Read a matrix with two rows and three columns:
///
/// ```rust,no_run
/// use iof::{read, Mat};
/// let _: Mat<i32> = read!(2, 3);
/// ```
pub type Mat<T> = Vec<Vec<T>>;
