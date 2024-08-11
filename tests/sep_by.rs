use iof::SepBy;
use std::io::Write;

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
    let s = [1, 2, 3].sep_by(", ").to_string();
    assert_eq!(s, "1, 2, 3");
    let s = ([0i32; 0]).sep_by(", ").to_string();
    assert_eq!(s, "");
    let s = [1].sep_by(", ").to_string();
    assert_eq!(s, "1");
}
