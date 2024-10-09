use super::read_one_from::ReadOneFromError;
use crate::{
    locale::Locale,
    stream::{error::StreamError, line_buf::LineBuf},
    BufReadExt, ReadError, ReadOneFrom,
};
use std::marker::PhantomData;

/// Iterator for all elements.
pub(super) struct ReadAll<'l, 's, L: Locale, S: ?Sized, T: ReadOneFrom> {
    locale: &'l L,
    stream: &'s mut S,
    phantom: PhantomData<T>,
}

impl<'l, 's, L: Locale, S: ?Sized, T: ReadOneFrom> ReadAll<'l, 's, L, S, T> {
    pub(crate) fn new(stream: &'s mut S, locale: &'l L) -> Self {
        let phantom = PhantomData;
        Self {
            locale,
            stream,
            phantom,
        }
    }
}

impl<'l, 's, L: Locale, S: BufReadExt + ?Sized, T: ReadOneFrom> Iterator
    for ReadAll<'l, 's, L, S, T>
{
    type Item = Result<T, ReadOneFromError<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self
            .stream
            .try_get_string_some(self.locale.whitespace_chars())
        {
            Ok(s) => Some(T::parse(s)),
            Err(StreamError::Eof | StreamError::Eol) => None,
            Err(e) => Some(Err(e.into())),
        }
    }
}

/// Iterator for all elements in a string.
pub(super) struct ReadAllIn<'l, 's, L: Locale, T: ReadOneFrom> {
    locale: &'l L,
    stream: LineBuf<'s>,
    phantom: PhantomData<T>,
}

impl<'l, 's, L: Locale, T: ReadOneFrom> ReadAllIn<'l, 's, L, T> {
    pub(crate) fn new(buffer: &'s str, locale: &'l L) -> Self {
        let stream = LineBuf::new(buffer);
        let phantom = PhantomData;
        Self {
            locale,
            stream,
            phantom,
        }
    }
}

impl<'l, 's, L: Locale, T: ReadOneFrom> Iterator for ReadAllIn<'l, 's, L, T> {
    type Item = Result<T, ReadOneFromError<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match T::try_read_one_from(&mut self.stream, self.locale) {
            Ok(t) => Some(Ok(t)),
            Err(ReadError::EOF | ReadError::EOL) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReadAll;
    use crate::{
        read::locale::{ASCII, WHITE_SPACES},
        stream::line_buf::LineBuf,
        unwrap, BufReadExt, InputStream, ReadInto,
    };
    use std::io::Cursor;

    #[test]
    fn line_buf_strings() {
        let s = "Hello, world!";
        let mut buf = LineBuf::new(s);
        let iter = ReadAll::new(&mut buf, &ASCII);
        let res: Result<Vec<String>, _> = iter.collect();
        let res = unwrap!(res);
        assert_eq!(res, vec!["Hello,", "world!"]);
    }

    #[test]
    fn input_stream_strings() {
        let s = "Hello, world!";
        let mut buf = InputStream::new(Cursor::new(s));
        let iter = ReadAll::new(&mut buf, &ASCII);
        let res: Result<Vec<String>, _> = iter.collect();
        let res = unwrap!(res);
        assert_eq!(res, vec!["Hello,", "world!"]);
    }

    #[test]
    #[should_panic = "expect more characters before EOL"]
    fn line_buf_string() {
        let s = "\n";
        let mut buf = LineBuf::new(s);
        let _: &str = unwrap!(buf.try_get_string_some(&WHITE_SPACES));
    }

    #[test]
    #[should_panic = "expect more characters before EOF"]
    fn input_stream_string() {
        let s = "\n";
        let mut buf = InputStream::new(Cursor::new(s));
        let _: &str = unwrap!(buf.try_get_string_some(&WHITE_SPACES));
    }

    #[test]
    #[should_panic = "expect more characters before EOL"]
    fn line_buf_tuple() {
        let s = "1 2";
        let mut buf = LineBuf::new(s);
        let _: (f64, f64, f64) = unwrap!(buf.try_read());
    }

    #[test]
    #[should_panic = "expect more characters before EOF"]
    fn input_stream_tuple() {
        let s = "1 2";
        let mut buf = InputStream::new(Cursor::new(s));
        let _: (f64, f64, f64) = unwrap!(buf.try_read());
    }
}
