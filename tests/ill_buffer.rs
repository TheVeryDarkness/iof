use iof::{InputStream, ReadInto, ReadIntoSingle};

struct IllBuffer;

impl std::io::Read for IllBuffer {
    fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "ill buffer"))
    }
}
impl std::io::BufRead for IllBuffer {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "ill buffer"))
    }
    fn consume(&mut self, _: usize) {}
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
    let res: Result<u32, _> = buf.try_read_line();
    assert!(res.is_err());
}

#[test]
fn try_read_remained_line_ill() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<u32, _> = buf.try_read_remained_line();
    assert!(res.is_err());
}
