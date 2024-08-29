use iof::*;
use std::io::Cursor;

#[test]
fn read_array_0() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 0] = reader.read();
    assert_eq!(vec, []);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
fn read_boxed_array_0() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Box<[i32; 0]> = reader.read();
    assert_eq!(vec.as_ref(), &[]);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
fn read_array_1() {
    let reader = Cursor::new("1".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 1] = reader.read();
    assert_eq!(vec, [1]);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
fn read_boxed_array_1() {
    let reader = Cursor::new("1".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Box<[i32; 1]> = reader.read();
    assert_eq!(vec.as_ref(), &[1]);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
fn read_array_4() {
    let reader = Cursor::new("1 -2 3 -4".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 4] = reader.read();
    assert_eq!(vec, [1, -2, 3, -4]);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
fn read_boxed_array_4() {
    let reader = Cursor::new("1 -2 3 -4".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Box<[i32; 4]> = reader.read();
    assert_eq!(vec.as_ref(), &[1, -2, 3, -4]);

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
}

#[test]
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn read_array_insuffcient() {
    let reader = Cursor::new("-1 -2".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: [i32; 4] = reader.read();
}

#[test]
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn read_boxed_array_insuffcient() {
    let reader = Cursor::new("-1 -2".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: Box<[i32; 4]> = reader.read();
}
