use iof::*;
use std::io::Cursor;

#[test]
fn read_3() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read();
    assert_eq!(a, 1);

    let b: u32 = reader.read();
    assert_eq!(b, 2);

    let c: u32 = reader.read();
    assert_eq!(c, 3);

    assert!(iof::ReadInto::<u32>::try_read(&mut reader).is_err());
}

#[test]
fn read_char_3() {
    let reader = Cursor::new("123".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_char();
    assert_eq!(a, 1);

    let b: u32 = reader.read_char();
    assert_eq!(b, 2);

    let c: u32 = reader.read_char();
    assert_eq!(c, 3);

    assert!(iof::ReadInto::<u32>::try_read_char(&mut reader).is_err());
}

#[test]
fn read_char_in_3_lines() {
    let reader = Cursor::new("\n1\n2\n3".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_char();
    assert_eq!(a, 1);

    let b: u32 = reader.read_char();
    assert_eq!(b, 2);

    let c: u32 = reader.read_char();
    assert_eq!(c, 3);

    assert!(iof::ReadInto::<u32>::try_read_char(&mut reader).is_err());
}

#[test]
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn read_char_empty() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read_char();
}

#[test]
#[should_panic = "invalid digit found in string"]
fn read_sign_error() {
    let reader = Cursor::new("-1".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read();
}

#[test]
#[should_panic = "called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }"]
fn try_read_sign_error() {
    let reader = Cursor::new("-1".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.try_read().unwrap();
}

#[test]
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn read_empty() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read();
}

#[test]
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn try_read_empty() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.try_read().unwrap();
}
