use iof::*;
use std::{collections::BTreeSet, io::Cursor};

#[test]
fn read_n() {
    let reader = Cursor::new("1 2 3".as_bytes());
    let mut reader = InputStream::new(reader);

    let vec: Vec<u32> = reader.read_n(3);
    assert_eq!(vec, &[1, 2, 3]);
    assert_eq!(vec.sep_by(" ").to_string(), "1 2 3");

    assert!(iof::ReadInto::<u32>::try_read_n(&mut reader, 1).is_err());
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

    let set: BTreeSet<u32> = reader.read_all().collect();

    assert_eq!(set, BTreeSet::from([1, 2, 3]));
    assert_eq!(set.iter().sep_by(" ").to_string(), "1 2 3");

    Ok(())
}
