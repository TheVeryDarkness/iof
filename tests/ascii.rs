use iof::{fmt::Default, ASCIIChar, ASCIIString, InputStream, ReadFrom as _, ReadInto as _};
use std::{io::Cursor, str::FromStr};

#[test]
fn ascii_char() {
    let reader = Cursor::new("1".as_bytes());
    let mut reader = InputStream::new(reader);

    let c: ASCIIChar = reader.read();
    assert_eq!(c, ASCIIChar::Digit1);
    assert_eq!(char::from(c), '1');
    assert_eq!(c, '1');
    assert_eq!('1', c);
    assert_eq!(c.to_u8(), b'1');
    assert_eq!(c, b'1');
    assert_eq!(b'1', c);
    assert_eq!(c.as_str(), "1");
    assert_eq!(c.to_string(), "1");
    assert_eq!(format!("{:?}", c), "'1'");

    assert!(<ASCIIChar>::try_read_from(&mut reader, &Default).is_err());
}

#[test]
#[should_panic = "invalid length: 2"]
fn ascii_char_failed_length() {
    let reader = Cursor::new("ab".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: ASCIIChar = reader.read();
}

#[test]
#[should_panic = r#"invalid length: 4"#]
fn ascii_char_failed() {
    let reader = Cursor::new("🦀");
    let mut reader = InputStream::new(reader);

    let _: ASCIIChar = reader.read();
}

#[test]
fn ascii_string_0_empty() {
    let reader = Cursor::new("a\n".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: ASCIIString = reader.read();
    assert_eq!(vec, ASCIIString::from_str("a").unwrap());

    assert!(<ASCIIChar>::try_read_from(&mut reader, &Default).is_err());
}

#[test]
fn ascii_string_0() {
    let reader = Cursor::new("b\n".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: ASCIIString = reader.read();
    assert_eq!(vec, ASCIIString::from_str("b").unwrap());

    assert!(<ASCIIChar>::try_read_from(&mut reader, &Default).is_err());
}

#[test]
fn ascii_string_1() {
    let reader = Cursor::new("a".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: ASCIIString = reader.read();
    assert_eq!(vec, ASCIIString::from_str("a").unwrap());

    assert!(<ASCIIChar>::try_read_from(&mut reader, &Default).is_err());
}

#[test]
fn ascii_string_2() {
    let reader = Cursor::new("ab".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: ASCIIString = reader.read();
    assert_eq!(vec, ASCIIString::from_str("ab").unwrap());

    assert!(<ASCIIChar>::try_read_from(&mut reader, &Default).is_err());
}

#[test]
#[should_panic = "invalid byte f0 at 0"]
fn ascii_string_failed() {
    let reader = Cursor::new("🦀");
    let mut reader = InputStream::new(reader);

    let _: ASCIIString = reader.read();
}

#[test]
fn ascii_string_eq() {
    assert_eq!(
        ASCIIString::from_str("a").unwrap(),
        ASCIIString::from_str("a").unwrap()
    );
    assert_ne!(
        ASCIIString::from_str("a").unwrap(),
        ASCIIString::from_str("b").unwrap()
    );
    assert_ne!(
        ASCIIString::from_str("a").unwrap(),
        ASCIIString::from_str("ab").unwrap()
    );
    assert_ne!(
        ASCIIString::from_str("ab").unwrap(),
        ASCIIString::from_str("a").unwrap()
    );
    assert_eq!(
        ASCIIString::from_str("ab").unwrap(),
        ASCIIString::from_str("ab").unwrap()
    );

    assert_eq!(&ASCIIString::new(), "");
    assert_eq!(&ASCIIString::from_str("a").unwrap(), "a");
    assert_eq!(&ASCIIString::from_str("b").unwrap(), "b");
    assert_eq!("b", &ASCIIString::from_str("b").unwrap());

    assert_eq!(ASCIIString::new().len(), 0);
    assert_eq!(ASCIIString::from_str("a").unwrap().len(), 1);
    assert_eq!(ASCIIString::from_str("ab").unwrap().len(), 2);
    assert_eq!(ASCIIString::from_str("abc").unwrap().len(), 3);
    assert_eq!(ASCIIString::from_str("abcd").unwrap().len(), 4);

    assert_eq!(
        ASCIIString::from_str("abcd")
            .unwrap()
            .into_iter()
            .collect::<Vec<ASCIIChar>>(),
        ['a', 'b', 'c', 'd']
    );

    let mut s = ASCIIString::from_str("012345678").unwrap();
    for c in s.iter_mut() {
        *c += 1;
    }
    assert_eq!(s, ASCIIString::from_str("123456789").unwrap());
    assert_eq!(&s[..], b"123456789");
    for c in s.iter_mut() {
        *c -= 1;
    }
    assert_eq!(s, ASCIIString::from_str("012345678").unwrap());

    assert_eq!(s[0], '0');
    assert_eq!(s[0], b'0');
    assert_eq!(s[1], '1');

    assert_eq!(&s[0..1], b"0");
    assert_eq!(&s[..], b"012345678");

    for i in 0..s.len() {
        assert_eq!(s[i], (i as u8 + b'0'));
        s[i] += 1;
        assert_eq!(s[i], (i as u8 + b'0' + 1));
        s[i] -= 1;
    }

    assert_eq!(s.to_string(), "012345678");
    assert_eq!(format!("{:?}", s), "\"012345678\"");

    let mut s = ASCIIString::from_str("abcdef").unwrap();
    assert_eq!(s, ASCIIString::from_str("abcdef").unwrap());
    s.as_mut_str().make_ascii_uppercase();
    assert_eq!(s, ASCIIString::from_str("ABCDEF").unwrap());

    let full = ('\x00'..='\x7f').collect::<String>();
    let s = ASCIIString::from_str(full.as_str()).unwrap();
    // assert_eq!(format!("{:?}", s), format!("{:?}", full));
    assert_eq!(format!("{:?}", s), "\"\\0\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\t\\n\\x0b\\x0c\\r\\x0e\\x0f\\x10\\x11\\x12\\x13\\x14\\x15\\x16\\x17\\x18\\x19\\x1a\\x1b\\x1c\\x1d\\x1e\\x1f !\"#$%&\\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\\x7f\"");
}
