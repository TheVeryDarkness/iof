#![cfg(test)]

use super::Mat;

#[test]
fn empty() {
    let mat: Mat<u32> = Mat::new();
    assert_eq!(mat.first_row(), None);
    assert_eq!(mat.last_row(), None);
    assert_eq!(mat.len_rows(), 0);
    assert_eq!(mat.len_columns(), 0);
    assert_eq!(mat.iter().count(), 0);
}

#[test]
fn zeros() {
    let mat: Mat<i32> = Mat::with_default(3, 2);
    assert_eq!(mat.first_row(), Some([0, 0].as_slice()));
    assert_eq!(mat.last_row(), Some([0, 0].as_slice()));
    assert_eq!(mat.len_rows(), 3);
    assert_eq!(mat.len_columns(), 2);
    assert_eq!(mat.iter().count(), 3);
    assert_eq!(mat.iter().size_hint(), (3, Some(3)));
    assert_eq!(mat.iter().last(), Some([0, 0].as_slice()));
    assert_eq!(format!("{:?}", mat), "[[0, 0], [0, 0], [0, 0]]");
    for row in &mat {
        assert_eq!(row, [0, 0].as_slice());
    }
}

#[test]
fn ones() {
    let mat: Mat<i32> = Mat::with_clone(3, 2, 1);
    assert_eq!(mat.first_row(), Some([1, 1].as_slice()));
    assert_eq!(mat.last_row(), Some([1, 1].as_slice()));
    assert_eq!(mat.len_rows(), 3);
    assert_eq!(mat.len_columns(), 2);
    assert_eq!(mat.iter().count(), 3);
    assert_eq!(mat.iter().size_hint(), (3, Some(3)));
    assert_eq!(mat.iter().last(), Some([1, 1].as_slice()));
    assert_eq!(format!("{:?}", mat), "[[1, 1], [1, 1], [1, 1]]");
    for row in &mat {
        assert_eq!(row, [1, 1].as_slice());
    }
}

#[test]
fn repeating_rows() {
    let mat: Mat<i32> = Mat::with_repeat(3, vec![1, 2]);
    assert_eq!(mat.first_row(), Some([1, 2].as_slice()));
    assert_eq!(mat.last_row(), Some([1, 2].as_slice()));
    assert_eq!(mat.len_rows(), 3);
    assert_eq!(mat.len_columns(), 2);
    assert_eq!(mat.iter().count(), 3);
    assert_eq!(mat.iter().size_hint(), (3, Some(3)));
    assert_eq!(mat.iter().last(), Some([1, 2].as_slice()));
    assert_eq!(format!("{:?}", mat), "[[1, 2], [1, 2], [1, 2]]");
    for row in &mat {
        assert_eq!(row, [1, 2].as_slice());
    }
}
