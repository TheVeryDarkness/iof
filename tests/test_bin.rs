use std::{
    io::Write,
    process::{Command, Stdio},
    str,
};

fn test_example(name: &str, input: &str, out: &str) {
    #[cfg(target_os = "windows")]
    let out = out.replace("\r\n", "\n");
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(name)
        .env("RUST_BACKTRACE", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "STATUS: {:?}\nSTDOUT: {:?}\nSTDERR: {:?}",
        output.status.code(),
        str::from_utf8(&output.stdout).unwrap(),
        str::from_utf8(&output.stderr).unwrap(),
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), out);
}

#[test]
#[cfg_attr(miri, ignore)]
fn dot_product() {
    test_example("dot_product", "3\n1 2 3\n4 5 6\n4 10 18", "");
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
        "",
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn macros() {
    test_example(
        "macros",
        include_str!("../examples/macros-input.txt"),
        include_str!("../examples/macros-output.txt"),
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_read() {
    test_example("doc_read", include_str!("../examples/doc_read.txt"), "");
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_read_c_compatible() {
    test_example(
        "doc_read_c_compatible",
        include_str!("../examples/doc_read_c_compatible.txt"),
        "",
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_macro_read() {
    test_example(
        "doc_macro_read",
        include_str!("../examples/doc_macro_read.txt"),
        "",
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_fn_read() {
    test_example(
        "doc_fn_read",
        include_str!("../examples/doc_fn_read.txt"),
        "",
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_get_line() {
    test_example(
        "doc_get_line",
        include_str!("../examples/doc_get_line.txt"),
        "",
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_get_line_some() {
    test_example(
        "doc_get_line_some",
        include_str!("../examples/doc_get_line_some.txt"),
        "",
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_show() {
    test_example("doc_show", "", include_str!("../examples/doc_show.txt"));
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_macro_show() {
    test_example(
        "doc_macro_show",
        "",
        include_str!("../examples/doc_macro_show.txt"),
    );
}
