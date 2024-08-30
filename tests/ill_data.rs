use iof::WriteInto;
use std::io::{Result, Write};

struct IllData(&'static [u8]);

impl WriteInto for IllData {
    fn try_write_into<S: Write>(&self, s: &mut S) -> Result<()> {
        s.write_all(self.0)?;
        Ok(())
    }
}

#[test]
#[should_panic = "incomplete utf-8 byte sequence from index 0"]
fn try_write_ill_0xcc() {
    let _ = IllData(b"\xcc").write_into_string();
}

#[test]
#[should_panic = "invalid utf-8 sequence of 1 bytes from index 0"]
fn try_write_ill_0xff() {
    let _ = IllData(b"\xff").write_into_string();
}

#[test]
#[should_panic = "incomplete utf-8 byte sequence from index 0"]
fn try_write_ill_0xd0() {
    let _ = IllData(b"\xd0").write_into_string();
}
