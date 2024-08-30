use iof::*;
use std::{io::Cursor, iter::repeat, vec};

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
    assert_eq!(mat.first_row(), Some([1, 2, 3].as_slice()));
    assert_eq!(mat.last_row(), Some([4, 5, 6].as_slice()));
    assert_eq!(&mat[0], [1, 2, 3].as_slice());
    assert_eq!(&mat[1], [4, 5, 6].as_slice());
    assert_eq!(mat.len_rows(), 2);
    assert_eq!(mat.len_columns(), 3);
    assert_eq!(mat.iter().len(), 2);
    assert_eq!(mat.iter().count(), 2);
    assert_eq!(mat.iter().size_hint(), (2, Some(2)));
    assert_eq!(mat.iter().last(), Some([4, 5, 6].as_slice()));
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
    assert_eq!(mat.iter().collect::<Mat<_>>(), mat);
    assert_eq!(mat.transpose(), Mat::from_iter([[1, 4], [2, 5], [3, 6]]));
    assert_eq!(mat.transpose().transpose(), mat);
    assert_eq!(mat.last_row(), Some([4, 5, 6].as_slice()));

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
fn read_same_rows() {
    let reader = Cursor::new("2 3 2\n2 3 2\n2 3 2".as_bytes());
    let mut reader = InputStream::new(reader);

    let mat: Mat<u32> = reader.read_m_n(3, 3);
    assert_eq!(mat.first_row(), Some([2, 3, 2].as_slice()));
    assert_eq!(mat.last_row(), Some([2, 3, 2].as_slice()));
    assert_eq!(&mat[0], &[2, 3, 2]);
    assert_eq!(&mat[1], &[2, 3, 2]);
    assert_eq!(&mat[2], &[2, 3, 2]);
    assert_eq!(mat.len_rows(), 3);
    assert_eq!(mat.len_columns(), 3);
    assert_eq!(mat.iter().len(), 3);
    assert_eq!(mat.iter().count(), 3);
    assert_eq!(mat.iter().size_hint(), (3, Some(3)));
    assert_eq!(mat.iter().last(), Some([2, 3, 2].as_slice()));
    assert_eq!(format!("{:?}", mat), "[[2, 3, 2], [2, 3, 2], [2, 3, 2]]");
    assert_eq!(mat.iter().collect::<Mat<_>>(), mat);
    assert_eq!(mat, Mat::with_repeat(3, vec![2, 3, 2]));

    let mut iter = mat.iter();
    let mut i = 0;
    while let Some(row) = iter.next() {
        assert_eq!(row, [2, 3, 2].as_slice());
        assert_eq!(iter.len() + i + 1, mat.len_rows());
        assert_eq!(
            format!("{:?}", iter),
            format!(
                "[{}]",
                repeat("[2, 3, 2]")
                    .take(mat.len_rows() - 1 - i)
                    .sep_by(", ")
            )
        );
        i += 1;
    }
}

#[test]
fn read_all_same() {
    let reader = Cursor::new("2 2 2\n2 2 2".as_bytes());
    let mut reader = InputStream::new(reader);

    let mat: Mat<u32> = reader.read_m_n(2, 3);
    assert_eq!(mat.first_row(), Some([2, 2, 2].as_slice()));
    assert_eq!(mat.last_row(), Some([2, 2, 2].as_slice()));
    assert_eq!(&mat[0], [2, 2, 2].as_slice());
    assert_eq!(&mat[1], [2, 2, 2].as_slice());
    assert_eq!(mat.len_rows(), 2);
    assert_eq!(mat.len_columns(), 3);
    assert_eq!(mat.iter().len(), 2);
    assert_eq!(mat.iter().count(), 2);
    assert_eq!(mat.iter().size_hint(), (2, Some(2)));
    assert_eq!(mat.iter().last(), Some([2, 2, 2].as_slice()));
    assert_eq!(format!("{:?}", mat), "[[2, 2, 2], [2, 2, 2]]");
    assert_eq!(mat.iter().collect::<Mat<_>>(), mat);
    assert_eq!(mat, Mat::with_clone(2, 3, 2));
}

#[test]
fn display() {
    let s = Mat::from([[1, 2, 3], [4, 5, 6]]);
    assert_eq!(s.try_write_into_string().unwrap(), "1 2 3\n4 5 6");
    let s = Mat::from([[1, 2, 3]]);
    assert_eq!(s.try_write_into_string().unwrap(), "1 2 3");
    let s: Mat<i32> = Mat::new();
    assert_eq!(s.try_write_into_string().unwrap(), "");
}
