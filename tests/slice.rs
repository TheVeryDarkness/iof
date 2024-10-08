use iof::{show, unwrap, WriteInto};

#[test]
fn write_into() {
    let buf = unwrap!([1, 2, 3, 4].as_slice().try_write_into_string());
    assert_eq!(buf, "1 2 3 4");
    let buf = unwrap!([1, 2].as_slice().try_write_into_string());
    assert_eq!(buf, "1 2");
    let buf = unwrap!([1].as_slice().try_write_into_string());
    assert_eq!(buf, "1");
    let buf = unwrap!([0i32; 0].as_slice().try_write_into_string());
    assert_eq!(buf, "");

    let buf = unwrap!(['1', '2', '3', '4'].as_slice().try_write_into_string());
    assert_eq!(buf, "1234");
    let buf = unwrap!(['1', '2'].as_slice().try_write_into_string());
    assert_eq!(buf, "12");
    let buf = unwrap!(['1'].as_slice().try_write_into_string());
    assert_eq!(buf, "1");
    let buf = unwrap!(['1'; 0].as_slice().try_write_into_string());
    assert_eq!(buf, "");
}

#[test]
fn show() {
    show!([1, 2, 3, 4].as_slice());
}

#[test]
fn show_to_string() {
    use std::str::from_utf8;
    let mut buf = Vec::new();

    show!([1, 2, 3, 4].as_slice() => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 2 3 4\n");

    buf.clear();
    show!([[1, 2], [3, 4]].as_slice() => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 2\n3 4\n");

    buf.clear();
    show!([[1, 2].as_slice(), [3, 4].as_slice()] => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 2\n3 4\n");

    buf.clear();
    show!([[1, 2].as_slice(), [3, 4].as_slice()].as_slice() => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 2\n3 4\n");
}
