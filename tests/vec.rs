use iof::*;
use std::{
    collections::{BTreeSet, VecDeque},
    io::BufReader,
};

#[test]
fn parse_vec_by_space() -> anyhow::Result<()> {
    let buf = VecDeque::from("1 2 3".to_owned().into_bytes());
    let mut reader = BufReader::new(buf);
    let vec = reader.parse_vec_by_space::<u32>(3)?;
    assert_eq!(vec, &[1, 2, 3]);

    assert!(reader.parse_vec_by_space::<u32>(1).is_err());

    Ok(())
}

#[test]
fn parse_by_space() -> anyhow::Result<()> {
    let buf = VecDeque::from("3 2 1".to_owned().into_bytes());
    let mut reader = BufReader::new(buf);

    let set: BTreeSet<u32> = reader.parse_by_space();

    assert_eq!(set, BTreeSet::from([1, 2, 3]));

    Ok(())
}

#[test]
fn read_vec_err() -> anyhow::Result<()> {
    let buf = VecDeque::from(" 1 2 3".to_owned().into_bytes());
    let mut reader = BufReader::new(buf);

    assert!(matches!(
        reader.parse_vec_by_space::<u32>(1),
        Err(Error::ParseError(_, _))
    ));

    Ok(())
}
