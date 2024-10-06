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
