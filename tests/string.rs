use iof::{show, unwrap, InputStream, ReadFrom, ReadInto, ReadOneFrom, ReadOneInto};
use std::io::Cursor;

#[allow(dead_code)]
mod check_impl {
    use iof::{dimension::Dimension, separator::GetDefaultSeparator};
    const fn check_impl<T: Dimension + GetDefaultSeparator + ?Sized>() {}
    const STRING: () = check_impl::<String>();
    const STR: () = check_impl::<str>();
    const STATIC_STR: () = check_impl::<&'static str>();
}

#[test]
fn read_strings() {
    let reader = Cursor::new("Hello, World!".as_bytes());
    let mut reader = InputStream::new(reader);

    let hello: String = reader.read();
    let world: String = reader.read();
    assert_eq!(hello, "Hello,");
    assert_eq!(world, "World!");
    assert!(<String>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_string_vec() {
    let reader = Cursor::new("There are 4 strings.".as_bytes());
    let mut reader = InputStream::new(reader);

    let strings: Vec<String> = reader.read_n(4);
    assert_eq!(strings, vec!["There", "are", "4", "strings."]);
    assert!(<String>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_all_strings() {
    let reader = Cursor::new("There are 4 strings.".as_bytes());
    let mut reader = InputStream::new(reader);

    let strings: Vec<String> = reader.read_all();
    assert_eq!(strings, vec!["There", "are", "4", "strings."]);
    assert!(<String>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_line() {
    let reader = Cursor::new("\n\nThere are 4 strings.".as_bytes());
    let mut reader = InputStream::new(reader);

    let strings: String = reader.read_in_line_some_trimmed();
    assert_eq!(strings, "There are 4 strings.");
    assert!(<String>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_in_line() {
    let reader = Cursor::new("There are 4 strings.\nThere are 3 lines.\n".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: String = reader.read_in_line_trimmed();
    assert_eq!(a, "There are 4 strings.");

    let b: String = reader.read_in_line_trimmed();
    assert_eq!(b, "There are 3 lines.");

    assert!(<String>::try_read_some_in_line_from(&mut reader).is_err());
}

#[test]
fn read_in_line_some_trimmed_spaces() {
    let reader = Cursor::new(" s ".as_bytes());
    let mut reader = InputStream::new(reader);

    let s: String = reader.read_in_line_some_trimmed();
    assert_eq!(s, "s");
    assert!(<String>::try_read_some_in_line_from(&mut reader).is_err());
}

#[test]
#[should_panic = "number too large to fit in target type"]
fn read_in_line_from_str_err() {
    let reader = Cursor::new("123456789".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: u16 = reader.read_in_line_trimmed();
}

#[test]
#[should_panic = "expect more characters before EOF"]
fn read_in_line_some_failure() {
    let reader = Cursor::new("\n \n \n".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: String = reader.read_in_line_some_trimmed();
}

#[test]
fn read_unicode() {
    let reader = Cursor::new("🦀🦀🦀 Rust 你好！ καλημέρα ".as_bytes());
    let mut reader = InputStream::new(reader);

    let s: String = reader.read();
    assert_eq!(s, "🦀🦀🦀");

    let s: String = reader.read();
    assert_eq!(s, "Rust");

    let s: String = reader.read();
    assert_eq!(s, "你好！");

    let s: String = reader.read();
    assert_eq!(s, "καλημέρα");

    assert!(<String>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_in_line_trimmed_unicode() {
    let reader = Cursor::new("🦀🦀🦀\nRust \n 你好！\n καλημέρα ".as_bytes());
    let mut reader = InputStream::new(reader);

    let s: String = reader.read_in_line_trimmed();
    assert_eq!(s, "🦀🦀🦀");

    let s: String = reader.read_in_line_trimmed();
    assert_eq!(s, "Rust");

    let s: String = reader.read_in_line_trimmed();
    assert_eq!(s, "你好！");

    let s: String = reader.read_in_line_trimmed();
    assert_eq!(s, "καλημέρα");

    assert!(<String>::try_read_in_line_trimmed_from(&mut reader).is_err());
}

#[test]
fn read_in_line_some_unicode() {
    let reader = Cursor::new("🦀🦀🦀\nRust \n 你好！\n καλημέρα ".as_bytes());
    let mut reader = InputStream::new(reader);

    let s: String = reader.read_in_line_some_trimmed();
    assert_eq!(s, "🦀🦀🦀");

    let s: String = reader.read_in_line_some_trimmed();
    assert_eq!(s, "Rust");

    let s: String = reader.read_in_line_some_trimmed();
    assert_eq!(s, "你好！");

    let s: String = reader.read_in_line_some_trimmed();
    assert_eq!(s, "καλημέρα");

    assert!(<String>::try_read_in_line_some_trimmed_from(&mut reader).is_err());
}

#[test]
fn string() {
    let mut buf = Cursor::new(Vec::new());
    show!("Hello, World!", end = "" => &mut buf);
    show!("🦀🦀🦀", => &mut buf);
    assert_eq!(
        unwrap!(String::from_utf8(buf.into_inner())),
        "Hello, World!🦀🦀🦀\n",
    );
}
