use iof::{
    ext::{Pattern, PatternError, StrExt},
    fmt::skip,
    impl_read_one_from_for_from_str, impl_write_into_for_display, read, show, InputStream,
    ReadOneFrom, ReadOneInto,
};
use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug)]
struct Wrapper<T>(T);

impl<T, U> PartialEq<U> for Wrapper<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &U) -> bool {
        self.0 == *other
    }
}

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: FromStr> FromStr for Wrapper<T> {
    type Err = <T as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Wrapper(s.parse()?))
    }
}

impl_write_into_for_display!(Wrapper<usize>);
impl_write_into_for_display!(Wrapper<&str>);

impl_read_one_from_for_from_str!(Wrapper<usize> => ['0','1','2','3','4','5','6','7','8','9'].as_slice());
impl_read_one_from_for_from_str!(Wrapper<String>);

#[test]
fn test_read() {
    let s: Wrapper<usize> = read!(; src = InputStream::new(b"42".as_slice()));
    assert_eq!(s.0, 42);

    let mut src = InputStream::new(b"42+56".as_slice());
    let s: Wrapper<usize> = read!(; src = src);
    assert_eq!(s.0, 42);
    let s: char = read!(; src = src);
    assert_eq!(s, '+');
    let s: Wrapper<String> = read!(; src = src);
    assert_eq!(s.0, "56");

    let mut src = InputStream::new(b"Hello, World!".as_slice());
    let s: Vec<Wrapper<String>> = src.read_all();
    assert_eq!(s, ["Hello,", "World!"]);

    let mut src = InputStream::new(b"1 2 3 4 5".as_slice());
    let s: Vec<Wrapper<usize>> = src.read_all();
    assert_eq!(s, [1, 2, 3, 4, 5]);

    let s: Wrapper<String> = read!(; src = InputStream::new(b"Hello, World!".as_slice()));
    assert_eq!(s.0, "Hello,");

    let s: Vec<Wrapper<String>> = read!(; src = InputStream::new(b"Hello, World!".as_slice()));
    assert_eq!(s, ["Hello,", "World!"]);

    let s: Wrapper<String> =
        read!(; src = InputStream::new(b"Hello, World!".as_slice()); fmt = skip([]));
    assert_eq!(s.0, "Hello, World!");
}

#[derive(Debug, PartialEq)]
enum E {}

impl Display for E {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "E")
    }
}

impl Error for E {}

#[test]
fn test_accept() {
    let x = Wrapper::<usize>::accept();
    for c in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
        let s = c.to_string();
        assert_eq!(x.forward::<E>(&s), Ok(1));
    }
    for c in ('a'..='z').chain('A'..='Z') {
        let s = c.to_string();
        assert_eq!(
            x.forward::<E>(&s),
            Err(PatternError::UnexpectedChar(s.clone())),
        );
        assert_eq!(
            x.forward::<E>(&s).unwrap_err().to_string(),
            format!("unexpected character at the end of {s:?}"),
        );
    }

    for (a, b, c, d, e) in [
        ("123", "", Some(0), None, Some('1')),
        ("1a2", "a", Some(0), Some(1), Some('1')),
        ("", "", None, None, None),
        ("å1", "å", Some(2), Some(0), Some('å')),
        ("1å", "å", Some(0), Some(1), Some('1')),
        ("🦀", "🦀", None, Some(0), Some('🦀')),
    ] {
        let _ = (b, c, d);
        // assert_eq!(x.trim(a), b);
        // assert_eq!(x.find_first_matching(a), c);
        // assert_eq!(x.find_first_not_matching(a), d);
        assert_eq!(a.first_char(), e);
    }
}

#[test]
fn test_show() {
    show!(Wrapper(42));
    show!(Wrapper("Hello, World!"));
}
