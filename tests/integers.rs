use dimension::Dimension;
use iof::*;
use std::io::Cursor;

#[test]
fn check_separator() {
    assert_eq!(<Vec<u32> as Dimension>::get_default_separator(), " ");
    assert_eq!(<Vec<i32> as Dimension>::get_default_separator(), " ");
}

#[test]
fn try_read_single_3() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.try_read_one().unwrap();
    assert_eq!(a, 1);

    let b: u32 = reader.try_read_one().unwrap();
    assert_eq!(b, 2);

    let c: u32 = reader.try_read_one().unwrap();
    assert_eq!(c, 3);

    assert!(<u32>::try_read_one_from(&mut reader).is_err());
}

#[test]
fn read_single_3() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_one();
    assert_eq!(a, 1);

    let b: u32 = reader.read_one();
    assert_eq!(b, 2);

    let c: u32 = reader.read_one();
    assert_eq!(c, 3);

    assert!(<u32>::try_read_from(&mut reader).is_err());
}

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

    assert!(<u32>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_char_3() {
    let reader = Cursor::new("123".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_in_char();
    assert_eq!(a, 1);

    let b: u32 = reader.read_in_char();
    assert_eq!(b, 2);

    let c: u32 = reader.read_in_char();
    assert_eq!(c, 3);

    assert!(<u32>::try_read_in_char_from(&mut reader).is_err());
}

#[test]
fn read_char_in_3_lines() {
    let reader = Cursor::new("\n1\n2\n3".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_in_char();
    assert_eq!(a, 1);

    let b: u32 = reader.read_in_char();
    assert_eq!(b, 2);

    let c: u32 = reader.read_in_char();
    assert_eq!(c, 3);

    assert!(<u32>::try_read_in_char_from(&mut reader).is_err());
}

#[test]
fn read_one_then_all_in_line() {
    let reader = Cursor::new("1\n2 3 4\n 5 6 7".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_one();
    assert_eq!(a, 1);

    let b: Vec<u32> = reader.read_any_in_line();
    assert_eq!(b, []);

    let c: Vec<u32> = reader.read_any_in_line();
    assert_eq!(c, [2, 3, 4]);

    let d: Vec<u32> = reader.read_any_in_line();
    assert_eq!(d, [5, 6, 7]);

    assert!(<u32>::try_read_any_in_line_from(&mut reader).is_err());
}

#[test]
fn read_one_then_all_in_line_some() {
    let reader = Cursor::new("1\n2 3 4\n 5 6 7".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_one();
    assert_eq!(a, 1);

    let b: Vec<u32> = reader.read_some_in_line();
    assert_eq!(b, [2, 3, 4]);

    let c: Vec<u32> = reader.read_some_in_line();
    assert_eq!(c, [5, 6, 7]);

    assert!(<u32>::try_read_some_in_line_from(&mut reader).is_err());
}
#[test]
fn read_all() {
    let reader = Cursor::new("1 2\n 3 4\n 5 6 \n7".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: Vec<u32> = reader.read_all();
    assert_eq!(a, [1, 2, 3, 4, 5, 6, 7]);

    assert!(<u32>::try_read_one_from(&mut reader).is_err());
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn read_char_empty() {
    let reader = Cursor::new(" \n\n \n  \n ".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read_in_char();
}

#[test]
#[should_panic = "invalid digit found in string"]
fn read_sign_error() {
    let reader = Cursor::new("-1".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read();
}

#[test]
#[should_panic = "Error during converting a string \"-1\" to a value of `u32`: invalid digit found in string"]
fn try_read_sign_error() {
    let reader = Cursor::new("-1".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = unwrap!(reader.try_read());
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn read_empty() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read();
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn try_read_empty() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = unwrap!(reader.try_read());
}

#[test]
#[should_panic = "Error during converting a string \"1 2 3\" to a value of `u32`: invalid digit found in string"]
fn try_read_line_too_much() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = unwrap!(reader.try_read_in_line_some_trimmed());
}

#[test]
#[should_panic = "Error during converting a string \"-\" to a value of `i32`: invalid digit found in string"]
fn try_read_char_only_sign() {
    let reader = Cursor::new("-1".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: i32 = unwrap!(reader.try_read_in_char());
}

#[test]
fn try_write_one_into() {
    let mut s = Vec::new();
    42.try_write_into(&mut s).unwrap();
    let s = String::from_utf8(s).unwrap();
    assert_eq!(s, "42");
}
