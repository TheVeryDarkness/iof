use iof::{sep_by, SepBy, WriteInto};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
};

#[test]
fn hex() {
    let x = [2, 3, 5, 8, 13].sep_by(" ");

    let s = format!("{:x}", x);
    assert_eq!(s, "2 3 5 8 d");
    assert_eq!(s, format!("{:x} {:x} {:x} {:x} {:x}", 2, 3, 5, 8, 13));

    let s = format!("{:#x}", x);
    assert_eq!(s, "0x2 0x3 0x5 0x8 0xd");
    assert_eq!(s, format!("{:#x} {:#x} {:#x} {:#x} {:#x}", 2, 3, 5, 8, 13));

    let s = format!("{:X}", x);
    assert_eq!(s, "2 3 5 8 D");
    assert_eq!(s, format!("{:X} {:X} {:X} {:X} {:X}", 2, 3, 5, 8, 13));

    let s = format!("{:#X}", x);
    assert_eq!(s, "0x2 0x3 0x5 0x8 0xD");
    assert_eq!(s, format!("{:#X} {:#X} {:#X} {:#X} {:#X}", 2, 3, 5, 8, 13));
}

#[test]
#[should_panic = "failed to write whole buffer"]
fn limited_buffer_0() {
    let mut buf = [0u8; 0];
    let mut buf = buf.as_mut();
    write!(buf, "{}", [1, 2, 3].sep_by(" ")).unwrap_or_else(|err| {
        panic!("{err}");
    });
}

#[test]
#[should_panic = "failed to write whole buffer"]
fn limited_buffer_4() {
    let mut buf = [0u8; 4];
    let mut buf = buf.as_mut();
    write!(buf, "{}", [1, 2, 3].sep_by(" ")).unwrap_or_else(|err| {
        panic!("{err}");
    });
}

#[test]
fn test_macro_set() {
    let set = BTreeSet::from_iter([3, 2, 1]);

    let x = sep_by!(&set, " :: ");
    assert_eq!(x.to_string(), "1 :: 2 :: 3");

    let x = sep_by!(set.iter(), " :: ");
    assert_eq!(x.to_string(), "1 :: 2 :: 3");
}

#[test]
fn test_macro_map() {
    let map = BTreeMap::from_iter([(3, "x"), (2, "y"), (1, "z")]);

    let x = sep_by!(map.iter().map(|(k, v)| format!("{}->{}", k, v)), ", ");
    assert_eq!(x.to_string(), "1->z, 2->y, 3->x");
}

#[test]
fn test_macro_vec() {
    let x = sep_by!([1, 2, 3], " ");
    assert_eq!(x.to_string(), "1 2 3");

    let x = sep_by!([1, 2, 3], ", ");
    assert_eq!(x.to_string(), "1, 2, 3");

    let x = sep_by!(vec![1, 2, 3], " ");
    assert_eq!(x.to_string(), "1 2 3");

    let x = sep_by!(vec![1, 2, 3], ", ");
    assert_eq!(x.to_string(), "1, 2, 3");
}

#[test]
fn test_macro_mat() {
    let x = sep_by!([[1, 2], [3, 4]], "\n", " ");
    assert_eq!(x.to_string(), "1 2\n3 4");
}

#[test]
fn test_macro_tensor() {
    let x = sep_by!([[[1, 2], [3, 4]], [[4, 5], [6, 7]]], "\n\n", "\n", " ");
    assert_eq!(x.to_string(), "1 2\n3 4\n\n4 5\n6 7");

    let x = sep_by!([[[1, 2], [3, 4]], [[4, 5], [6, 7]]], "\n\n", "\n", " ");
    assert_eq!(format!("{x:?}"), "1 2\n3 4\n\n4 5\n6 7");
}

#[test]
fn test_vec() {
    let s = [1, 2, 3].sep_by(", ");
    assert_eq!(s.to_string(), "1, 2, 3");
    assert_eq!(s.try_write_into_string().unwrap(), "1, 2, 3");
    assert_eq!(format!("{:?}", s), "1, 2, 3");

    let s = ([0i32; 0]).sep_by(", ");
    assert_eq!(s.to_string(), "");
    assert_eq!(s.try_write_into_string().unwrap(), "");
    assert_eq!(format!("{:?}", s), "");

    let s = [1].sep_by(", ");
    assert_eq!(s.to_string(), "1");
    assert_eq!(s.try_write_into_string().unwrap(), "1");
    assert_eq!(format!("{:?}", s), "1");
}

#[test]
fn test_mat() {
    let s = [[1, 2], [3, 4]].map(|x| x.sep_by(", ")).sep_by("\n");
    assert_eq!(s.to_string(), "1, 2\n3, 4");
}

#[test]
fn test_set() {
    let s = [1, 2, 3].iter().sep_by(", ");
    assert_eq!(s.to_string(), "1, 2, 3");
}
