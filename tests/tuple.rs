use iof::*;
use std::io::Cursor;

#[test]
fn read_tuple_3() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: (u32, u32, u32) = reader.read();
    assert_eq!(vec, (1, 2, 3));

    assert!(<u32>::try_read_from(&mut reader).is_err());
}

#[test]
fn try_read_tuple_3_from_str_err() {
    let reader = Cursor::new("1 2 -3".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Result<(i32, i8, u32), _> = reader.try_read();
    let err = vec.unwrap_err();
    assert_eq!(err.to_string(), "invalid digit found in string");
    assert_eq!(
        format!("{:?}", err),
        "T3(ParseIntError { kind: InvalidDigit })"
    );
}

#[test]
#[should_panic = "invalid digit found in string"]
fn read_tuple_3_from_str_err() {
    let reader = Cursor::new("1 2 -3".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: (u32, u32, u32) = reader.read();
}

#[test]
fn read_tuple_12() {
    let reader = Cursor::new("1 2 3 4 5 6 7 8 9 10 11 12".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) = reader.read();
    assert_eq!(vec, (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));

    assert!(<u32>::try_read_from(&mut reader).is_err());
}
