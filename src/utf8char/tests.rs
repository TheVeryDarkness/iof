use crate::utf8char::{FixedUtf8Char, Utf8Char};

use super::{iter_extensible::IterUtf8Char, iter_fixed::IterFixedUtf8Char};

const S: &str = "aHello🦀🦀🦀：【中文字符串】？烫烫烫。";

#[test]
fn iter_fixed_utf8_char() {
    let f = IterFixedUtf8Char::new(S);
    assert_eq!(f, unsafe {
        IterFixedUtf8Char::new_from_bytes_unchecked(S.as_bytes())
    });
    let f: Vec<_> = f.collect();

    let u = IterUtf8Char::new(S);
    assert_eq!(u, unsafe {
        IterUtf8Char::new_from_bytes_unchecked(S.as_bytes())
    });
    let u: Vec<_> = u.collect();

    let c: Vec<_> = S.chars().collect();
    assert_eq!(f, c);
    assert_eq!(u, c);

    let mut s = S;

    for ((c, f), u) in c.into_iter().zip(f.into_iter()).zip(u.into_iter()) {
        let string = c.to_string();
        let bytes = string.as_bytes();
        assert_eq!(f, c);
        assert_eq!(c, f);
        assert_eq!(f.as_bytes(), bytes);
        assert_eq!(AsRef::<[u8]>::as_ref(&f), bytes);
        assert_eq!(f.as_str(), c.to_string());
        assert_eq!(AsRef::<str>::as_ref(&f), c.to_string());

        assert_eq!(u, c);
        assert_eq!(c, u);
        assert_eq!(u, &c);
        assert_eq!(&c, u);
        assert_eq!(u.as_bytes(), bytes);
        assert_eq!(AsRef::<[u8]>::as_ref(&u), bytes);
        assert_eq!(u.as_str(), c.to_string());
        assert_eq!(AsRef::<str>::as_ref(&u), c.to_string());

        assert_eq!(f, u);
        assert_eq!(u, f);

        assert_eq!(&f, u);
        assert_eq!(u, &f);

        assert_eq!(u.len(), c.len_utf8());
        assert_eq!(f.len(), c.len_utf8());

        assert_eq!(u, unsafe { Utf8Char::from_bytes_unchecked(u.as_bytes()) });
        assert_eq!(f, unsafe { Utf8Char::from_bytes_unchecked(f.as_bytes()) });

        let mut bytes_4 = [0u8; 4];
        let _ = c.encode_utf8(&mut bytes_4);
        assert_eq!(u, unsafe { FixedUtf8Char::from_bytes_unchecked(bytes_4) });

        assert_eq!(FixedUtf8Char::from_first_char(s), Some(f));
        assert_eq!(Utf8Char::from_first_char(s), Some(u));

        s = &s[c.len_utf8()..];
    }

    assert_eq!(FixedUtf8Char::from_first_char(s), None);
    assert_eq!(Utf8Char::from_first_char(s), None);
}
