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
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn read_panic() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read();
}
