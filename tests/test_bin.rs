use std::{
    io::Write,
    process::{Command, Stdio},
};

fn test_example(name: &str, input: &str, out: &str) {
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
    assert!(output.status.success());
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
        "2\n3 3\n1 2 3\n4 5 6\n7 8 9\n10 11 12\n13 14 15\n16 17 18",
        "\
2
3 3
1 2 3 4 5 6 7 8 9
10 11 12 13 14 15 16 17 18
2
3 3
1 2 3 4 5 6 7 8 9
10 11 12 13 14 15 16 17 18
2, 2 2 :: 3 3 :: 1 2 3 4 5 6 7 8 9
10 11 12 13 14 15 16 17 18
",
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn doc_read() {
    test_example("doc_read", include_str!("../examples/doc_read.txt"), "");
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
