use iof::*;
use std::{collections::BTreeSet, io::Cursor, str::from_utf8};

#[test]
fn read_n() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Vec<u32> = reader.read_n(3);
    assert_eq!(vec, &[1, 2, 3]);
    assert_eq!(vec.sep_by(&" ").to_string(), "1 2 3");

    assert!(<u32>::try_read_n_from(&mut reader, 1).is_err());
}

#[test]
fn read() {
    let reader = Cursor::new("1 2 3\n 4 5 6\n 7   8".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: Vec<u32> = reader.read();
    assert_eq!(a, [1, 2, 3]);

    let b: Vec<u32> = reader.read();
    assert_eq!(b, [4, 5, 6]);

    let c: Vec<u32> = reader.read();
    assert_eq!(c, [7, 8]);

    assert!(<Vec<u32>>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_one_then_read_2() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_one();
    assert_eq!(a, 1);

    let b: Vec<u32> = reader.read();
    assert_eq!(b, [2, 3]);

    assert!(<u32>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_one_then_read_0() {
    let reader = Cursor::new("1\n2 \n3\n".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: u32 = reader.read_one();
    assert_eq!(a, 1);

    let a: u32 = reader.read_one();
    assert_eq!(a, 2);

    let a: u32 = reader.read_one();
    assert_eq!(a, 3);

    assert!(<u32>::try_read_from(&mut reader).is_err());
    assert!(<Vec<u32>>::try_read_from(&mut reader).is_err());
    assert!(<Vec<u32>>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_any_in_line() {
    let reader = Cursor::new("\n1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    unwrap!(reader.try_skip_eol());

    let a: Vec<u32> = reader.read_any_in_line();
    assert_eq!(a, vec![1, 2, 3]);

    assert!(<u32>::try_read_any_in_line_from(&mut reader).is_err());
}

#[test]
#[should_panic = "invalid digit found in string"]
fn read_n_from_str_err() {
    let reader = Cursor::new("1 -2 -3".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: Vec<u32> = reader.read_n(3);
}

#[test]
#[should_panic = "invalid digit found in string"]
fn read_from_str_err() {
    let reader = Cursor::new("1 -2 -3".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: Vec<u32> = reader.read();
}

#[test]
fn read_char_3() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let a: Vec<char> = reader.read();
    assert_eq!(a, vec!['1', '2', '3']);

    assert!(<char>::try_read_from(&mut reader).is_err());
}

#[test]
fn read_all() -> anyhow::Result<()> {
    let reader = Cursor::new("3 2 1".as_bytes());
    let mut reader = InputStream::new(reader);

    let set: BTreeSet<u32> = reader.read_all().into_iter().collect();

    assert_eq!(set, BTreeSet::from([1, 2, 3]));
    assert_eq!(set.iter().sep_by(&" ").to_string(), "1 2 3");

    Ok(())
}

#[test]
#[should_panic = "stream did not contain valid UTF-8"]
fn read_all_encoding_error() {
    let reader = Cursor::new(b"3 2 \xcc1");
    let mut reader = InputStream::new(reader);

    let _: Vec<u32> = reader.read_all();
}

#[test]
#[should_panic = "Error during converting a string \",1\" to a value of `u32`: invalid digit found in string"]
fn read_all_digit_error() {
    let reader = Cursor::new("3 2 ,1".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: Vec<u32> = reader.read_all();
}

#[test]
fn display() {
    let s = Vec::from([1, 2, 3]);
    assert_eq!(s.try_write_into_string().unwrap(), "1 2 3");
    assert_eq!(unwrap!(s.try_write_into_string()), "1 2 3");

    let s = Vec::from([1]);
    assert_eq!(s.try_write_into_string().unwrap(), "1");
    assert_eq!(unwrap!(s.try_write_into_string()), "1");

    let s: Vec<i32> = Vec::from([]);
    assert_eq!(s.try_write_into_string().unwrap(), "");
    assert_eq!(unwrap!(s.try_write_into_string()), "");
}

#[test]
fn show() {
    show!(vec![1, 2, 3]);
    show!(vec![vec![1, 2], vec![3, 4]]);
    show!(vec![] as Vec<usize>);
    show!(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

    let mut buf = Vec::new();
    let s = " :: ".to_string();
    show!(vec![1, 2, 3], sep = [&s], end = "" => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 :: 2 :: 3");

    buf.clear();
    show!(vec![1, 2, 3], sep = &" ++ ", end = "" => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1 ++ 2 ++ 3");

    buf.clear();
    show!(vec![vec![1, 2, 3], vec![4, 5, 6]], sep = [&";", &","], end = "\n\n" => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1,2,3;4,5,6\n\n");

    buf.clear();
    show!(vec![vec![1, 2, 3], vec![4, 5, 6]], sep = [&';', &','], end = "\n\n" => buf);
    assert_eq!(unwrap!(from_utf8(&buf)), "1,2,3;4,5,6\n\n");
}
