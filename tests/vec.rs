use iof::*;
use std::{collections::BTreeSet, io::Cursor};

#[test]
fn read_n() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Vec<u32> = reader.read_n(3);
    assert_eq!(vec, &[1, 2, 3]);
    assert_eq!(vec.sep_by(" ").to_string(), "1 2 3");

    assert!(ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
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

    assert_eq!(ReadInto::<Vec<u32>>::try_read(&mut reader).unwrap(), [],);
}

#[test]
#[should_panic = "invalid digit found in string"]
fn read_n_from_str_err() {
    let reader = Cursor::new("1 -2 -3".as_bytes());
    let mut reader = InputStream::new(reader);

    let _: Vec<u32> = reader.read_n(3);
}

#[test]
fn read_all() -> anyhow::Result<()> {
    let reader = Cursor::new("3 2 1".as_bytes());
    let mut reader = InputStream::new(reader);

    let set: BTreeSet<u32> = reader.consume_all(|s| s.parse().unwrap()).collect();

    assert_eq!(set, BTreeSet::from([1, 2, 3]));
    assert_eq!(set.iter().sep_by(" ").to_string(), "1 2 3");

    Ok(())
}

#[test]
fn display() {
    let s = Vec::from([1, 2, 3]);
    assert_eq!(s.try_write_into_string().unwrap(), "1 2 3");
    assert_eq!(s.write_into_string(), "1 2 3");

    let s = Vec::from([1]);
    assert_eq!(s.try_write_into_string().unwrap(), "1");
    assert_eq!(s.write_into_string(), "1");

    let s: Vec<i32> = Vec::from([]);
    assert_eq!(s.try_write_into_string().unwrap(), "");
    assert_eq!(s.write_into_string(), "");
}
