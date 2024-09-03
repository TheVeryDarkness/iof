use iof::{get_line, read};

fn main() {
    // Read a single string from input.
    //
    // v
    // 42
    // abc
    let s: String = read!();
    assert_eq!(s, "42");

    // Read a line of string from input.
    // Before reading, the cursor is at the end of the previous line.
    // Therefore, it reads an empty string.
    //
    //   v
    // 42
    // abc
    let s: String = get_line();
    assert_eq!(s, "");

    // Read a line of string from input.
    //
    // v
    // abc
    let s: String = get_line();
    assert_eq!(s, "abc");
}
