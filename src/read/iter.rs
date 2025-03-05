use super::read_one_from::ReadOneFromError;
use crate::{
    fmt::Format,
    stream::{error::StreamError, line_buf::LineBuf, traits::BufReadExtWithFormat},
    BufReadExt, ReadError, ReadOneFrom,
};
use std::marker::PhantomData;

/// Iterator for all elements.
pub(super) struct ReadAll<'s, F: Format, S: ?Sized, T: ReadOneFrom> {
    format: F,
    stream: &'s mut S,
    phantom: PhantomData<T>,
}

impl<'s, F: Format, S: ?Sized, T: ReadOneFrom> ReadAll<'s, F, S, T> {
    #[inline]
    pub(crate) fn new(stream: &'s mut S, format: F) -> Self {
        let phantom = PhantomData;
        Self {
            format,
            stream,
            phantom,
        }
    }
}

impl<F: Format, S: BufReadExt + ?Sized, T: ReadOneFrom> Iterator for ReadAll<'_, F, S, T> {
    type Item = Result<T, ReadOneFromError<T>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self
            .stream
            .try_get_string_some(self.format.skip(), T::accept())
        {
            Ok(s) => Some(T::parse(s)),
            Err(StreamError::Eof | StreamError::Eol) => None,
            Err(e) => Some(Err(e.into())),
        }
    }
}

/// Iterator for all elements in a string.
pub(super) struct ReadAllIn<'s, F: Format, T: ReadOneFrom> {
    format: F,
    stream: LineBuf<'s>,
    phantom: PhantomData<T>,
}

impl<'s, F: Format, T: ReadOneFrom> ReadAllIn<'s, F, T> {
    #[inline]
    pub(crate) fn new(buffer: &'s str, format: F) -> Self {
        let stream = LineBuf::new(buffer);
        let phantom = PhantomData;
        Self {
            format,
            stream,
            phantom,
        }
    }
}

impl<F: Format, T: ReadOneFrom> Iterator for ReadAllIn<'_, F, T> {
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
    use super::*;
    use crate::{
        fmt::Default,
        stream::{ext::Any, line_buf::LineBuf},
        unwrap, InputStream, ReadInto,
    };
    use std::io::Cursor;

    #[test]
    fn line_buf_strings() {
        let s = "Hello, world!";
        let mut buf = LineBuf::new(s);
        let iter = ReadAll::new(&mut buf, Default::new());
        let res: Result<Vec<String>, _> = iter.collect();
        let res = unwrap!(res);
        assert_eq!(res, vec!["Hello,", "world!"]);
    }

    #[test]
    fn input_stream_strings() {
        let s = "Hello, world!";
        let mut buf = InputStream::new(Cursor::new(s));
        let skip = Default::new();
        let iter = ReadAll::new(&mut buf, &skip);
        let res: Result<Vec<String>, _> = iter.collect();
        let res = unwrap!(res);
        assert_eq!(res, vec!["Hello,", "world!"]);
    }

    #[test]
    #[should_panic = "expect more characters before EOL"]
    fn line_buf_string() {
        let s = "\n";
        let mut buf = LineBuf::new(s);
        let _: &str = unwrap!(buf.try_get_string_some(Default::<char>::new().skip(), Any::new()));
    }

    #[test]
    #[should_panic = "expect more characters before EOF"]
    fn input_stream_string() {
        let s = "\n";
        let mut buf = InputStream::new(Cursor::new(s));
        let _: &str = unwrap!(buf.try_get_string_some(Default::<char>::new().skip(), Any::new()));
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
