use dimension::Dimension;
use fmt::Default;
use iof::*;
use std::io::Cursor;

#[test]
fn check_separator() {
    assert_eq!(<Vec<u32> as Dimension>::get_default_separator(), " ");
    assert_eq!(<Vec<i32> as Dimension>::get_default_separator(), " ");
    assert_eq!(<Vec<f64> as Dimension>::get_default_separator(), " ");
    assert_eq!(<Mat<u32> as Dimension>::get_default_separator(), "\n");
    assert_eq!(<Mat<i32> as Dimension>::get_default_separator(), "\n");
    assert_eq!(<Mat<f64> as Dimension>::get_default_separator(), "\n");
}

#[test]
#[should_panic = "not implemented: Default separator for dimension 0 is not supported."]
fn check_separator_scalar() {
    let _ = <f64 as Dimension>::get_default_separator();
}

#[test]
#[should_panic = "not implemented: Default separator for dimension 3 is not supported."]
fn check_separator_3_dim_tensor() {
    let _ = <Vec<Vec<Vec<f64>>> as Dimension>::get_default_separator();
}

#[test]
fn try_read() {
    for (s, i) in [
        ("0", 0u8),
        ("1", 1u8),
        ("2", 2u8),
        ("+2", 2u8),
        ("99", 99u8),
    ] {
        let mut reader = InputStream::new(s.as_bytes());

        let a: u32 = unwrap!(reader.try_read());
        assert_eq!(a, i.into());

        assert_eq!(reader.get_cur_line(), "");

        let mut reader = InputStream::new(s.as_bytes());

        let a: i32 = unwrap!(reader.try_read());
        assert_eq!(a, i.into());

        assert_eq!(reader.get_cur_line(), "");
    }
}

#[test]
#[cfg(feature = "c-compatible")]
fn try_read_truncated() {
    for (s, i, r) in [("0+", 0u8, "+"), ("10+", 10u8, "+")] {
        let mut reader = InputStream::new(s.as_bytes());

        let a: u32 = unwrap!(reader.try_read());
        assert_eq!(a, i.into());

        assert_eq!(reader.get_cur_line(), r);

        let mut reader = InputStream::new(s.as_bytes());

        let a: i32 = unwrap!(reader.try_read());
        assert_eq!(a, i.into());

        assert_eq!(reader.get_cur_line(), r);
    }
}

#[test]
fn try_read_error() {
    for s in ["+", "++", "x", ".", "", " "] {
        let reader = Cursor::new(s.as_bytes());
        let mut reader = InputStream::new(reader);

        let a: Result<u32, _> = reader.try_read();
        assert!(a.is_err());
        let a: Result<i32, _> = reader.try_read();
        assert!(a.is_err());
    }
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

    assert!(<u32>::try_read_one_from(&mut reader, Default::new()).is_err());
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

    assert!(<u32>::try_read_from(&mut reader, Default::new()).is_err());
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

    assert!(<u32>::try_read_from(&mut reader, Default::new()).is_err());
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

    assert!(<u32>::try_read_in_char_from(&mut reader, Default::new()).is_err());
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

    assert!(<u32>::try_read_in_char_from(&mut reader, Default::new()).is_err());
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

    assert!(<u32>::try_read_any_in_line_from(&mut reader, Default::new()).is_err());
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

    assert!(<u32>::try_read_some_in_line_from(&mut reader, Default::new()).is_err());
}
#[test]
fn read_all() {
    let reader = Cursor::new("1 2\n 3 4\n 5 6 \n7".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: Vec<u32> = reader.read_all();
    assert_eq!(a, [1, 2, 3, 4, 5, 6, 7]);

    assert!(<u32>::try_read_one_from(&mut reader, Default::new()).is_err());
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn read_char_empty() {
    let reader = Cursor::new(" \n\n \n  \n ".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read_in_char();
}

#[test]
#[cfg_attr(
    feature = "c-compatible",
    should_panic = "found unexpected character at the end of the string \"-\" during converting it to a value of \"u32\""
)]
#[cfg_attr(
    not(feature = "c-compatible"),
    should_panic = "error during converting a string \"-1\" to a value of `u32`: invalid digit found in string"
)]
fn read_sign_error() {
    let reader = Cursor::new("-1".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = reader.read();
}

#[test]
#[cfg_attr(
    feature = "c-compatible",
    should_panic = "found unexpected character at the end of the string \"-\" during converting it to a value of \"u32\""
)]
#[cfg_attr(
    not(feature = "c-compatible"),
    should_panic = "error during converting a string \"-1\" to a value of `u32`: invalid digit found in string"
)]
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
#[should_panic = "error during converting a string \"1 2 3\" to a value of `u32`: invalid digit found in string"]
fn try_read_line_too_much() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u32 = unwrap!(reader.try_read_in_line_some_trimmed());
}

#[test]
#[should_panic = "error during converting a string \"-\" to a value of `i32`: invalid digit found in string"]
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
