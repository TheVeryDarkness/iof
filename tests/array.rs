use iof::*;
use std::io::Cursor;

#[test]
fn read_array_1() {
    let reader = Cursor::new("1".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 1] = *reader.read_array();
    assert_eq!(vec, [1]);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
fn read_array_4() {
    let reader = Cursor::new("1 -2 3 -4".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 4] = *reader.read_array();
    assert_eq!(vec, [1, -2, 3, -4]);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn read_array_insuffcient() {
    let reader = Cursor::new("-1 -2".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: [i32; 4] = *reader.read_array();
}
