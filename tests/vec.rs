use iof::*;
use std::{collections::BTreeSet, io::Cursor};

#[test]
fn parse_vec_by_space() -> anyhow::Result<()> {
    let buf = Vec::from("1 2 3".to_owned().into_bytes());
    let mut reader = Cursor::new(buf.as_slice());
    let vec = reader.parse_n_to_vec_by_space::<u32>(3)?;
    assert_eq!(vec, &[1, 2, 3]);

    assert!(reader.parse_n_to_vec_by_space::<u32>(1).is_err());

    Ok(())
}

#[test]
fn parse_by_space() -> anyhow::Result<()> {
    let buf = Vec::from("3 2 1".to_owned().into_bytes());
    let mut reader = Cursor::new(buf.as_slice());

    let set: BTreeSet<u32> = reader.parse_by_space()?;

    assert_eq!(set, BTreeSet::from([1, 2, 3]));

    Ok(())
}

#[test]
fn read_vec_err() -> anyhow::Result<()> {
    let buf = Vec::from(" 1 2 3".to_owned().into_bytes());
    let mut reader = Cursor::new(buf.as_slice());

    assert!(matches!(
        reader.parse_n_to_vec_by_space::<u32>(1),
        Err(Error::ParseError(_, _))
    ));

    Ok(())
}

fn main() {
    // let lock = stdin().lock();
    // let a: usize = read();
    // let a: usize = stdin().read();
}