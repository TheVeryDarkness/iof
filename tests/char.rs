use iof::{dimension::Dimension, InputStream, Mat, ReadInto, ReadOneInto};
use std::io::Cursor;

#[test]
fn check_separator() {
    assert_eq!(<Vec<char> as Dimension>::get_default_separator(), "");
    assert_eq!(<Mat<char> as Dimension>::get_default_separator(), "\n");
}

#[test]
#[should_panic = "not implemented: Default separator for dimension 0 is not supported."]
fn check_separator_scalar() {
    let _ = <char as Dimension>::get_default_separator();
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn from_empty() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: char = reader.read();
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn from_all_spaces() {
    let reader = Cursor::new("  \n\r\n \t \r\n".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: char = reader.read();
}

#[test]
#[should_panic = "error during converting a string \"abc\" to a value of `char`: too many characters in string"]
fn from_multiple() {
    let reader = Cursor::new("abc".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: char = reader.read_in_line_some_trimmed();
}
