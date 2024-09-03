use iof::{BufReadExt, InputStream, ReadInto, ReadOneInto, WriteInto, WriteOneInto};
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
fn try_get_string_some() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<&str, _> = buf.try_get_string_some();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
fn try_get_line_some_trimmed() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<&str, _> = buf.try_get_line_some_trimmed();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
fn try_read() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<u32, _> = buf.try_read();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
fn try_read_some_in_line() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<Vec<u32>, _> = buf.try_read_some_in_line();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
fn try_read_any_in_line() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<Vec<u32>, _> = buf.try_read_any_in_line();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
fn try_read_in_line_some_trimmed() {
    let mut buf = InputStream::new(IllBuffer);

    let res: Result<u32, _> = buf.try_read_in_line_some_trimmed();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
fn try_read_in_line_trimmed() {
    let mut buf = InputStream::new(IllBuffer);

    let res: Result<u32, _> = buf.try_read_in_line_trimmed();
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
fn try_write() {
    let mut buf = IllBuffer;

    let res: Result<(), _> = [1, 2, 3].try_write_into(&mut buf);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");

    let res: Result<(), _> = ["", "", ""].try_write_into(&mut buf);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");

    let res: Result<(), _> = ().try_write_into(&mut buf);
    assert!(res.is_ok());

    let res: Result<(), _> = ("\n", 2, ()).try_write_into(&mut buf);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
#[should_panic = "ill buffer"]
fn write_into() {
    [1, 2, 3].write_into(&mut IllBuffer);
}

#[test]
#[should_panic = "ill buffer"]
fn write_one_into() {
    42_usize.write_one_into(&mut IllBuffer);
}

#[test]
fn write() {
    let mut buf = IllBuffer;
    let mut buf = &mut buf;
    let mut buf = &mut buf;
    let buf = &mut buf;

    assert_eq!(
        ' '.try_write_into(buf).unwrap_err().to_string(),
        "ill buffer",
    );
    assert_eq!(
        " ".try_write_into(buf).unwrap_err().to_string(),
        "ill buffer",
    );
    assert_eq!(
        1_usize.try_write_into(buf).unwrap_err().to_string(),
        "ill buffer",
    );
}
