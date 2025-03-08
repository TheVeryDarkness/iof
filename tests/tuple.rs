use fmt::Default;
use iof::*;
use std::{io::Cursor, num::NonZero, str::from_utf8};

#[test]
fn read_tuple_3() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: (u32, u32, u32) = reader.read();
    assert_eq!(vec, (1, 2, 3));

    assert!(<u32>::try_read_from(&mut reader, Default::new()).is_err());
}

#[test]
fn read_tuple_5() {
    let reader = Cursor::new("1 2 3.0 false 1".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: (u32, char, f64, bool, NonZero<u32>) = reader.read();
    assert_eq!(vec, (1, '2', 3.0, false, 1.try_into().unwrap()));

    assert!(<char>::try_read_from(&mut reader, Default::new()).is_err());
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn try_read_tuple_3_insufficient_err() {
    let reader = Cursor::new("1".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: (i32, i8, u32) = reader.read();
}

#[test]
#[should_panic = "stream did not contain valid UTF-8"]
fn try_read_tuple_3_encoding_err() {
    let reader = Cursor::new(b"1 \xff 3");
    let mut reader = InputStream::new(reader);

    let _: (i32, i8, u32) = reader.read();
}

#[test]
fn try_read_tuple_3_from_str_err() {
    let reader = Cursor::new("l m n".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Result<(char, String, bool), _> = reader.try_read();
    let err = vec.unwrap_err();
    assert_eq!(err.to_string(), "error during converting a string \"n\" to a value of `bool`: provided string was not `true` or `false`");
    assert_eq!(
        format!("{:?}", err),
        "FromStrError(T3(ParseBoolError), \"n\", \"bool\")"
    );
}

#[test]
#[cfg_attr(
    feature = "c-compatible",
    should_panic = "found unexpected character at the end of the string \"-\" during converting it to a value of \"u32\""
)]
#[cfg_attr(
    not(feature = "c-compatible"),
    should_panic = "error during converting a string \"-3\" to a value of `u32`: invalid digit found in string"
)]
fn read_tuple_3_unexpected_char_err() {
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

    assert!(<u32>::try_read_from(&mut reader, Default::new()).is_err());
}

#[test]
fn write() {
    let mut buf = Vec::new();
    let vec = (1, 2, 3);
    unwrap!(vec.try_write_into(&mut buf));
    assert_eq!(from_utf8(&buf).unwrap(), "1 2 3");

    buf.clear();
    let vec = (1,);
    unwrap!(vec.try_write_into(&mut buf));
    assert_eq!(from_utf8(&buf).unwrap(), "1");

    buf.clear();
    let vec = ((1, 2), (3, 4));
    unwrap!(vec.try_write_into(&mut buf));
    assert_eq!(from_utf8(&buf).unwrap(), "1 2\n3 4");

    buf.clear();
    let vec = ();
    unwrap!(vec.try_write_into(&mut buf));
    assert_eq!(from_utf8(&buf).unwrap(), "");

    buf.clear();
    let vec = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    unwrap!(vec.try_write_into(&mut buf));
    assert_eq!(from_utf8(&buf).unwrap(), "1 2 3\n4 5 6\n7 8 9");

    buf.clear();
    let vec = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    unwrap!(vec.try_write_into_with_sep(&mut buf, &[' '; 0]));
    assert_eq!(from_utf8(&buf).unwrap(), "1 2 3\n4 5 6\n7 8 9");

    buf.clear();
    let vec = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    unwrap!(vec.try_write_into_with_sep(&mut buf, ' '));
    assert_eq!(from_utf8(&buf).unwrap(), "1 2 3 4 5 6 7 8 9");
}

#[test]
fn show() {
    use std::str::from_utf8;
    let mut buf = Vec::new();
    buf.clear();
    show!((1, 2, 3) => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 2 3\n");

    buf.clear();
    show!(((1, 2), (3, 4)) => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 2\n3 4\n");

    buf.clear();
    show!(() => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "\n");

    buf.clear();
    show!(((1, 2, 3), (4, 5, 6), (7, 8, 9)) => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 2 3\n4 5 6\n7 8 9\n");

    buf.clear();
    show!(((1, 2, 3), (4, 5, 6), (7, 8, 9)) => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 2 3\n4 5 6\n7 8 9\n");
}
