use std::{
    io::Write,
    process::{Command, Stdio},
};

fn test_example(name: &str, input: &str) {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(name)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let status = child.wait().unwrap();
    assert!(status.success());
}

#[test]
#[cfg_attr(miri, ignore)]
fn dot_product() {
    test_example("dot_product", "3\n1 2 3\n4 5 6\n4 10 18");
}

/// Reads two lines of strings from input, and checks if they are in reverse order.
///
/// ```python
/// assert read_line() == read_line()[::-1]
/// ```
#[test]
#[cfg_attr(miri, ignore)]
fn string_reverse() {
    // καλημέραHello,你好！🦀
    // 🦀！好你,olleHαρέμηλακ
    test_example(
        "string_reverse",
        "καλημέραHello,你好！🦀\n🦀！好你,olleHαρέμηλακ",
    );
}
