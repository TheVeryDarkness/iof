use super::line_buf::LineBuf;
use crate::{unwrap, BufReadExt, InputStream};
use std::io::Cursor;

fn all(stream: &mut impl BufReadExt) {
    let c = unwrap!(stream.try_get());
    assert_eq!(c, 'H');

    let c = unwrap!(stream.try_peek());
    assert_eq!(c, 'e');

    let c = unwrap!(stream.try_get_if(&['H', 'e']));
    assert_eq!(c, Some('e'));

    let () = unwrap!(stream.try_skip_eol());

    let c = unwrap!(stream.try_skip_all(&['l', 'o']));
    assert_eq!(c, 3);

    let s = unwrap!(stream.try_get_until_in_line(&['!']));
    assert_eq!(s, ", world");

    let s = unwrap!(stream.try_get_string_some());
    assert_eq!(s, "!");
}

#[test]
fn line_buf() {
    let s = "Hello, world!";
    let mut stream = LineBuf::new(s);
    all(&mut stream);
}

#[test]
fn ref_line_buf() {
    let s = "Hello, world!";
    let mut stream = LineBuf::new(s);
    all(&mut &mut stream);
}

#[test]
fn input_stream() {
    let s = "Hello, world!";
    let mut stream = InputStream::new(Cursor::new(s));
    all(&mut stream);
}

#[test]
fn ref_input_stream() {
    let s = "Hello, world!";
    let mut stream = InputStream::new(Cursor::new(s));
    all(&mut &mut stream);
}
