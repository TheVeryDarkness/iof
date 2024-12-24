use super::read_one_from::ReadOneFromError;
use crate::{
    fmt::Format,
    stream::{error::StreamError, line_buf::LineBuf},
    BufReadExt, ReadError, ReadOneFrom,
};
use std::marker::PhantomData;

/// Iterator for all elements.
pub(super) struct ReadAll<'f, 's, F: Format, S: ?Sized, T: ReadOneFrom> {
    format: &'f F,
    stream: &'s mut S,
    phantom: PhantomData<T>,
}

impl<'f, 's, F: Format, S: ?Sized, T: ReadOneFrom> ReadAll<'f, 's, F, S, T> {
    #[inline]
    pub(crate) fn new(stream: &'s mut S, format: &'f F) -> Self {
        let phantom = PhantomData;
        Self {
            format,
            stream,
            phantom,
        }
    }
}

impl<F: Format, S: BufReadExt + ?Sized, T: ReadOneFrom> Iterator for ReadAll<'_, '_, F, S, T> {
    type Item = Result<T, ReadOneFromError<T>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.try_get_string_some(self.format.skipped_chars()) {
            Ok(s) => Some(T::parse(s)),
            Err(StreamError::Eof | StreamError::Eol) => None,
            Err(e) => Some(Err(e.into())),
        }
    }
}

/// Iterator for all elements in a string.
pub(super) struct ReadAllIn<'f, 's, F: Format, T: ReadOneFrom> {
    format: &'f F,
    stream: LineBuf<'s>,
    phantom: PhantomData<T>,
}

impl<'l, 's, L: Format, T: ReadOneFrom> ReadAllIn<'l, 's, L, T> {
    #[inline]
    pub(crate) fn new(buffer: &'s str, format: &'l L) -> Self {
        let stream = LineBuf::new(buffer);
        let phantom = PhantomData;
        Self {
            format,
            stream,
            phantom,
        }
    }
}

impl<L: Format, T: ReadOneFrom> Iterator for ReadAllIn<'_, '_, L, T> {
    type Item = Result<T, ReadOneFromError<T>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match T::try_read_one_from(&mut self.stream, self.format) {
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
        fmt::{Default, Format},
        stream::line_buf::LineBuf,
        unwrap, BufReadExt, InputStream, ReadInto,
    };
    use std::io::Cursor;

    #[test]
    fn line_buf_strings() {
        let s = "Hello, world!";
        let mut buf = LineBuf::new(s);
        let iter = ReadAll::new(&mut buf, &Default);
        let res: Result<Vec<String>, _> = iter.collect();
        let res = unwrap!(res);
        assert_eq!(res, vec!["Hello,", "world!"]);
    }

    #[test]
    fn input_stream_strings() {
        let s = "Hello, world!";
        let mut buf = InputStream::new(Cursor::new(s));
        let iter = ReadAll::new(&mut buf, &Default);
        let res: Result<Vec<String>, _> = iter.collect();
        let res = unwrap!(res);
        assert_eq!(res, vec!["Hello,", "world!"]);
    }

    #[test]
    #[should_panic = "expect more characters before EOL"]
    fn line_buf_string() {
        let s = "\n";
        let mut buf = LineBuf::new(s);
        let _: &str = unwrap!(buf.try_get_string_some(Default.skipped_chars()));
    }

    #[test]
    #[should_panic = "expect more characters before EOF"]
    fn input_stream_string() {
        let s = "\n";
        let mut buf = InputStream::new(Cursor::new(s));
        let _: &str = unwrap!(buf.try_get_string_some(Default.skipped_chars()));
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
