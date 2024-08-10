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
