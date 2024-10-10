use crate::utf8char::FixedUtf8Char;
use error::StreamError;

pub(super) mod error;
pub(super) mod input_stream;
pub(super) mod line_buf;
#[cfg(test)]
mod tests;
pub(super) mod traits;

pub(crate) const SP: FixedUtf8Char =
    unsafe { FixedUtf8Char::from_bytes_unchecked([b' ', 0, 0, 0]) };
pub(crate) const HT: FixedUtf8Char =
    unsafe { FixedUtf8Char::from_bytes_unchecked([b'\t', 0, 0, 0]) };
pub(crate) const COMMA: FixedUtf8Char =
    unsafe { FixedUtf8Char::from_bytes_unchecked([b',', 0, 0, 0]) };
pub(crate) const LF: FixedUtf8Char =
    unsafe { FixedUtf8Char::from_bytes_unchecked([b'\n', 0, 0, 0]) };
pub(crate) const CR: FixedUtf8Char =
    unsafe { FixedUtf8Char::from_bytes_unchecked([b'\r', 0, 0, 0]) };

/// End of line characters.
// const EOL: [FixedUtf8Char; 2] = [LF, CR];
const CHAR_EOL: [char; 2] = ['\n', '\r'];

const fn is_eol(c: FixedUtf8Char) -> bool {
    matches!(c, LF | CR)
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

pub(super) const MSG_EOF: &str = "expect more characters before EOF";
pub(super) const MSG_EOL: &str = "expect more characters before EOL";
