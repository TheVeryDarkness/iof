use super::{as_slice_from, err_eol, error::StreamError, traits::BufReadExt};

pub(crate) struct LineBuf<'a> {
    buf: &'a str,
    cursor: usize,
}

impl<'a> LineBuf<'a> {
    #[inline]
    pub(crate) const fn new(buf: &'a str) -> Self {
        let cursor = 0;
        Self { buf, cursor }
    }
}

impl BufReadExt<char> for LineBuf<'_> {
    #[inline]
    fn get_cur_line(&self) -> &str {
        let line = as_slice_from(self.buf, self.cursor);
        line
    }
    #[inline]
    unsafe fn skip(&mut self, n: usize) {
        self.cursor += n;
        debug_assert!(self.buf.is_char_boundary(self.cursor));
    }
    #[inline]
    fn read_buf(&mut self) -> Result<bool, StreamError> {
        if self.cursor < self.buf.len() {
            self.cursor = self.buf.len();
            Ok(true)
        } else {
            Ok(false)
        }
    }
    #[inline]
    fn fill_buf(&mut self) -> Result<(), StreamError> {
        if self.read_buf()? {
            Ok(())
        } else {
            Err(err_eol())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ext::PatternError,
        fmt::{Default, Format, Skip},
        stream::{
            error::StreamError, ext::Any, line_buf::LineBuf, traits::BufReadExtWithFormat as _,
        },
        BufReadExt,
    };

    #[test]
    fn try_get() {
        let s = "Hello, world!";
        let mut stream = LineBuf::new(s);
        assert_eq!(stream.try_get().unwrap(), 'H');
        assert_eq!(stream.try_get().unwrap(), 'e');
        assert_eq!(stream.try_get().unwrap(), 'l');
        assert_eq!(stream.try_get().unwrap(), 'l');
        assert_eq!(stream.try_get().unwrap(), 'o');
        assert_eq!(stream.try_get().unwrap(), ',');
        assert_eq!(stream.try_get().unwrap(), ' ');
        assert_eq!(stream.try_get().unwrap(), 'w');
        assert_eq!(stream.try_get().unwrap(), 'o');
        assert_eq!(stream.try_get().unwrap(), 'r');
        assert_eq!(stream.try_get().unwrap(), 'l');
        assert_eq!(stream.try_get().unwrap(), 'd');
        assert_eq!(stream.try_get().unwrap(), '!');
        assert!(matches!(stream.try_get().unwrap_err(), StreamError::Eol));
        assert_eq!(
            stream
                .try_get_string_some(Default::<char>::new().skip(), Any::new())
                .unwrap_err()
                .to_string(),
            StreamError::Eol.to_string(),
        );
    }

    #[test]
    fn try_get_string() {
        let s = "Hello, world!";
        let mut stream = LineBuf::new(s);
        let d: Default<char> = Default::new();
        let a = Any::new();
        assert_eq!(stream.try_get_string_some(d.skip(), a).unwrap(), "Hello,");
        assert_eq!(stream.try_get_string_some(d.skip(), a).unwrap(), "world!");
        assert!(matches!(
            stream.try_get_string_some(d.skip(), a).unwrap_err(),
            PatternError::Extra(StreamError::Eol),
        ));
        assert_eq!(
            stream
                .try_get_string_some(d.skip(), a)
                .unwrap_err()
                .to_string(),
            StreamError::Eol.to_string(),
        );
    }

    #[test]
    fn try_get_until_in_line() {
        let s = "Hello, world!";
        let mut stream = LineBuf::new(s);
        assert_eq!(
            stream
                .try_get_until_in_line(Skip::<char>::from_iter([',']).skip())
                .unwrap(),
            "Hello",
        );
        assert_eq!(
            stream
                .try_get_until_in_line(Skip::<char>::from_iter(['!']).skip())
                .unwrap(),
            ", world",
        );
        assert_eq!(
            stream
                .try_get_until_in_line(Skip::<char>::from_iter(['!']).skip())
                .unwrap(),
            "",
        );
        assert_eq!(
            stream
                .try_get_until_in_line(Skip::<char>::from_iter([' '; 0]).skip())
                .unwrap(),
            "!"
        );
        assert!(stream
            .try_get_until_in_line(Skip::<char>::from_iter([' '; 0]).skip())
            .is_err());
    }
}
