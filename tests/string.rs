use iof::{InputStream, ReadInto};
use std::io::Cursor;

#[test]
fn read_strings() {
    let reader = Cursor::new("Hello, World!".as_bytes());
    let mut reader = InputStream::new(reader);

    let hello: String = reader.read();
    let world: String = reader.read();
    assert_eq!(hello, "Hello,");
    assert_eq!(world, "World!");
    assert!(iof::ReadInto::<String>::try_read(&mut reader).is_err());
}

#[test]
fn read_string_vec() {
    let reader = Cursor::new("There are 4 strings.".as_bytes());
    let mut reader = InputStream::new(reader);

    let strings: Vec<String> = reader.read_n(4);
    assert_eq!(strings, vec!["There", "are", "4", "strings."]);
    assert!(iof::ReadInto::<String>::try_read(&mut reader).is_err());
}

#[test]
fn read_all_strings() {
    let reader = Cursor::new("There are 4 strings.".as_bytes());
    let mut reader = InputStream::new(reader);

    let strings: Vec<String> = reader.read_all().collect();
    assert_eq!(strings, vec!["There", "are", "4", "strings."]);
    assert!(iof::ReadInto::<String>::try_read(&mut reader).is_err());
}

#[test]
fn read_line() {
    let reader = Cursor::new("\n\nThere are 4 strings.".as_bytes());
    let mut reader = InputStream::new(reader);

    let strings: String = reader.read_line();
    assert_eq!(strings, "There are 4 strings.");
    assert!(iof::ReadInto::<String>::try_read(&mut reader).is_err());
}

#[test]
fn read_remained_line() {
    let reader = Cursor::new("There are 4 strings.\n\n".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: String = reader.read_remained_line();
    assert_eq!(a, "There are 4 strings.");

    let b: String = reader.read_remained_line();
    assert_eq!(b, "");

    let c: String = reader.read_remained_line();
    assert_eq!(c, "");

    let d: String = reader.read_remained_line();
    assert_eq!(d, "");

    assert!(iof::ReadInto::<String>::try_read(&mut reader).is_err());
}

#[test]
fn read_line_spaces() {
    let reader = Cursor::new(" s ".as_bytes());
    let mut reader = InputStream::new(reader);

    let s: String = reader.read_line();
    assert_eq!(s, "s");
    assert!(iof::ReadInto::<String>::try_read(&mut reader).is_err());
}

#[test]
#[should_panic = "failed to read a non-whitespace character before EOF"]
fn read_line_failure() {
    let reader = Cursor::new("".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: String = reader.read_line();
}
