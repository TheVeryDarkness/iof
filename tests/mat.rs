use iof::*;
use std::{io::Cursor, vec};

#[test]
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn read_m_n_insufficient() {
    let reader = Cursor::new("1 2\n3".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: Mat<u32> = reader.read_m_n(2, 2);
}

#[test]
fn read_m_n() {
    let reader = Cursor::new("1 2 3\n4 5 6".as_bytes());
    let mut reader = InputStream::new(reader);

    let mat: Mat<u32> = reader.read_m_n(2, 3);
    assert_eq!(&mat[0], [1, 2, 3].as_slice());
    assert_eq!(&mat[1], [4, 5, 6].as_slice());
    assert_eq!(mat.iter().len(), 2);
    assert_eq!(mat.iter().count(), 2);
    assert_eq!(mat.iter().size_hint(), (2, Some(2)));
    assert_eq!(format!("{:?}", mat), "[[1, 2, 3], [4, 5, 6]]");
    assert_eq!(
        format!("{:#?}", mat),
        "\
[
    [
        1,
        2,
        3,
    ],
    [
        4,
        5,
        6,
    ],
]"
    );

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
fn read_same_rows() {
    let reader = Cursor::new("2 3 2\n2 3 2\n2 3 2".as_bytes());
    let mut reader = InputStream::new(reader);

    let mat: Mat<u32> = reader.read_m_n(3, 3);
    assert_eq!(&mat[0], &[2, 3, 2]);
    assert_eq!(&mat[1], &[2, 3, 2]);
    assert_eq!(&mat[2], &[2, 3, 2]);
    assert_eq!(mat.iter().len(), 3);
    assert_eq!(mat.iter().count(), 3);
    assert_eq!(mat.iter().size_hint(), (3, Some(3)));
    assert_eq!(format!("{:?}", mat), "[[2, 3, 2], [2, 3, 2], [2, 3, 2]]");

    let iter = mat.iter();
    for row in iter {
        assert_eq!(row, [2, 3, 2].as_slice());
    }
}

#[test]
fn read_all_same() {
    let reader = Cursor::new("2 2 2\n2 2 2".as_bytes());
    let mut reader = InputStream::new(reader);

    let mat: Mat<u32> = reader.read_m_n(2, 3);
    assert_eq!(&mat[0], [2, 2, 2].as_slice());
    assert_eq!(&mat[1], [2, 2, 2].as_slice());
    assert_eq!(mat.iter().len(), 2);
    assert_eq!(mat.iter().count(), 2);
    assert_eq!(mat.iter().size_hint(), (2, Some(2)));
    assert_eq!(format!("{:?}", mat), "[[2, 2, 2], [2, 2, 2]]");
}

#[test]
fn display() {
    let s = Mat::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    assert_eq!(s.try_write_into_string().unwrap(), "1 2 3\n4 5 6");
    let s = Mat::from(vec![vec![1, 2, 3]]);
    assert_eq!(s.try_write_into_string().unwrap(), "1 2 3");
    let s: Mat<i32> = Mat::new();
    assert_eq!(s.try_write_into_string().unwrap(), "");
}

#[test]
fn complex() {
    let reader = Cursor::new(
        b"
2
4 4
1 0 0 0
0 1 0 0
0 0 1 0
0 0 0 1
3 3
0 0 1
0 1 0
1 0 0
",
    );
    let mut reader = InputStream::new(reader);

    // Read the number of matrices.
    let c: usize = reader.read();
    assert_eq!(c, 2);

    let (m1, n1): (usize, usize) = reader.read();
    let mat1: Mat<i32> = reader.read_m_n(m1, n1);

    assert_eq!((m1, n1), (4, 4));
    assert_eq!(
        mat1,
        Mat::from(vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ]),
    );

    let (m2, n2): (usize, usize) = reader.read();
    let mat2: Mat<i32> = reader.read_m_n(m2, n2);

    assert_eq!((m2, n2), (3, 3));
    assert_eq!(
        mat2,
        Mat::from(vec![vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0]]),
    );
}
