use super::read_one_from::ReadOneFromError;
use crate::{
    stream::{error::StreamError, line_buf::LineBuf},
    BufReadExt, ReadIntoError, ReadOneFrom,
};
use std::marker::PhantomData;

/// Iterator for all elements.
pub(super) struct ReadAll<'s, S: ?Sized, T: ReadOneFrom> {
    stream: &'s mut S,
    phantom: PhantomData<T>,
}

impl<'s, S: ?Sized, T: ReadOneFrom> ReadAll<'s, S, T> {
    pub(crate) fn new(stream: &'s mut S) -> Self {
        let phantom = PhantomData;
        Self { stream, phantom }
    }
}

impl<'s, S: BufReadExt + ?Sized, T: ReadOneFrom> Iterator for ReadAll<'s, S, T> {
    type Item = Result<T, ReadOneFromError<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.try_get_string_some() {
            Ok(s) => Some(T::parse(s)),
            Err(StreamError::EOF | StreamError::EOL) => None,
            Err(e) => Some(Err(e.into())),
        }
    }
}

/// Iterator for all elements in a string.
pub(super) struct ReadAllIn<'s, T: ReadOneFrom> {
    stream: LineBuf<'s>,
    phantom: PhantomData<T>,
}

impl<'s, T: ReadOneFrom> ReadAllIn<'s, T> {
    pub(crate) fn new(buffer: &'s str) -> Self {
        let stream = LineBuf::new(buffer);
        let phantom = PhantomData;
        Self { stream, phantom }
    }
}

impl<'s, T: ReadOneFrom> Iterator for ReadAllIn<'s, T> {
    type Item = Result<T, ReadOneFromError<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match T::try_read_one_from(&mut self.stream) {
            Ok(t) => Some(Ok(t)),
            Err(ReadIntoError::EOF | ReadIntoError::EOL) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReadAll;
    use crate::{stream::line_buf::LineBuf, unwrap, InputStream};
    use std::io::Cursor;

    #[test]
    fn line_buf_strings() {
        let s = "Hello, world!";
        let mut buf = LineBuf::new(s);
        let iter = ReadAll::new(&mut buf);
        let res: Result<Vec<String>, _> = iter.collect();
        let res = unwrap!(res);
        assert_eq!(res, vec!["Hello,", "world!"]);
    }

    #[test]
    fn input_stream_strings() {
        let s = "Hello, world!";
        let mut buf = InputStream::new(Cursor::new(s));
        let iter = ReadAll::new(&mut buf);
        let res: Result<Vec<String>, _> = iter.collect();
        let res = unwrap!(res);
        assert_eq!(res, vec!["Hello,", "world!"]);
    }
}
