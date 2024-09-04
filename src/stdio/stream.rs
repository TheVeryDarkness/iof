use super::stdin;
use crate::{unwrap, BufReadExt};

/// Read a line from standard input. The trailing newline will be consumed and trimmed.
///
/// ```txt
///
/// a b c
/// ```
#[track_caller]
#[inline]
pub fn get_line() -> String {
    unwrap!(stdin().try_get_line().map(ToOwned::to_owned))
}

/// Read a non-empty line from standard input. The trailing newline will be consumed and trimmed.
///
/// ```txt
///
/// a b c
/// ```
#[track_caller]
#[inline]
pub fn get_line_some() -> String {
    unwrap!(stdin().try_get_line_some().map(ToOwned::to_owned))
}
