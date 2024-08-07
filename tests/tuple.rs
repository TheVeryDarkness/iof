use iof::*;
use std::io::Cursor;

#[test]
fn read_tuple_3() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: (u32, u32, u32) = reader.read_tuple();
    assert_eq!(vec, (1, 2, 3));

    assert!(iof::ReadInto::<u32>::try_read(&mut reader).is_err());
}

#[test]
fn read_tuple_3_err() {
    let reader = Cursor::new("1 2 -3".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Result<(u32, u32, u32), _> = reader.try_read_tuple();
    assert!(vec.is_err());
}

#[test]
fn read_tuple_12() {
    let reader = Cursor::new("1 2 3 4 5 6 7 8 9 10 11 12".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = reader.read_tuple();
    assert_eq!(vec, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));

    assert!(iof::ReadInto::<u32>::try_read(&mut reader).is_err());
}
