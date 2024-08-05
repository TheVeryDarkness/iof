use iof::*;
use std::io::Cursor;

#[test]
fn read_m_n_1() {
    let reader = Cursor::new("1 2 3\n4 5 6".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Mat<u32> = reader.read_m_n(2, 3);
    assert_eq!(vec.first_row(), Some([1, 2, 3].as_slice()));
    assert_eq!(vec.last_row(), Some([4, 5, 6].as_slice()));
    assert_eq!(vec.len_rows(), 2);
    assert_eq!(vec.len_columns(), 3);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}
