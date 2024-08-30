use iof::{SepBy, WriteInto};
use std::io::Write;

#[test]
fn hex() {
    let x = [2, 3, 5, 8, 13].sep_by(" ");

    let s = format!("{:x}", x);
    assert_eq!(s, "2 3 5 8 d");
    assert_eq!(s, format!("{:x} {:x} {:x} {:x} {:x}", 2, 3, 5, 8, 13));

    let s = format!("{:#x}", x);
    assert_eq!(s, "0x2 0x3 0x5 0x8 0xd");
    assert_eq!(s, format!("{:#x} {:#x} {:#x} {:#x} {:#x}", 2, 3, 5, 8, 13));

    let s = format!("{:X}", x);
    assert_eq!(s, "2 3 5 8 D");
    assert_eq!(s, format!("{:X} {:X} {:X} {:X} {:X}", 2, 3, 5, 8, 13));

    let s = format!("{:#X}", x);
    assert_eq!(s, "0x2 0x3 0x5 0x8 0xD");
    assert_eq!(s, format!("{:#X} {:#X} {:#X} {:#X} {:#X}", 2, 3, 5, 8, 13));
}

#[test]
#[should_panic = "failed to write whole buffer"]
fn limited_buffer_0() {
    let mut buf = [0u8; 0];
    let mut buf = buf.as_mut();
    write!(buf, "{}", [1, 2, 3].sep_by(" ")).unwrap_or_else(|err| {
        panic!("{err}");
    });
}

#[test]
#[should_panic = "failed to write whole buffer"]
fn limited_buffer_4() {
    let mut buf = [0u8; 4];
    let mut buf = buf.as_mut();
    write!(buf, "{}", [1, 2, 3].sep_by(" ")).unwrap_or_else(|err| {
        panic!("{err}");
    });
}

#[test]
fn test_sep_by() {
    let s = [1, 2, 3].sep_by(", ");
    assert_eq!(s.to_string(), "1, 2, 3");
    assert_eq!(s.try_write_into_string().unwrap(), "1, 2, 3");
    let s = ([0i32; 0]).sep_by(", ");
    assert_eq!(s.to_string(), "");
    assert_eq!(s.try_write_into_string().unwrap(), "");
    let s = [1].sep_by(", ");
    assert_eq!(s.to_string(), "1");
    assert_eq!(s.try_write_into_string().unwrap(), "1");
}
