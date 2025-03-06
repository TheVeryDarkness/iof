use iof::{
    dimension::Dimension, fmt::Default, read, BufReadExt, InputStream, Mat, ReadOneFrom,
    ReadOneInto as _,
};
use std::io::Cursor;

#[test]
fn check_separator() {
    assert_eq!(<Vec<char> as Dimension>::get_default_separator(), "");
    assert_eq!(<Mat<char> as Dimension>::get_default_separator(), "\n");
}

#[test]
fn try_read_single_3() {
    let reader = Cursor::new("1. 2e0 3.e0 ".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: f64 = reader.try_read_one().unwrap();
    assert_eq!(a, 1.0);

    let b: f64 = reader.try_read_one().unwrap();
    assert_eq!(b, 2.0);

    let c: f64 = reader.try_read_one().unwrap();
    assert_eq!(c, 3.0);

    assert!(<u32>::try_read_one_from(&mut reader, Default::new()).is_err());
}

#[test]
fn try_read_inf() {
    for s in ["inf", "-inf", "+inf", "infinity", "-infinity", "+infinity"] {
        let reader = Cursor::new(s.as_bytes());
        let mut reader = InputStream::new(reader);

        let a: f64 = read!(; src = reader);
        assert!(a.is_infinite());

        assert!(
            <char>::try_read_one_from(&mut reader, Default::new()).is_err(),
            "{a:?} {s:?}"
        );
    }
}

#[test]
fn try_read_nan() {
    for s in ["nan", "-nan", "+nan"] {
        let reader = Cursor::new(s.as_bytes());
        let mut reader = InputStream::new(reader);

        let a: f64 = read!(; src = reader);
        assert!(a.is_nan());

        assert!(
            <char>::try_read_one_from(&mut reader, Default::new()).is_err(),
            "{a:?} {s:?}"
        );
    }
}

#[test]
fn try_read_error() {
    for s in [
        "n", "a", "np", "na", "nap", "i", "ia", "in", "in_", ",", ".", "e", ".e", "++", "+.",
    ] {
        let reader = Cursor::new(s.as_bytes());
        let mut reader = InputStream::new(reader);

        let a: Result<f64, _> = reader.try_read_one();
        assert!(a.is_err(), "{s} {a:?}");
    }
    for (s, f, r) in [
        ("1.e", 1., "e"),
        ("1.1ee", 1.1, "ee"),
        ("-2-2", -2., "-2"),
        ("2..", 2., "."),
        ("+2..", 2., "."),
        ("+2.2-", 2.2, "-"),
        ("+2.22-", 2.22, "-"),
        ("1.e-", 1., "e-"),
        ("1.e+", 1., "e+"),
        ("1.e-2+", 0.01, "+"),
        ("1.e-+", 1., "e-+"),
        ("1.e+-", 1., "e+-"),
        ("infin", f64::INFINITY, "in"),
        ("infini_", f64::INFINITY, "ini_"),
        ("infini_t", f64::INFINITY, "ini_t"),
        ("infinit_", f64::INFINITY, "init_"),
        ("infinity", f64::INFINITY, ""),
        ("iNfiNitY", f64::INFINITY, ""),
        ("InFiNiTy", f64::INFINITY, ""),
        ("infinity_", f64::INFINITY, "_"),
        ("inf_", f64::INFINITY, "_"),
        ("infi_", f64::INFINITY, "i_"),
        ("infin_", f64::INFINITY, "in_"),
    ] {
        let reader = Cursor::new(s.as_bytes());
        let mut reader = InputStream::new(reader);

        let a: f64 = reader.read_one();
        assert_eq!(a, f);

        assert_eq!(reader.get_cur_line(), r);
    }
    for (s, r) in [
        ("nana", "a"),
        ("nan__", "__"),
        ("nan", ""),
        ("nannan", "nan"),
    ] {
        let reader = Cursor::new(s.as_bytes());
        let mut reader = InputStream::new(reader);

        let a: f64 = reader.read_one();
        assert!(a.is_nan());

        assert_eq!(reader.get_cur_line(), r);
    }
}

#[test]
fn try_read_single() {
    for (s, data) in [
        ("1.", 1.0),
        ("12", 12.0),
        ("3.14", 3.14),
        ("3.1415926", 3.1415926),
        ("2e0", 2.0),
        ("3.e0", 3.0),
        ("3.e10", 3.0e10),
        ("3.e-10", 3.0e-10),
        ("3.14e1", 3.14e1),
        ("3.14e-1", 0.314),
        ("4.", 4.0),
    ] {
        let reader = Cursor::new(s.as_bytes());
        let mut reader = InputStream::new(reader);

        let a: f64 = read!(; src = reader);
        assert_eq!(a, data);

        assert!(
            <char>::try_read_one_from(&mut reader, Default::new()).is_err(),
            "{a:?} {s:?}"
        );
    }
}

#[test]
fn try_read_array_4() {
    for (s, data) in [
        ("1. 2e0 3.e0 4. ", [1.0, 2.0, 3.0, 4.0]),
        ("-1. -2e0 -3.e0 -4. ", [-1.0, -2.0, -3.0, -4.0]),
        ("1e-2 2.e4 .8 4. ", [1e-2, 2e4, 0.8, 4.0]),
    ] {
        let reader = Cursor::new(s.as_bytes());
        let mut reader = InputStream::new(reader);

        let a: [f64; 4] = read!(; src = reader);
        assert_eq!(a, data);

        assert!(<u32>::try_read_one_from(&mut reader, Default::new()).is_err());
    }
}
