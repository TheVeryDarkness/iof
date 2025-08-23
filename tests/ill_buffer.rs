use iof::{
    ext::Any,
    fmt::{default, Format},
    show, unwrap, BufReadExtWithFormat as _, InputStream, ReadInto, ReadOneInto, WriteInto,
};
use std::io::{self, Cursor};

struct IllBuffer;

impl io::Read for IllBuffer {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("ill buffer"))
    }
}
impl io::BufRead for IllBuffer {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Err(io::Error::other("ill buffer"))
    }
    fn consume(&mut self, _: usize) {}
}
impl io::Write for IllBuffer {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::other("ill buffer"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::other("ill buffer"))
    }
}

#[test]
fn try_get_string_some() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<&str, _> = buf.try_get_string_some(default::<char>().skip(), Any::new());
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.to_string(), "ill buffer");
}

#[test]
fn try_get_line_some_trimmed() {
    let mut buf = InputStream::new(IllBuffer);
    let res: Result<&str, _> = buf.try_get_line_some_trimmed(default::<char>().skip());
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
fn write_error_error() {
    use std::io::Write;
    let mut buf = InputStream::new(Cursor::new("-1 -2 -3".as_bytes()));
    let vec: Result<[u32; 3], iof::ReadError<_>> = buf.try_read();
    let err = vec.unwrap_err();
    unwrap!(write!(&mut IllBuffer, "{}", err));
}

#[test]
#[should_panic = "ill buffer"]
fn write_array() {
    unwrap!([1, 2, 3].try_write_into(&mut IllBuffer));
}

#[test]
#[should_panic = "ill buffer"]
fn write_tuple() {
    unwrap!((1, 2, 3).try_write_into(&mut IllBuffer));
}

#[test]
#[should_panic = "ill buffer"]
fn show_array() {
    show!([1, 2, 3] => IllBuffer);
}

#[test]
#[should_panic = "ill buffer"]
fn show_tuple() {
    show!((1, 2, 3), sep = [", "] => IllBuffer);
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
