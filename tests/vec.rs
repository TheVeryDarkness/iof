use iof::*;
use std::{collections::VecDeque, io::BufReader};

#[test]
fn read_vec() -> anyhow::Result<()> {
    let buf = VecDeque::from("1 2 3".to_owned().into_bytes());
    let mut reader = BufReader::new(buf);
    let vec = reader.read_multiple_separated_by_space::<u32>(3)?;
    assert_eq!(vec, &[1, 2, 3]);

    assert!(reader.read_multiple_separated_by_space::<u32>(1).is_err());

    Ok(())
}
