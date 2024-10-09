use iof::{dimension::Dimension, unwrap, Separators, WriteInto};
use std::io::{Result, Write};

struct IllData(&'static [u8]);

impl Dimension for IllData {
    const DIMENSION: usize = 0;
    const SPACE: bool = true;
}
impl WriteInto for IllData {
    fn try_write_into_with_sep<S: Write + ?Sized>(
        &self,
        s: &mut S,
        _sep: impl Separators,
    ) -> Result<()> {
        s.write_all(self.0)?;
        Ok(())
    }
}

#[test]
fn write_unicode() {
    unwrap!(IllData("🦀🦀🦀".as_bytes()).try_write());
}

#[test]
#[should_panic = "incomplete utf-8 byte sequence from index 0"]
fn try_write_to_string_ill_0xcc() {
    let _ = unwrap!(IllData(b"\xcc").try_write_into_string());
}

#[test]
#[should_panic = "invalid utf-8 sequence of 1 bytes from index 0"]
fn try_write_to_string_ill_0xff() {
    let _ = unwrap!(IllData(b"\xff").try_write_into_string());
}

#[test]
#[should_panic = "incomplete utf-8 byte sequence from index 0"]
fn try_write_to_string_ill_0xd0() {
    let _ = unwrap!(IllData(b"\xd0").try_write_into_string());
}
