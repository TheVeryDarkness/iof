use super::{
    as_slice_from, as_slice_to, err_eol, error::StreamError, is_eol, traits::BufReadExt, WHITE,
};

pub(crate) struct LineBuf<'a> {
    buf: &'a str,
    cursor: usize,
}

impl<'a> LineBuf<'a> {
    pub const fn new(buf: &'a str) -> Self {
        let cursor = 0;
        Self { buf, cursor }
    }
}

impl<'a> BufReadExt for LineBuf<'a> {
    fn try_get(&mut self) -> Result<char, StreamError> {
        let remaining = as_slice_from(self.buf, self.cursor);
        if let Some(c) = remaining.chars().next() {
            self.cursor += c.len_utf8();
            debug_assert!(self.buf.is_char_boundary(self.cursor));
            Ok(c)
        } else {
            Err(err_eol())
        }
    }

    fn try_peek(&mut self) -> Result<char, StreamError> {
        let remaining = as_slice_from(self.buf, self.cursor);
        if let Some(c) = remaining.chars().next() {
            Ok(c)
        } else {
            Err(err_eol())
        }
    }

    fn try_get_if(&mut self, pattern: &[char]) -> Result<Option<char>, StreamError> {
        let remaining = as_slice_from(self.buf, self.cursor);
        if let Some(c) = remaining.chars().next() {
            if pattern.contains(&c) {
                self.cursor += c.len_utf8();
                debug_assert!(self.buf.is_char_boundary(self.cursor));
                Ok(Some(c))
            } else {
                Ok(None)
            }
        } else {
            Err(err_eol())
        }
    }

    fn try_skip_eol(&mut self) -> Result<(), StreamError> {
        let remaining = as_slice_from(self.buf, self.cursor);
        for c in remaining.chars() {
            if is_eol(c) {
                self.cursor += c.len_utf8();
                debug_assert!(self.buf.is_char_boundary(self.cursor));
            } else {
                break;
            }
        }
        Ok(())
    }

    fn try_skip_all(&mut self, skipped: &[char]) -> Result<usize, StreamError> {
        let mut count = 0;
        let s = as_slice_from(self.buf, self.cursor);
        for c in s.chars() {
            if skipped.contains(&c) {
                self.cursor += c.len_utf8();
                debug_assert!(self.buf.is_char_boundary(self.cursor));
                count += c.len_utf8();
            } else {
                break;
            }
        }
        Ok(count)
    }

    fn try_get_until_in_line(&mut self, pattern: &[char]) -> Result<&str, StreamError> {
        let s = as_slice_from(self.buf, self.cursor);
        let i = s.find(pattern).unwrap_or(s.len());
        let frag: &str = as_slice_to(s, i);
        self.cursor += i;
        debug_assert!(self.buf.is_char_boundary(self.cursor));
        Ok(frag)
    }

    fn try_get_string_some(&mut self) -> Result<&str, StreamError> {
        let _: usize = self.try_skip_all_ws()?;
        let s = as_slice_from(self.buf, self.cursor);
        let i = s.find(WHITE).unwrap_or(s.len());
        if i == 0 {
            Err(err_eol())?
        }
        let frag: &str = as_slice_to(s, i);
        self.cursor += i;
        debug_assert!(self.buf.is_char_boundary(self.cursor));
        Ok(frag)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        stream::{error::StreamError, line_buf::LineBuf},
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
        assert!(matches!(stream.try_get().unwrap_err(), StreamError::EOL));
    }

    #[test]
    fn try_get_string() {
        let s = "Hello, world!";
        let mut stream = LineBuf::new(s);
        assert_eq!(stream.try_get_string_some().unwrap(), "Hello,");
        assert_eq!(stream.try_get_string_some().unwrap(), "world!");
        assert!(
            matches!(stream.try_get_string_some().unwrap_err(), StreamError::EOL),
            "{:?}",
            stream.try_get_string_some()
        );
    }
}
