use iof::{read_array, read_m_n, read_n, try_read_array, try_read_m_n, try_read_n, Mat};
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn dot_product() {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg("dot_product")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all("3\n1 2 3\n4 5 6\n4 10 18".as_bytes())
        .unwrap();
    let status = child.wait().unwrap();
    assert!(status.success());
}

#[test]
fn read_n_0() {
    let s1: Vec<usize> = read_n(0);
    let s2: Vec<char> = try_read_n(0).unwrap();
    assert!(s1.is_empty());
    assert!(s2.is_empty());
}

#[test]
fn read_m_n_0_0() {
    let s1: Mat<usize> = read_m_n(0, 1);
    let s2: Mat<char> = try_read_m_n(1, 0).unwrap();
    assert!(s1.is_empty());
    assert!(s2.is_empty());
}

#[test]
fn read_array_0() {
    let s1: [usize; 0] = *read_array();
    let s2: [char; 0] = *try_read_array().unwrap();
    assert!(s1.is_empty());
    assert!(s2.is_empty());
}
