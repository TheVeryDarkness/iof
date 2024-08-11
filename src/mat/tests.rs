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
    assert_eq!(mat.iter().size_hint(), (0, Some(0)));
    assert_eq!(mat.iter().last(), None);
    assert_eq!(format!("{:?}", mat), "[]");
    assert_eq!(mat, Mat::from_iter([] as [[u32; 0]; 0]));
    assert_eq!(mat, Mat::from_iter([] as [[u32; 1]; 0]));
    assert_eq!(mat, Mat::from_iter(&[] as &[[u32; 0]; 0]));
    // assert_eq!(mat, Mat::from_iter(&[] as &[[u32; 1]; 0]));
    assert_eq!(mat, Mat::from_iter([] as [&[u32]; 0]));
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
    assert_eq!(mat, Mat::from_iter([[1, 2], [1, 2], [1, 2]]));
    assert_eq!(mat, Mat::from_iter([&[1, 2], &[1, 2], &[1, 2]]));
    assert_eq!(mat, Mat::from_iter(&[[1, 2], [1, 2], [1, 2]]));
}

#[test]
fn unit() {
    let a = Mat::diagonal_from_fn(5, |_| 1);
    let b = Mat::scalar(5, 1);
    assert_eq!(a, b);
    let arr_arr = [
        [1, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1],
    ];
    let arr_vec = [
        vec![1, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1],
    ];
    let vec_arr = vec![
        [1, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1],
    ];
    let vec_ref_arr = vec![
        &[1, 0, 0, 0, 0],
        &[0, 1, 0, 0, 0],
        &[0, 0, 1, 0, 0],
        &[0, 0, 0, 1, 0],
        &[0, 0, 0, 0, 1],
    ];
    let vec_slice_arr = vec![
        [1, 0, 0, 0, 0].as_slice(),
        [0, 1, 0, 0, 0].as_slice(),
        [0, 0, 1, 0, 0].as_slice(),
        [0, 0, 0, 1, 0].as_slice(),
        [0, 0, 0, 0, 1].as_slice(),
    ];
    let vec_vec = vec![
        vec![1, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1],
    ];
    assert_eq!(a, arr_arr);
    assert_eq!(b, arr_arr);
    assert_eq!(a, arr_vec);
    assert_eq!(b, arr_vec);
    assert_eq!(a, vec_arr);
    assert_eq!(b, vec_arr);
    assert_eq!(a, vec_ref_arr);
    assert_eq!(b, vec_ref_arr);
    assert_eq!(a, vec_slice_arr);
    assert_eq!(b, vec_slice_arr);
    assert_eq!(a, vec_vec);
    assert_eq!(b, vec_vec);
    assert_eq!(Mat::from(arr_arr), a);
    assert_eq!(Mat::from(arr_arr), arr_arr);
}
