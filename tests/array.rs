use iof::*;
use std::io::Cursor;

#[test]
fn read_array_0() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 0] = reader.read();
    assert_eq!(vec, []);

    assert!(<[u32; 0]>::try_read_from(&mut reader).is_ok());
    assert!(<[u32; 1]>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_boxed_array_0() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Box<[i32; 0]> = reader.read();
    assert_eq!(vec.as_ref(), &[]);

    assert!(<Box<[u32; 0]>>::try_read_from(&mut reader).is_ok());
    assert!(<Box<[u32; 1]>>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_array_1() {
    let reader = Cursor::new("1".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 1] = reader.read();
    assert_eq!(vec, [1]);

    assert!(<[u32; 0]>::try_read_from(&mut reader).is_ok());
    assert!(<[u32; 1]>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_boxed_array_1() {
    let reader = Cursor::new("1".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Box<[i32; 1]> = reader.read();
    assert_eq!(vec.as_ref(), &[1]);

    assert!(<Box<[u32; 0]>>::try_read_from(&mut reader).is_ok());
    assert!(<Box<[u32; 1]>>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_array_4() {
    let reader = Cursor::new("1 -2 3 -4".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 4] = reader.read();
    assert_eq!(vec, [1, -2, 3, -4]);

    assert!(<[u32; 0]>::try_read_from(&mut reader).is_ok());
    assert!(<[u32; 1]>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_boxed_array_4() {
    let reader = Cursor::new("1 -2 3 -4".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Box<[i32; 4]> = reader.read();
    assert_eq!(vec.as_ref(), &[1, -2, 3, -4]);

    assert!(<Box<[u32; 0]>>::try_read_from(&mut reader).is_ok());
    assert!(<Box<[u32; 1]>>::try_read_from(&mut reader).is_err());
}

#[test]
fn try_read_array_4() {
    let reader = Cursor::new("1 -2 3 -4".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: [i32; 4] = reader.try_read().unwrap();
    assert_eq!(vec, [1, -2, 3, -4]);

    assert!(<[u32; 0]>::try_read_from(&mut reader).is_ok());
    assert!(<[u32; 1]>::try_read_from(&mut reader).is_err());
}

#[test]
fn try_read_boxed_array_4() {
    let reader = Cursor::new("1 -2 3 -4".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Box<[i32; 4]> = reader.try_read().unwrap();
    assert_eq!(vec.as_ref(), &[1, -2, 3, -4]);

    assert!(<Box<[u32; 0]>>::try_read_from(&mut reader).is_ok());
    assert!(<Box<[u32; 1]>>::try_read_from(&mut reader).is_err());
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn read_array_insufficient() {
    let reader = Cursor::new("-1 -2".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: [i32; 4] = reader.read();
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn read_boxed_array_insufficient() {
    let reader = Cursor::new("-1 -2".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: Box<[i32; 4]> = reader.read();
}

#[test]
fn display() {
    let s = [1, 2, 3];
    assert_eq!(s.try_write_into_string_with_sep(&[" "]).unwrap(), "1 2 3");

    let s = [[1, 2, 3], [4, 5, 6]];
    assert_eq!(
        s.try_write_into_string_with_sep(&["\n", " "]).unwrap(),
        "1 2 3\n4 5 6",
    );

    let s = [[1, 2], [3, 4]];
    assert_eq!(
        s.try_write_into_string_with_sep(&["\n", " "]).unwrap(),
        "1 2\n3 4",
    );

    let s = [[1, 2]];
    assert_eq!(
        s.try_write_into_string_with_sep(&["\n", " "]).unwrap(),
        "1 2",
    );

    let s: [[usize; 0]; 0] = [];
    assert_eq!(s.try_write_into_string_with_sep(&[" "]).unwrap(), "");
}
