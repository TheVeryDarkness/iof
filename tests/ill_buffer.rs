use iof::{InputStream, ReadInto, ReadOneInto, WriteInto, WriteOneInto};
use std::io;

struct IllBuffer;

impl io::Read for IllBuffer {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "ill buffer"))
    }
}
impl io::BufRead for IllBuffer {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::new(io::ErrorKind::Other, "ill buffer"))
    }
    fn consume(&mut self, _: usize) {}
}
impl io::Write for IllBuffer {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::Other, "ill buffer"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "ill buffer"))
    }
}

#[test]
fn try_read_ill() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<u32, _> = buf.try_read();
    assert!(res.is_err());
}

#[test]
fn try_read_line_ill() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<u32, _> = buf.try_read_in_line_some_trimmed();
    assert!(res.is_err());
}

#[test]
fn try_read_remained_line_ill() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<u32, _> = buf.try_read_in_line_trimmed();
    assert!(res.is_err());
}

#[test]
fn try_write_ill() {
    let mut buf = IllBuffer;
    let res: Result<(), _> = [1, 2, 3].try_write_into(&mut buf);
    assert!(res.is_err());
    let res: Result<(), _> = ["", "", ""].try_write_into(&mut buf);
    assert!(res.is_err());
}

#[test]
#[should_panic = "ill buffer"]
fn write_into_ill() {
    [1, 2, 3].write_into(&mut IllBuffer);
}

#[test]
#[should_panic = "ill buffer"]
fn write_one_into_ill() {
    42_usize.write_one_into(&mut IllBuffer);
}
