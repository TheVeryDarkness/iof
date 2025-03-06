use iof::{dimension::Dimension, InputStream, Mat, ReadInto, ReadOneInto};
use std::io::Cursor;

#[test]
fn check_separator() {
    assert_eq!(<Vec<bool> as Dimension>::get_default_separator(), " ");
    assert_eq!(<Mat<bool> as Dimension>::get_default_separator(), "\n");
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn from_empty() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: bool = reader.read();
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn from_all_spaces() {
    let reader = Cursor::new("  \n\r\n \t \r\n".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: bool = reader.read();
}

#[test]
#[should_panic = "error during converting a string \"abc\" to a value of `bool`: provided string was not `true` or `false`"]
fn from_multiple() {
    let reader = Cursor::new("abc".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: bool = reader.read_in_line_some_trimmed();
}
