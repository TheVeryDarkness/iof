use error::StreamError;

pub(super) mod error;
pub(super) mod input_stream;
pub(super) mod line_buf;
pub(super) mod traits;

/// ASCII white space characters.
const WHITE: [char; 4] = [' ', '\t', '\n', '\r'];
/// End of line characters.
const EOL: [char; 2] = ['\n', '\r'];

const fn is_eol(c: char) -> bool {
    matches!(c, '\n' | '\r')
}

fn err_eof() -> StreamError {
    StreamError::Eof
}

fn err_eol() -> StreamError {
    StreamError::Eol
}

#[inline]
fn as_slice_from(s: &str, i: usize) -> &str {
    // Assume we get correct encoding.
    debug_assert!(s.is_char_boundary(i));
    unsafe { s.get_unchecked(i..) }
}

#[inline]
fn as_slice_to(s: &str, i: usize) -> &str {
    // Assume we get correct encoding.
    debug_assert!(s.is_char_boundary(i));
    unsafe { s.get_unchecked(..i) }
}

pub(super) const MSG_EOF: &str = "expect more characters before EOF";
pub(super) const MSG_EOL: &str = "expect more characters before EOL";
