use super::line_buf::LineBuf;
use crate::{unwrap, BufReadExt, InputStream};
use std::io::Cursor;

/// Test all methods.
///
/// Pass "Hello, world!\r\n" to the stream and test all methods.
fn all_1(stream: &mut impl BufReadExt) {
    let c = unwrap!(stream.try_get());
    assert_eq!(c, 'H');

    let c = unwrap!(stream.try_peek());
    assert_eq!(c, 'e');

    let c = unwrap!(stream.try_get_if(&['H', 'e']));
    assert_eq!(c, Some('e'));

    let () = unwrap!(stream.try_skip_eol());

    assert!(stream.try_get_if(&[]).unwrap().is_none());

    let c = unwrap!(stream.try_skip_all(&['l', 'o']));
    assert_eq!(c, 3);

    let s = unwrap!(stream.try_skip_all(&[' ', '!']));
    assert_eq!(s, 0);

    let s = unwrap!(stream.try_get_until_in_line(&['!']));
    assert_eq!(s, ", world");

    let s = unwrap!(stream.try_get_string_some());
    assert_eq!(s, "!");

    let c = unwrap!(stream.try_peek());
    assert_eq!(c, '\r');

    unwrap!(stream.try_skip_eol());

    assert!(stream.try_peek().is_err());
    assert!(stream.try_get().is_err());
    assert!(stream
        .try_get_if(&['H', 'e', 'l', 'o', ',', 'w', 'r', 'd', '!', '\r', '\n'])
        .is_err());
    assert!(stream.try_get_if(&[]).is_err());
}

#[test]
fn line_buf() {
    let s = "Hello, world!\r\n";
    let mut stream = LineBuf::new(s);
    all_1(&mut stream);
}

#[test]
fn ref_line_buf() {
    let s = "Hello, world!\r\n";
    let mut stream = LineBuf::new(s);
    all_1(&mut &mut stream);
}

#[test]
fn input_stream() {
    let s = "Hello, world!\r\n";
    let mut stream = InputStream::new(Cursor::new(s));
    all_1(&mut stream);
}

#[test]
fn ref_input_stream() {
    let s = "Hello, world!\r\n";
    let mut stream = InputStream::new(Cursor::new(s));
    all_1(&mut &mut stream);
}
