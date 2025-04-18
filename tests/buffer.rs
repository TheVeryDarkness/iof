use iof::{
    ext::{Any, Pattern, State},
    fmt::{Default, Format, Skip},
    unwrap, BufReadExt, BufReadExtWithFormat as _, InputStream,
};
use std::io::Cursor;

// #[test]
// fn try_get_if() {
//     let reader = Cursor::new("1 2 3".as_bytes());
//     let mut reader = InputStream::new(reader);

//     let c = reader.try_get_if(&['1'].map(Into::into)).unwrap();
//     assert_eq!(c, Some('1'));

//     let c = reader.try_get_if(&['2'].map(Into::into)).unwrap();
//     assert_eq!(c, None);

//     let c = reader.try_get_if(&[' '].map(Into::into)).unwrap();
//     assert_eq!(c, Some(' '));

//     let c = reader.try_get_if(&['2'].map(Into::into)).unwrap();
//     assert_eq!(c, Some('2'));

//     let c = reader.try_get_if(&['3'].map(Into::into)).unwrap();
//     assert_eq!(c, None);

//     let c = reader.try_get_if(&[' '].map(Into::into)).unwrap();
//     assert_eq!(c, Some(' '));

//     let c = reader.try_get_if(&['3'].map(Into::into)).unwrap();
//     assert_eq!(c, Some('3'));

//     let c = reader.try_get_if(&['1', '2', '3', ' '].map(Into::into));
//     assert!(c.is_err());
// }

#[test]
fn skip_all() {
    let buf: Vec<u8> = (0..100).flat_map(|_| b"\r\n".to_owned()).collect();
    let reader = Cursor::new(buf);
    let mut reader = InputStream::new(reader);

    let w = Skip::<char>::from_iter([' ', '\t', '\r']);
    let w = w.skip();
    for _ in 0..100 {
        let c = reader.try_peek().unwrap();
        assert_eq!(c, '\r');
        let c = reader.try_skip_all(w).unwrap();
        assert_eq!(c, 1);
        let c = reader.try_get().unwrap();
        assert_eq!(c, '\n');
    }
    let c = unwrap!(reader.try_skip_all(w));
    assert_eq!(c, 0);
    let c = reader.try_peek();
    assert!(c.is_err());
    let c = reader.try_get();
    assert!(c.is_err());
}

#[test]
fn empty() {
    let mut reader = InputStream::new(b"".as_slice());
    assert!(reader.try_peek().is_err());
}

#[test]
fn try_skip_any() {
    let buf: Vec<u8> = (0..100).flat_map(|_| b"\r".to_owned()).collect();
    let reader = Cursor::new(buf);
    let mut reader = InputStream::new(reader);

    for _ in 0..100 {
        reader.try_skip_any().unwrap();
    }

    let c = reader.try_skip_any();
    assert!(c.is_err());
}

// #[test]
// fn skip_ws() {
//     let buf: Vec<u8> = (0..100).flat_map(|_| b"\r\n".to_owned()).collect();
//     let reader = Cursor::new(buf);
//     let mut reader = InputStream::new(reader);

//     for _ in 0..100 {
//         let c = reader.try_skip_ws().unwrap();
//         assert!(c);
//         let c = reader.try_skip_ws().unwrap();
//         assert!(c);
//     }

//     let c = reader.try_skip_ws();
//     assert!(c.is_err());
// }

// #[test]
// fn skip_all_ws() {
//     let buf: Vec<u8> = (0..100).flat_map(|_| b"\r\n".to_owned()).collect();
//     let reader = Cursor::new(buf);
//     let mut reader = InputStream::new(reader);

//     let c = reader.try_skip_all_ws().unwrap();
//     assert_eq!(c, 200);

//     let c = reader.try_get();
//     assert!(c.is_err());
// }

// #[test]
// fn get_non() {
//     let buf: Vec<u8> = (0..100).flat_map(|_| b"\r\n".to_owned()).collect();
//     let reader = Cursor::new(buf);
//     let mut reader = InputStream::new(reader);

//     let w = &Skip::from_iter([' ', '\t', '\r']);
//     for _ in 0..100 {
//         let c = reader.try_get_non(w).unwrap();
//         assert_eq!(c, '\n');
//     }
//     let c = reader.try_get_non(w);
//     assert!(c.is_err());
// }

// #[test]
// fn get_non_ws_error() {
//     let buf: Vec<u8> = (0..100).map(|_| b'\n').collect();
//     let reader = Cursor::new(buf);
//     let mut reader = InputStream::new(reader);

//     let c = reader.try_get_non_ws();

//     assert!(c.is_err());
// }

#[test]
fn empty_lines() {
    let buf: Vec<u8> = (0..100).map(|_| b'\n').collect();

    let reader = Cursor::new(buf);
    let mut reader = InputStream::new(reader);

    for i in 0..100 {
        let vec = reader.try_get_line().unwrap();
        assert_eq!(vec, "", "i = {}", i);
    }

    assert!(reader.try_get_line().is_err());
    assert!(reader.try_get_line_some().is_err());
}

#[test]
fn read_string() {
    let buf: Vec<u8> = (0..100).flat_map(|_| b"\r\n".to_owned()).collect();

    let reader = Cursor::new(buf);
    let mut reader = InputStream::new(reader);

    let d = Default::<char>::new();
    let d = d.skip();
    assert!(reader.try_get_string_some(d, Any::new()).is_err());
}

#[test]
fn growing() {
    let buf: Vec<u8> = (0..100)
        .flat_map(|i| b"1 ".repeat(i).into_iter().chain([b'\n']))
        .collect();

    let reader = Cursor::new(buf);
    let mut reader = InputStream::new(reader);

    for i in 0..100 {
        let vec = reader.try_get_line().unwrap();
        assert_eq!(vec, "1 ".repeat(i), "i = {}", i);
    }

    assert!(reader.try_get_line().is_err());
}

#[test]
fn read_one_then_read_line() {
    let reader = Cursor::new("1\n2 \n3 ".as_bytes());
    let mut reader = InputStream::new(reader);
    #[derive(Default, Clone, Copy)]
    struct F;
    impl Pattern for F {
        type Item = char;
        fn step(&mut self, c: Self::Item) -> bool {
            c.is_ascii_digit()
        }
        fn state(&self) -> State {
            State::Stoppable
        }
    }
    let f = F;
    let d = Default::<char>::new();
    let d = d.skip();

    let a = reader.try_get_string_some(d, f).unwrap();
    assert_eq!(a, "1");

    let b = reader.try_get_line().unwrap();
    assert_eq!(b, "");

    let a = reader.try_get_string_some(d, f).unwrap();
    assert_eq!(a, "2");

    let b = reader.try_get_line().unwrap();
    assert_eq!(b, " ");

    let a = reader.try_get_string_some(d, f).unwrap();
    assert_eq!(a, "3");

    let b = reader.try_get_line().unwrap();
    assert_eq!(b, " ");

    assert!(reader.try_get_string_some(d, f).is_err());
    assert!(reader.try_get_line().is_err());
    assert!(reader.try_get_line_some().is_err());
}
