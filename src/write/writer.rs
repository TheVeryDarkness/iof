use super::WriteInto;
use crate::Separators;
use std::io::{self, Write};

/// Write the given value into the buffer.
pub fn write(
    value: impl WriteInto,
    buf: &mut impl Write,
    sep: impl Separators,
    end: &'_ str,
) -> io::Result<()> {
    value.try_write_into_with_sep(buf, sep)?;
    buf.write_all(end.as_bytes())
}
