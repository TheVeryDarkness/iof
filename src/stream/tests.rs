use super::{
    ext::{self, CharExt},
    line_buf::LineBuf,
    traits::BufReadExtWithFormat,
};
use crate::{
    fmt::{Default, Format, Skip},
    stream::ext::Any,
    unwrap, BufReadExt, InputStream,
};
use std::io::Cursor;

/// Test all methods.
///
/// Pass "Hello, world!\r\n" to the stream and test all methods.
fn all_1<Char, S>(stream: &mut S)
where
    S: BufReadExt<Char> + BufReadExtWithFormat<Char>,
    for<'a> &'a Skip<Char>: Format<Char>,
    Char: CharExt + Copy + From<char> + Ord + PartialEq<char>,
    for<'a> &'a [Char]: ext::CharSet<Item = Char>,
    for<'a> &'a str: ext::StrExt<'a, Char>,
    char: From<Char>,
    Default<Char>: Format<Char>,
{
    let c = unwrap!(stream.try_get());
    assert_eq!(c, 'H');

    let c = unwrap!(stream.try_peek());
    assert_eq!(c, 'e');

    let c = unwrap!(stream.try_get_non_skipped(Default::<Char>::new().skip()));
    assert_eq!(c, 'e');

    let nl = unwrap!(stream.try_skip_eol());
    assert!(nl.is_none());

    let c = unwrap!(stream.try_skip_all(Skip::<Char>::from_iter(['l', 'o']).skip()));
    assert_eq!(c, 3);

    let s = unwrap!(stream.try_skip_all(Skip::<Char>::from_iter([' ', '!']).skip()));
    assert_eq!(s, 0);

    let s = unwrap!(stream.try_get_until_in_line(Skip::<Char>::from_iter(['!']).skip()));
    assert_eq!(s, ", world");

    let s = unwrap!(stream.try_get_string_some(Default::new().skip(), Any::new()));
    assert_eq!(s, "!");

    let c = unwrap!(stream.try_peek());
    assert_eq!(c, '\r');

    let nl = unwrap!(stream.try_skip_eol());
    assert!(matches!(nl, Some(true | false)));

    assert!(stream.try_peek().is_err());
    assert!(stream.try_get().is_err());
    assert!(stream.try_get_non_skipped(Skip::new().skip()).is_err());
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
